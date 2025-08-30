use candid::Nat;
use std::ops::{Div, Mul};

use ::utils::util::current_timestamp_secs;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::pool_stats as pool_stats_domain,
    canisters::domains::pool_stats::components as pool_stats_domain_components,
};

use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::pool_snapshots::position_data::position_data::PositionData;
use crate::pool_snapshots::pool_data::pool_data::PoolData;
use crate::repository::pools_repo;

// Module code: "03-02-03"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,                                     // Area code: "03"
    pool_stats_domain::DOMAIN_CODE,                               // Domain code: "02"
    pool_stats_domain_components::TEST_SNAPSHOTS_SERVICE          // Component code: "03"
);

/// Creates test snapshots for a pool with controlled APY
/// Creates two snapshots: one week ago and one current
pub fn create_test_snapshots(pool_id: String, tvl: u128, target_apy: f64) -> Result<(), InternalError> {
    // Check that the pool exists
    let pool = pools_repo::get_pool_by_id(pool_id.clone())
        .ok_or_else(|| InternalError::not_found(
            build_error_code(InternalErrorKind::NotFound, 1), // Error code: "03-02-02 01 01"
            "test_snapshots_service::create_test_snapshots".to_string(),
            "Pool not found".to_string(),
            errors::error_extra! {
                "pool_id" => pool_id.clone(),
            },
        ))?;

    // Delete existing snapshots for this pool
    pools_repo::delete_pool_snapshots(pool_id.clone());

    let current_timestamp = current_timestamp_secs();
    let week_ago_timestamp = current_timestamp - (7 * 24 * 60 * 60); // 7 days ago

    // Base amounts for the old snapshot
    let base_amount = 100_000_000u128;

    // Create old snapshot with fixed values
    let old_position_data = PositionData {
        id: 1,
        amount0: Nat::from(base_amount),
        amount1: Nat::from(base_amount),
        usd_amount0: Nat::from(base_amount),
        usd_amount1: Nat::from(base_amount),
    };

    let old_pool_data = PoolData {
        tvl: Nat::from(tvl),
    };

    let old_snapshot = PoolSnapshot::new(
        "1".to_string(),
        pool_id.clone(),
        week_ago_timestamp,
        Some(old_position_data),
        Some(old_pool_data),
    );

    // Calculate required values for the current snapshot to achieve target APY
    let duration_days = 7.0; // 7 days between snapshots
    let target_growth_factor = calculate_target_growth_factor(target_apy, duration_days);
    
    // Convert growth factor to integer with precision (multiply by 1M, then divide)
    let growth_factor_int = (target_growth_factor * 1_000_000.0) as u128;
    let new_amount = base_amount * growth_factor_int / 1_000_000;

    let new_position_data = PositionData {
        id: 2,
        amount0: Nat::from(new_amount),
        amount1: Nat::from(new_amount),
        usd_amount0: Nat::from(new_amount),
        usd_amount1: Nat::from(new_amount),
    };

    let new_pool_data = PoolData {
        tvl: Nat::from(tvl),
    };

    let new_snapshot = PoolSnapshot::new(
        "2".to_string(),
        pool_id.clone(),
        current_timestamp,
        Some(new_position_data),
        Some(new_pool_data),
    );

    // Save both snapshots
    old_snapshot.save();
    new_snapshot.save();

    Ok(())
}

// =============== Private methods ===============

/// Calculate the target growth factor to achieve the desired APY
fn calculate_target_growth_factor(target_apy: f64, duration_days: f64) -> f64 {
    // Formula: growth_factor = (1 + APY/100)^(duration_days/365)
    let growth_factor = (1.0 + target_apy / 100.0).powf(duration_days / 365.0);
    
    growth_factor
}
