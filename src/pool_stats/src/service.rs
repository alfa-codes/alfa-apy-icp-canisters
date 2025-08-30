use std::collections::HashMap;
use candid::Nat;
use ic_cdk::caller;

use types::exchange_id::ExchangeId;
use types::liquidity::{AddLiquidityResponse, WithdrawLiquidityResponse};
use types::context::Context;
use types::CanisterId;
use types::pool::PoolTrait;
use swap::swap_service;
use utils::constants::ICP_TOKEN_CANISTER_ID;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::pool_stats as pool_stats_domain,
    canisters::domains::pool_stats::components as pool_stats_domain_components,
};

use crate::pool_snapshots::pool_snapshot_service;
use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::pools::pool::Pool;
use crate::pool_metrics::pool_metrics::PoolMetrics;
use crate::pool_metrics::pool_metrics_service;
use crate::repository::pools_repo;
use crate::liquidity::liquidity_service;
use crate::repository::event_records_repo;
use crate::event_records::event_record::EventRecord;
use crate::utils::service_resolver;

const ICP_AMOUNT_FOR_DEPOSIT: u64 = 10_000_000; // 0.1 ICP

// Module code: "03-02-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,          // Area code: "03"
    pool_stats_domain::DOMAIN_CODE,    // Domain code: "02"
    pool_stats_domain_components::CORE // Component code: "01"
);

// ========================== Pools management ==========================

pub fn add_pool(
    token0: CanisterId,
    token1: CanisterId,
    provider: ExchangeId
) -> Result<String, InternalError> {
    let pool = Pool::build(token0, token1, provider);
    pools_repo::add_pool_if_not_exists(pool.clone());
    Ok(pool.id)
}

pub fn delete_pool(id: String) -> Result<(), InternalError> {
    pools_repo::get_pool_by_id(id.clone())
        .map(|pool| {
            pool.delete();
            Ok(())
        })
        .unwrap_or(Err(InternalError::not_found(
                            build_error_code(InternalErrorKind::NotFound, 1), // Error code: "03-02-01 01 01"
            "service::delete_pool".to_string(),
            "Pool not found".to_string(),
            errors::error_extra! {
                "id" => id,
            },
        )))
}

pub fn get_pools() -> Result<Vec<Pool>, InternalError> {
    Ok(pools_repo::get_pools())
}

pub fn get_pool_by_id(id: String) -> Result<Pool, InternalError> {
    pools_repo::get_pool_by_id(id.clone())
        .ok_or_else(|| InternalError::not_found(
                            build_error_code(InternalErrorKind::NotFound, 2), // Error code: "03-02-01 01 02"
            "service::get_pool_by_id".to_string(),
            "Pool not found".to_string(),
            errors::error_extra! {
                "id" => id,
            },
        ))
}

// ========================== Pool metrics ==========================

pub fn get_pool_metrics(pool_ids: Vec<String>) -> HashMap<String, PoolMetrics> {
    pool_ids.into_iter()
        .filter_map(|pool_id| {
            pools_repo::get_pool_by_id(pool_id.clone())
                .map(|pool| (pool_id, pool_metrics_service::create_pool_metrics(pool)))
        })
        .collect()
}

pub fn get_pools_snapshots(pool_ids: Vec<String>) -> HashMap<String, Vec<PoolSnapshot>> {
    pool_ids.into_iter()
        .filter_map(|pool_id| {
            pools_repo::get_pool_by_id(pool_id.clone())
                .map(|pool| (pool_id, pools_repo::get_pool_snapshots(pool.id).unwrap_or_default()))
        })
        .collect()
}

// ========================== Liquidity management ==========================

pub async fn deposit_test_liquidity_to_pool(
    context: Context,
    pool_id: String,
) -> Result<AddLiquidityResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());

    if pool.is_none() {
        let error = InternalError::not_found(
            build_error_code(InternalErrorKind::NotFound, 8), // Error code: "03-02-01 01 08"
            "service::deposit_test_liquidity_to_pool".to_string(),
            "Pool not found".to_string(),
            errors::error_extra! {
                "context" => context,
                "pool_id" => pool_id,
            },
        );

        return Err(error);
    }

    let mut pool = pool.unwrap();

    if pool.position_id.is_some() {
        let error = InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 9), // Error code: "03-02-01 03 09"
            "service::deposit_test_liquidity_to_pool".to_string(),
            "Pool already has liquidity".to_string(),
            errors::error_extra! {
                "context" => context,
                "pool_id" => pool_id,
            },
        );

        return Err(error);
    }

    let service_resolver = service_resolver::get_service_resolver();
    let icrc_ledger_client = service_resolver.icrc_ledger_client();

    let token0 = pool.token0.clone();
    let deposit_icp_amount = Nat::from(ICP_AMOUNT_FOR_DEPOSIT);

    // Swap ICP → base_token or return ICP as is
    let deposit_amount = if token0 != *ICP_TOKEN_CANISTER_ID {
        swap_icp_to_base_token(token0.clone(), deposit_icp_amount.clone()).await?
    } else {
        deposit_icp_amount.clone()
    };

    let token0_fee = icrc_ledger_client.icrc1_fee(token0.clone()).await?;

    let available_for_deposit = if deposit_amount > token0_fee {
        deposit_amount - token0_fee
    } else {
        Nat::from(0u64)
    };

    let response = liquidity_service::add_liquidity_to_pool(
        context.clone(),
        pool.clone(),
        available_for_deposit
    ).await?;

    pool.position_id = Some(response.position_id);
    pools_repo::update_pool(pool_id.clone(), pool.clone());

    pool_snapshot_service::create_pool_snapshot(context, &pool).await?;

    Ok(response)
}

