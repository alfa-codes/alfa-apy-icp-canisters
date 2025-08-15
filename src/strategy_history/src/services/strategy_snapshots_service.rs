use candid::Nat;
use std::ops::{Div, Mul};

use errors::internal_error::error::{InternalError, build_error_code};
use validation::validation::Validation;

use crate::repository::snapshots_repo;
use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::types::types::{StrategyState, CreateStrategiesSnapshotsResponse};
use crate::types::external_canister_types::{StrategyId, VaultStrategyResponse};

pub fn save_strategy_snapshot(snapshot: StrategySnapshot) -> Result<(), InternalError> {
    // Validate snapshot
    snapshot.define_validations().validate()
        .map_err(|errors| {
            if let Some(first_error) = errors.first() {
                first_error.clone()
            } else {
                InternalError::business_logic(
                    build_error_code(0000, 0, 0),
                    "strategy_history_service::save_snapshot".to_string(),
                    "Validation failed".to_string(),
                    None
                )
            }
        })?;
    
    snapshots_repo::save_snapshot(snapshot);
    Ok(())
}

pub fn get_strategy_snapshots_count(strategy_id: u16) -> u64 {
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
            .find(|s| s.id == *strategy_id)
            .unwrap();

        let snapshot = build_strategy_snapshot(vault_strategy)?;

        match save_strategy_snapshot(snapshot.clone()) {
            Ok(_) => {
                success_count += 1;
            }
            Err(error) => {
                errors.push(format!(
                    "Failed to save snapshot for strategy {}: {:?}",
                    vault_strategy.id,
                    error
                ));
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
    let canister_principal = ic_cdk::api::id();
    let canister_liquidity_shares = vault_strategy.user_shares
        .get(&canister_principal)
        .unwrap_or(&Nat::from(0u64))
        .clone();

    let current_liquidity = vault_strategy.current_liquidity
        .clone()
        .unwrap_or(Nat::from(0u64));

    // TODO: calculate APY
    let _canister_current_liquidity = current_liquidity
        .mul(canister_liquidity_shares.clone())
        .div(vault_strategy.total_shares.clone());

    // Pseudo-random APY in [5.0, 15.0]
    let apy = 5.0
    + (((ic_cdk::api::time() ^ (vault_strategy.id as u64)) % 10_000) as f64
        / 10_000.0)
        * 10.0;

    Ok(StrategySnapshot::build(
        vault_strategy.id,
        vault_strategy.total_balance.clone(),
        vault_strategy.total_shares.clone(),
        vault_strategy.current_liquidity.clone(),
        vault_strategy.position_id,
        vault_strategy.users_count,
        vault_strategy.current_liquidity_updated_at,
        vault_strategy.current_pool.clone(),
        apy
    ))
}

