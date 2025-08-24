use candid::Nat;
use std::ops::{Div, Mul};

use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::strategy_history as strategy_history_domain,
    canisters::domains::strategy_history::components as strategy_history_domain_components,
};
use validation::validation::Validation;
use ::utils::util::current_timestamp_secs;
use ::types::strategies::StrategyId;

use crate::repository::snapshots_repo;
use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::types::types::{StrategyState, CreateStrategiesSnapshotsResponse};
use crate::types::external_canister_types::VaultStrategyResponse;
use crate::services::strategy_yield_service;

// Module code: "03-03-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,                    // Area code: "03"
    strategy_history_domain::DOMAIN_CODE,        // Domain code: "03"
    strategy_history_domain_components::CORE     // Component code: "01"
);

pub fn save_strategy_snapshot(snapshot: StrategySnapshot) -> Result<(), InternalError> {
    // Validate snapshot
    snapshot.define_validations().validate()
        .map_err(|errors| {
            if let Some(first_error) = errors.first() {
                first_error.clone()
            } else {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 4), // Error code: "03-03-01 03 04"
                    "strategy_snapshots_service::save_snapshot".to_string(),
                    "Validation failed".to_string(),
                    None
                )
            }
        })?;
    
    snapshots_repo::save_snapshot(snapshot);
    Ok(())
}

pub fn get_strategy_snapshots_count(strategy_id: StrategyId) -> u64 {
    snapshots_repo::get_snapshots_count_by_strategy_id(strategy_id)
}

pub fn create_strategies_snapshots(
    strategy_states: &Vec<(StrategyId, StrategyState)>,
    vault_strategies: &Vec<VaultStrategyResponse>,
) -> Result<CreateStrategiesSnapshotsResponse, InternalError> {
    let mut errors = Vec::new();
    let mut success_count = 0;

    for (strategy_id, _) in strategy_states {
        let vault_strategy = vault_strategies.iter()
            .find(|s| s.id == *strategy_id);

        match vault_strategy {
            Some(vault_strategy) => {
                let snapshot = build_strategy_snapshot(vault_strategy)?;

                match save_strategy_snapshot(snapshot.clone()) {
                    Ok(_) => {
                        success_count += 1;
                    }
                    Err(error) => {
                        errors.push(error);
                    }
                }
            }
            None => {
                let error = InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 11), // Error code: "03-03-01 03 11"
                    "strategy_snapshots_service::create_strategies_snapshots".to_string(),
                    format!("Vault strategy not found for strategy ID: {}", strategy_id),
                    None
                );
                errors.push(error);
            }
        }
    }

    Ok(CreateStrategiesSnapshotsResponse {
        success_count,
        errors,
    })
}

// =============== Private methods ===============

fn build_strategy_snapshot(
    vault_strategy: &VaultStrategyResponse,
) -> Result<StrategySnapshot, InternalError> {
    let test_liquidity_amount = current_test_liquidity_amount(vault_strategy);

    let apy = strategy_yield_service::calculate_strategy_yield_by_id(
        vault_strategy.id, 
        current_timestamp_secs()
    );

    Ok(StrategySnapshot::build(
        vault_strategy.id,
        vault_strategy.total_balance.clone(),
        vault_strategy.total_shares.clone(),
        vault_strategy.current_liquidity.clone(),
        vault_strategy.position_id,
        vault_strategy.users_count,
        vault_strategy.current_liquidity_updated_at,
        vault_strategy.current_pool.clone(),
        test_liquidity_amount,
        apy
    ))
}

fn current_test_liquidity_amount(vault_strategy: &VaultStrategyResponse) -> Option<Nat> {
    let canister_principal = ic_cdk::api::id();
    let test_liquidity_shares = vault_strategy
        .user_shares
        .get(&canister_principal);

    test_liquidity_shares.map(|shares| {
        vault_strategy
            .current_liquidity.clone().unwrap_or(Nat::from(0u64))
            .mul(shares.clone())
            .div(vault_strategy.total_shares.clone())
    })
}