pub async fn add_liquidity_to_pool(
    context: Context,
    ledger: CanisterId,
    pool_id: String,
    amount: Nat
) -> Result<AddLiquidityResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());

    if pool.is_none() {
        let error = InternalError::not_found(
            build_error_code(InternalErrorKind::NotFound, 3), // Error code: "03-02-01 01 03"
            "service::add_liquidity_to_pool".to_string(),
            "Pool not found".to_string(),
            errors::error_extra! {
                "context" => context,
                "ledger" => ledger,
                "pool_id" => pool_id,
                "amount" => amount,
            },
        );

        return Err(error);
    }

    let mut pool = pool.unwrap();

    if pool.position_id.is_some() {
        let error = InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 4), // Error code: "03-02-01 03 04"
            "service::add_liquidity_to_pool".to_string(),
            "Pool already has liquidity".to_string(),
            errors::error_extra! {
                "context" => context,
                "ledger" => ledger,
                "pool_id" => pool_id,
                "amount" => amount,
            },
        );

        return Err(error);
    }

    let service_resolver = service_resolver::get_service_resolver();
    let icrc_ledger_client = service_resolver.icrc_ledger_client();

    icrc_ledger_client.icrc2_transfer_from(
        caller(),
        ledger,
        amount.clone()
    ).await?;

    let response = liquidity_service::add_liquidity_to_pool(
        context.clone(),
        pool.clone(),
        amount
    ).await?;

    pool.position_id = Some(response.position_id);
    pools_repo::update_pool(pool_id.clone(), pool.clone());

    pool_snapshot_service::create_pool_snapshot(context, &pool).await?;

    Ok(response)
}

pub async fn withdraw_liquidity_from_pool(
    context: Context,
    pool_id: String
) -> Result<WithdrawLiquidityResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());

    if pool.is_none() {
        let error = InternalError::not_found(
            build_error_code(InternalErrorKind::NotFound, 5), // Error code: "03-02-01 01 05"
            "service::withdraw_liquidity_from_pool".to_string(),
            "Pool not found".to_string(),
            errors::error_extra! {
                "context" => context,
                "pool_id" => pool_id,
            },
        );

        return Err(error);
    }

    let mut pool = pool.unwrap();

    if pool.position_id.is_none() {
        let error = InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 6), // Error code: "03-02-01 03 06"
            "service::withdraw_liquidity_from_pool".to_string(),
            "Pool has no liquidity".to_string(),
            errors::error_extra! {
                "context" => context,
                "pool_id" => pool_id,
            },
        );

        return Err(error);
    }

    let response = liquidity_service::withdraw_liquidity_from_pool(
        context,
        pool.clone()
    ).await?;

    pool.position_id = None;
    pools_repo::update_pool(pool_id.clone(), pool.clone());

    Ok(response)
}

// ========================== Event records ==========================

pub fn get_event_records(offset: u64, limit: u64) -> Result<Vec<EventRecord>, InternalError> {
    let result = event_records_repo::get_event_records(offset as usize, limit as usize);
    Ok(result)
}

// ========================== Private methods ==========================

async fn swap_icp_to_base_token(
    base_token: CanisterId,
    icp_amount: Nat,
) -> Result<Nat, InternalError> {
    if base_token == *ICP_TOKEN_CANISTER_ID {
        return Ok(icp_amount.clone());
    }

    let service_resolver = service_resolver::get_service_resolver();

    let quote_response = swap_service::quote_swap_icrc2_optimal(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        *ICP_TOKEN_CANISTER_ID,
        base_token,
        icp_amount.clone(),
    ).await?;

    let swap_response = swap_service::swap_icrc2(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        *ICP_TOKEN_CANISTER_ID,
        base_token,
        icp_amount.clone(),
        quote_response.provider,
    )
    .await
    .map_err(|e| {
        InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 7), // Error code: "03-03-01 03 07"
            "strategy_history_service::swap_icp_to_base_token".to_string(),
            format!("Swap failed: {:?}", e),
            errors::error_extra! {
                "base_token" => base_token.to_text(),
                "icp_amount" => icp_amount,
                "quote_response" => quote_response,
            },
        )
    })?;

    Ok(Nat::from(swap_response.amount_out))
}