use candid::Nat;

use types::context::Context;
use types::liquidity::{AddLiquidityResponse, WithdrawLiquidityResponse};
use liquidity::liquidity_router::get_liquidity_client;
use errors::internal_error::error::InternalError;
use swap::swap_service;

use crate::pools::pool_data::PoolData;
use crate::pools::pool::Pool;
use crate::pool_stats::pool_stats_service;
use crate::event_records::event_record_service;
use crate::event_records::event_record::Event;
use crate::utils::service_resolver::get_service_resolver;

pub async fn get_pools_data(pools: Vec<Pool>) -> Vec<PoolData> {
    let pool_ids: Vec<String> = pools.iter().map(|pool| pool.id.clone()).collect();
    let pool_stats_actor = pool_stats_service::get_pool_stats_actor().await.unwrap();
    let pool_metrics = pool_stats_actor.get_pool_metrics(
        pool_ids
    ).await;

    let pool_data: Vec<PoolData> = pools
        .into_iter()
        .zip(pool_metrics.into_iter())
        .map(|(pool, pool_metric)|
            PoolData {
                pool: pool.clone(),
                apy: pool_metric.1.apy.tokens_apy,
            }
        )
        .collect();

    pool_data
}

pub async fn add_liquidity_to_pool(
    context: Context,
    amount: Nat,
    pool: Pool
) -> Result<AddLiquidityResponse, InternalError> {
    let user = context.user.clone().unwrap();

    // Event: Add liquidity to pool started
    event_record_service::create_event_record(
        Event::add_liquidity_to_pool_started(pool.id.clone(), Some(amount.clone()), None),
        context.correlation_id.clone(),
        Some(user),
    );

    let service_resolver = get_service_resolver();

    let liquidity_client = get_liquidity_client(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        pool.token0,
        pool.token1,
        pool.provider
    ).await;

    let add_liquidity_response = liquidity_client.add_liquidity_to_pool(
        amount.clone()
    ).await
        .map_err(|error| {
            // Event: Add liquidity to pool failed
            event_record_service::create_event_record(
                Event::add_liquidity_to_pool_failed(
                    pool.id.clone(),
                    Some(amount.clone()),
                    error.clone(),
                ),
                context.correlation_id.clone(),
                Some(user),
            );
            error
        })?;

    // Event: Add liquidity to pool completed
    event_record_service::create_event_record(
        Event::add_liquidity_to_pool_completed(
            pool.id.clone(),
            Some(add_liquidity_response.token_0_amount.clone()),
            Some(add_liquidity_response.token_1_amount.clone()),
        ),
        context.correlation_id.clone(),
        Some(user),
    );

    Ok(add_liquidity_response)
}

pub async fn withdraw_liquidity_from_pool(
    context: Context,
    total_shares: Nat,
    shares: Nat,
    pool: Pool
) -> Result<WithdrawLiquidityResponse, InternalError> {
    let user = context.user.clone().unwrap();

    // Event: Withdraw liquidity from pool started
    event_record_service::create_event_record(
        Event::withdraw_liquidity_from_pool_started(
            pool.id.clone(),
            total_shares.clone(),
            shares.clone(),
        ),
        context.correlation_id.clone(),
        Some(user),
    );

    let service_resolver = get_service_resolver();

    let liquidity_client = get_liquidity_client(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        pool.token0,
        pool.token1,
        pool.provider
    ).await;

    let withdraw_liquidity_response = 
        liquidity_client.withdraw_liquidity_from_pool(
            total_shares.clone(),
            shares.clone(),
        ).await
            .map_err(|error| {
                // Event: Withdraw liquidity from pool failed
                event_record_service::create_event_record(
                    Event::withdraw_liquidity_from_pool_failed(
                        pool.id.clone(),
                        total_shares.clone(),
                        shares.clone(),
                        error.clone(),
                    ),
                    context.correlation_id.clone(),
                    Some(user),
                );
                error
            })?;

    // Event: Withdraw liquidity from pool completed
    event_record_service::create_event_record(
        Event::withdraw_liquidity_from_pool_completed(
            pool.id.clone(),
            total_shares.clone(),
            shares.clone(),
            withdraw_liquidity_response.token_0_amount.clone(),
            withdraw_liquidity_response.token_1_amount.clone(),
        ),
        context.correlation_id.clone(),
        Some(user),
    );

    Ok(withdraw_liquidity_response)
}

pub async fn withdraw_liquidity_from_pool_and_swap(
    context: Context,
    total_shares: Nat,
    shares: Nat,
    pool: Pool
) -> Result<Nat, InternalError> {
    let user = context.user.clone().unwrap();

    let withdraw_response = withdraw_liquidity_from_pool(
        context.clone(),
        total_shares.clone(),
        shares.clone(),
        pool.clone(),
    ).await?;

    let service_resolver = get_service_resolver();

    // Reserve token1 fee before swap; if not enough for fee, skip swap safely
    let token1_fee = service_resolver.icrc_ledger_client()
        .icrc1_fee(pool.token1)
        .await
        .unwrap_or_else(|_| Nat::from(0u64));

    // Leave a safety reserve (2x fee) to avoid insufficient funds on transfer_from
    let token1_safety_fee = token1_fee.clone() * Nat::from(2u64);

    let token1_for_swap = if withdraw_response.token_1_amount > token1_safety_fee.clone() {
        withdraw_response.token_1_amount.clone() - token1_safety_fee.clone()
    } else {
        Nat::from(0u64)
    };

    let mut amount_0_to_withdraw = withdraw_response.token_0_amount.clone();

    // If token1 for swap is 0, skip swap
    if token1_for_swap <= Nat::from(0u64) {
        return Ok(amount_0_to_withdraw);
    }

    // Event: Swap token started
    event_record_service::create_event_record(
        Event::swap_token_started(
            pool.id.clone(),
            pool.token1,
            pool.token0,
            Some(token1_for_swap.clone()),
        ),
        context.correlation_id.clone(),
        Some(user),
    );

    // Swap withdrawn token_1 to token_0 (base token)
    let swap_response = swap_service::swap_icrc2_optimal(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        pool.token1,
        pool.token0,
        token1_for_swap.clone(),
    ).await
        .map_err(|error| {
            // Event: Swap token failed
            event_record_service::create_event_record(
                Event::swap_token_failed(
                    pool.id.clone(),
                    pool.token1,
                    pool.token0,
                    Some(token1_for_swap.clone()),
                    error.clone()
                ),
                context.correlation_id.clone(),
                Some(user),
            );

            error
        })?;

    // Event: Swap token completed
    event_record_service::create_event_record(
        Event::swap_token_completed(
            pool.id.clone(),
            pool.token1,
            pool.token0,
            Some(token1_for_swap.clone()),
            Some(Nat::from(swap_response.amount_out)),
        ),
        context.correlation_id.clone(),
        Some(user),
    );

    amount_0_to_withdraw = amount_0_to_withdraw + swap_response.amount_out;

    Ok(amount_0_to_withdraw)
}
