use candid::Nat;
use std::ops::{Div, Mul};
use noise::NoiseFn;

use ::utils::util::current_timestamp_secs;
use utils::util::nat_to_u128;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::strategy_history as strategy_history_domain,
    canisters::domains::strategy_history::components as strategy_history_domain_components,
};

use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::types::types::{CreateTestSnapshotsRequest, CreateTestSnapshotsResponse, TestLiquidityData};
use crate::vault::vault_service;

// Module code: "03-03-02"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,                                      // Area code: "03"
    strategy_history_domain::DOMAIN_CODE,                          // Domain code: "03"
    strategy_history_domain_components::TEST_SNAPSHOTS_SERVICE     // Component code: "02"
);

/// Creates test snapshots for a strategy with controlled APY
pub async fn create_test_snapshots(
    request: CreateTestSnapshotsRequest,
) -> Result<CreateTestSnapshotsResponse, InternalError> {
    // Check that the strategy exists and is initialized
    let vault_actor = vault_service::get_vault_actor().await?;
    let vault_strategies = vault_actor.get_strategies().await
        .map_err(|e| {
            InternalError::external_service(
                build_error_code(InternalErrorKind::ExternalService, 8), // Error code: "03-03-02 04 08"
                "test_snapshots_service::create_test_snapshots".to_string(),
                format!("Failed to get strategies from vault: {:?}", e),
                None,
            )
        })?;

    let vault_strategy = vault_strategies.iter()
        .find(|s| s.id == request.strategy_id)
        .ok_or_else(|| {
            InternalError::not_found(
                build_error_code(InternalErrorKind::NotFound, 1), // Error code: "03-03-02 01 01"
                "test_snapshots_service::create_test_snapshots".to_string(),
                "Strategy not found".to_string(),
                errors::error_extra! {
                    "strategy_id" => request.strategy_id,
                },
            )
        })?;

    // Check that the parameters are valid
    if request.min_apy >= request.max_apy {
        return Err(InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 3), // Error code: "03-03-02 03 03"
            "test_snapshots_service::create_test_snapshots".to_string(),
            "min_apy must be less than max_apy".to_string(),
            errors::error_extra! {
                "min_apy" => request.min_apy,
                "max_apy" => request.max_apy,
            },
        ));
    }

    if request.snapshot_interval_secs == 0 {
        return Err(InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 4), // Error code: "03-03-02 03 04"
            "test_snapshots_service::create_test_snapshots".to_string(),
            "snapshot_interval_secs must be greater than 0".to_string(),
            errors::error_extra! {
                "snapshot_interval_secs" => request.snapshot_interval_secs,
            },
        ));
    }

    let current_time = current_timestamp_secs();
    if request.from_timestamp >= current_time {
        return Err(InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 5), // Error code: "03-03-02 03 05"
            "test_snapshots_service::create_test_snapshots".to_string(),
            "from_timestamp must be in the past".to_string(),
            errors::error_extra! {
                "from_timestamp" => request.from_timestamp,
                "current_time" => current_time,
            },
        ));
    }

    // Get test liquidity data from StrategyState
    let strategy_state = crate::services::strategy_states_service::get_strategy_state(request.strategy_id)
        .ok_or_else(|| {
            InternalError::not_found(
                build_error_code(InternalErrorKind::NotFound, 2), // Error code: "03-03-02 01 02"
                "test_snapshots_service::create_test_snapshots".to_string(),
                "Strategy state not found".to_string(),
                errors::error_extra! {
                    "strategy_id" => request.strategy_id,
                },
            )
        })?;

    // TODO: for test purposes, we will use current test liquidity data
    let test_liquidity_data = strategy_state.test_liquidity_data.unwrap_or(
        TestLiquidityData {
            tx_id: 0,
            shares: Nat::from(200_000_000u64),
            amount: Nat::from(200_000_000u64),
            position_id: 0,
        }
    );

    // TODO: remove this after testing
    // let test_liquidity_data = strategy_state.test_liquidity_data
    //     .ok_or_else(|| {
    //         InternalError::business_logic(
    //             build_error_code(InternalErrorKind::BusinessLogic, 7), // Error code: "03-03-02 03 07"
    //             "test_snapshots_service::create_test_snapshots".to_string(),
    //             "Strategy is not initialized with test liquidity data".to_string(),
    //             errors::error_extra! {
    //                 "strategy_id" => request.strategy_id,
    //             },
    //         )
    //     })?;

    // Use test data as base
    let test_liquidity_data_amount = test_liquidity_data.amount.clone();
    let test_liquidity_data_shares = test_liquidity_data.shares.clone();
    let position_id = Some(test_liquidity_data.position_id);
    let users_count = 1; // For test strategy, always 1 user
    let current_pool = vault_strategy.current_pool.clone();
    let current_liquidity = vault_strategy.current_liquidity.clone();
    let current_liquidity_updated_at = Some(current_timestamp_secs());

    // Calculate the number of snapshots
    let total_duration = current_time - request.from_timestamp;
    let snapshots_count = (total_duration / request.snapshot_interval_secs) + 1;

    // Create snapshots
    let mut snapshots_created = 0;
    let mut actual_min_apy = f64::MAX;
    let mut actual_max_apy = f64::MIN;

    for i in 0..snapshots_count {
        let timestamp = request.from_timestamp + (i * request.snapshot_interval_secs);
        
        // Generate random APY in the given range
        let random_apy = generate_random_apy_in_range(request.min_apy, request.max_apy, i as u32);
        
        // Calculate the growth factor to achieve the random APY
        let duration_from_start = timestamp - request.from_timestamp;
        let target_growth_factor = calculate_target_growth_factor(
            random_apy,
            duration_from_start as f64,
        );
        
        // Calculate amount and shares to achieve the target APY
        // Convert growth_factor to integer with precision (multiply by 1M, then divide)
        let growth_factor_int = (target_growth_factor * 1_000_000.0) as u64;
        let test_amount = test_liquidity_data_amount.clone().mul(Nat::from(growth_factor_int))
            .div(Nat::from(1_000_000u64));
        
        let test_shares = test_liquidity_data_shares.clone().mul(Nat::from(growth_factor_int))
            .div(Nat::from(1_000_000u64));

        // Calculate APY for the current snapshot based on our test liquidity growth
        let apy = calculate_single_snapshot_apy(
            &test_liquidity_data_amount,
            &test_amount,
            timestamp - request.from_timestamp,
        );

        // Update the actual APY range
        actual_min_apy = actual_min_apy.min(apy);
        actual_max_apy = actual_max_apy.max(apy);

        // Create snapshot
        let snapshot = StrategySnapshot::new(
            format!("test_{}_{}", request.strategy_id, timestamp),
            request.strategy_id,
            timestamp,
            test_amount.clone(),
            test_shares.clone(),
            current_liquidity.clone(),
            position_id,
            users_count,
            current_liquidity_updated_at,
            current_pool.clone(),
            current_test_liquidity_amount(vault_strategy), // Calculate our test liquidity share
            apy,
        );

        // Save snapshot
        match crate::services::strategy_snapshots_service::save_strategy_snapshot(snapshot) {
            Ok(_) => {
                snapshots_created += 1;
            }
            Err(error) => {
                return Err(InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 6), // Error code: "03-03-02 03 06"
                    "test_snapshots_service::create_test_snapshots".to_string(),
                    "Failed to save test snapshot".to_string(),
                    errors::error_extra! {
                        "error" => error.to_string(),
                        "snapshot_index" => i,
                    },
                ));
            }
        }
    }

    Ok(CreateTestSnapshotsResponse {
        strategy_id: request.strategy_id,
        snapshots_created,
        from_timestamp: request.from_timestamp,
        to_timestamp: current_time,
        min_apy: request.min_apy,
        max_apy: request.max_apy,
        actual_apy_range: (actual_min_apy, actual_max_apy),
    })
}

