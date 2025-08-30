use ::types::context::Context;
use errors::internal_error::error::{InternalError, InternalErrorKind};

use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::vault as vault_domain,
    canisters::domains::vault::components as vault_domain_components,
};

// Module code: "03-01-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,     // Area code: "03"
    vault_domain::DOMAIN_CODE,    // Domain code: "01"
    vault_domain_components::CORE // Component code: "01"
);

use crate::repository::strategies_repo;
use crate::user::user_service;
use crate::strategies::strategy::IStrategy;
use crate::types::types::*;
use crate::event_records::event_record::EventRecord;
use crate::event_records::event_record_service;

/// Accepts an investment into a specified strategy.
///
/// # Arguments
///
/// * `args` - An `StrategyDepositArgs` struct containing the ledger, amount, and strategy ID.
///
/// # Returns
///
/// A `Result` containing a `StrategyDepositResponse` struct (with the amount, shares, transaction ID, and request ID)
/// or a `InternalError` if the strategy is not found or the deposit fails.
///
/// # Errors
///
/// Returns a `InternalError` if the strategy is not found or if the deposit operation fails.
pub async fn deposit(
    context: Context,
    args: StrategyDepositArgs
) -> Result<StrategyDepositResponse, InternalError> {
    let strategy_id = context.strategy_id.unwrap();

    let mut strategy = get_strategy_by_id(strategy_id)
        .ok_or_else(|| {
            InternalError::not_found(
                build_error_code(InternalErrorKind::NotFound, 1), // Error code: "03-01-01 01 01"
                "service::deposit".to_string(),
                "Strategy not found".to_string(),
                errors::error_extra! {
                    "context" => context,
                    "args" => args,
                },
            )
        })?;
    
    // TODO: Add validation for ledger
    // if args.ledger != strategy.get_base_token() {}

    user_service::accept_deposit(context.clone(), args.amount.clone(), args.ledger).await?;
    strategy.deposit(context.clone(), args.amount.clone()).await
}

/// Withdraws an amount from a specified strategy.
///
/// # Arguments
///
/// * `args` - A `StrategyWithdrawArgs` struct containing the ledger, amount, and strategy ID.
///
/// # Returns
///
/// A `Result` containing a `StrategyWithdrawResponse` struct (with the withdrawn amount and current shares)
/// or a `InternalError` if the strategy is not found or the withdrawal fails.
///
/// # Errors
///
/// Returns a `InternalError` if the strategy is not found or if the withdrawal operation fails.
pub async fn withdraw(
    context: Context,
    args: StrategyWithdrawArgs
) -> Result<StrategyWithdrawResponse, InternalError> {
    let strategy_id = context.strategy_id.unwrap();
    let mut strategy = get_strategy_by_id(strategy_id)
        .ok_or_else(|| {
            InternalError::not_found(
                build_error_code(InternalErrorKind::NotFound, 2), // Error code: "03-01-01 01 02"
                "service::withdraw".to_string(),
                "Strategy not found".to_string(),
                errors::error_extra! {
                    "context" => context,
                    "args" => args,
                },
            )
        })?;

    strategy.withdraw(context.clone(), args.percentage.clone()).await
}

// ========================== Event records ==========================

pub fn get_event_records(
    request: ListItemsPaginationRequest
) -> Result<ListItemsPaginationResponse<EventRecord>, InternalError> {
    let event_records = event_record_service::get_event_records(request.clone());

    Ok(ListItemsPaginationResponse {
        items: event_records.clone(),
        total: event_records.len() as u64,
        page: request.page,
        page_size: request.page_size,
    })
}

/// Retrieves a strategy by its ID.
///
/// # Arguments
///
/// * `id` - The ID of the strategy to retrieve.
///
/// # Returns
///
/// A `Box<dyn IStrategy>` containing the strategy.
fn get_strategy_by_id(id: u16) -> Option<Box<dyn IStrategy>> {
    strategies_repo::get_strategy_by_id(id)
}
