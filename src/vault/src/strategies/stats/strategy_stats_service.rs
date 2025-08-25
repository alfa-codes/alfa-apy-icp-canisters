use std::time::Duration;
use ic_cdk_timers::TimerId;
use candid::Nat;
use std::cell::RefCell;

use types::exchange_id::ExchangeId;
use liquidity::liquidity_router;
use swap::swap_service;
use utils::util::current_timestamp_secs;
use utils::constants::CKUSDT_TOKEN_CANISTER_ID;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::vault as vault_domain,
    canisters::domains::vault::components as vault_domain_components,
};

use crate::repository::strategies_repo;
use crate::strategies::strategy::IStrategy;
use crate::utils::service_resolver::get_service_resolver;

// Module code: "03-01-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,     // Area code: "03"
    vault_domain::DOMAIN_CODE,    // Domain code: "01"
    vault_domain_components::CORE // Component code: "01"
);

thread_local! {
    static STRATEGY_STATS_TIMER_ID: RefCell<Option<TimerId>> = RefCell::new(None);
}

fn set_timer_interval(
    interval: Duration,
    func: impl FnMut() + 'static,
) -> TimerId {
    ic_cdk_timers::set_timer_interval(interval, func)
}

pub fn start_strategy_stats_update_timer(interval: u64) {
    let timer_id = set_timer_interval(Duration::from_secs(interval), || {
        ic_cdk::spawn(async {
            update_all_strategy_liquidity().await;
        });
    });

    STRATEGY_STATS_TIMER_ID.with(|cell| {
        cell.replace(Some(timer_id));
    });
}

pub fn stop_strategy_stats_update_timer() {
    STRATEGY_STATS_TIMER_ID.with(|timer_id| {
        if let Some(timer_id) = timer_id.borrow_mut().take() {
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}

pub async fn update_all_strategy_liquidity() {
    let strategies = strategies_repo::get_all_strategies()
        .into_iter()
        .filter(|strategy| strategy.get_position_id().is_some())
        .collect::<Vec<_>>();

    for strategy in strategies {
        let _ = update_strategy_liquidity(strategy).await;
    }
}

pub async fn update_strategy_liquidity(
    mut strategy: Box<dyn IStrategy>
) -> Result<(), InternalError> {
    let liquidity_amount = get_strategy_current_liquidity(strategy.as_ref()).await?;
    
    strategy.set_current_liquidity(Some(liquidity_amount));
    strategy.set_current_liquidity_updated_at(Some(current_timestamp_secs()));

    strategies_repo::save_strategy(strategy);
    
    Ok(())
}

pub fn spawn_update_strategy_liquidity(strategy: Box<dyn IStrategy>) -> () {
    ic_cdk::spawn(async move {
        let _ = update_strategy_liquidity(strategy).await; // TODO: handle error
    });
}

pub async fn get_strategy_current_liquidity(
    strategy: &dyn IStrategy
) -> Result<Nat, InternalError> {
    let strategy_id = strategy.get_id();
    let current_pool = strategy.get_current_pool();

    if current_pool.is_none() {
        return Err(InternalError::business_logic(
                            build_error_code(InternalErrorKind::BusinessLogic, 5), // Error code: "03-01-01 03 05"
            "strategy_stats_service::get_strategy_current_liquidity".to_string(),
            "Strategy has no current pool".to_string(),
            errors::error_extra! {
                "strategy_id" => strategy_id,
            },
        ));
    }

    let pool = current_pool.unwrap();

    let service_resolver = get_service_resolver();

    let liquidity_client = liquidity_router::get_liquidity_client(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        pool.token0,
        pool.token1,
        pool.provider
    ).await;

    let position_id = strategy.get_position_id()
        .ok_or_else(|| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 6), // Error code: "03-01-01 03 06"
                "strategy_stats_service::get_strategy_current_liquidity".to_string(),
                "Strategy has no position id".to_string(),
                errors::error_extra! {
                    "strategy_id" => strategy_id,
                },
            )
        })?;

    let position_response = liquidity_client
        .get_position_by_id(position_id)
        .await?;

    let quote_response = swap_service::quote_swap_icrc2(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        pool.token1,
        pool.token0,
        position_response.token_1_amount,
        ExchangeId::KongSwap
    ).await?;

    let base_token_amount = Nat::from(quote_response.amount_out) + position_response.token_0_amount;

    Ok(base_token_amount)
}

pub async fn get_strategy_current_liquidity_usd(
    strategy: Box<dyn IStrategy>
) -> Result<f64, InternalError> {
    let current_liquidity_base = get_strategy_current_liquidity(strategy.as_ref()).await?;

    let pool = strategy.get_current_pool().ok_or_else(|| {
        InternalError::business_logic(
                            build_error_code(InternalErrorKind::BusinessLogic, 7), // Error code: "03-01-01 03 07"
            "strategy_stats_service::get_strategy_current_liquidity_usd".to_string(),
            "Strategy has no current pool".to_string(),
            errors::error_extra! {
                "strategy_id" => strategy.get_id(),
            },
        )
    })?;

    let service_resolver = get_service_resolver();

    // Quote base token (token0) to ckUSDT to approximate USD value
    let quote_to_usdt = swap_service::quote_swap_icrc2_optimal(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        pool.token0,
        *CKUSDT_TOKEN_CANISTER_ID,
        current_liquidity_base,
    ).await?;

    // Convert raw ckUSDT amount to float USD using token decimals
    let usdt_decimals = service_resolver
        .icrc_ledger_client()
        .icrc1_decimals(*CKUSDT_TOKEN_CANISTER_ID)
        .await?;

    let amount_out_u128 = quote_to_usdt.amount_out;
    let scale_factor = 10f64.powi(usdt_decimals as i32);
    let usd_value = (amount_out_u128 as f64) / scale_factor;

    Ok(usd_value)
}