// =============== Private methods ===============

/// Generates a random APY in the given range
/// Uses the snapshot index as seed for determinism
fn generate_random_apy_in_range(min_apy: f64, max_apy: f64, snapshot_index: u32) -> f64 {
    let perlin = noise::Perlin::new(42);
    let x = snapshot_index as f64 * 0.05;
    let noise_val = perlin.get([x]) * 0.5 + 0.5;
    min_apy + noise_val * (max_apy - min_apy)
}

/// Calculate the target growth factor to achieve the desired APY
fn calculate_target_growth_factor(target_apy: f64, duration_seconds: f64) -> f64 {
    let duration_days = duration_seconds / (24.0 * 60.0 * 60.0);
    
    // Formula: growth_factor = (1 + APY/100)^(duration_days/365)
    let growth_factor = (1.0 + target_apy / 100.0).powf(duration_days / 365.0);
    
    growth_factor
}

/// Calculate our test liquidity amount based on our shares in the strategy
fn current_test_liquidity_amount(vault_strategy: &crate::types::external_canister_types::VaultStrategyResponse) -> Option<Nat> {
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

/// Calculate APY for a single snapshot based on initial and current amounts
/// This function replicates the logic from yield_calculator for single value calculation
fn calculate_single_snapshot_apy(initial_amount: &Nat, current_amount: &Nat, duration_seconds: u64) -> f64 {
    if initial_amount == &Nat::from(0u64) || duration_seconds == 0 {
        return 0.0;
    }

    let initial = nat_to_u128(initial_amount) as f64;
    let current = nat_to_u128(current_amount) as f64;
    let duration_days = duration_seconds as f64 / (24.0 * 60.0 * 60.0);

    if initial <= 0.0 || duration_days <= 0.0 {
        return 0.0;
    }

    let growth_factor = current / initial;
    
    if growth_factor >= 1.0 {
        // Growth -> APY
        let apy = growth_factor.powf(365.0 / duration_days) - 1.0;
        apy * 100.0
    } else {
        // Fall -> percentage loss
        (growth_factor - 1.0) * 100.0
    }
}
