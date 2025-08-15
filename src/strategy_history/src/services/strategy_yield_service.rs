use yield_calculator::{YieldSnapshot, TimePeriod};

use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::types::external_canister_types::StrategyId;
use crate::repository::snapshots_repo;

impl YieldSnapshot for StrategySnapshot {
    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

pub fn calculate_strategy_yield(snapshots: &[StrategySnapshot], now: u64) -> f64 {
    yield_calculator::calculate_snapshot_yield_for_period(
        snapshots,
        TimePeriod::All,
        now,
        |snapshot: &StrategySnapshot| snapshot.test_liquidity_amount.clone().unwrap(),
    )
}

pub fn calculate_strategy_yield_by_id(strategy_id: StrategyId, now: u64) -> f64 {
    let snapshots = snapshots_repo::get_snapshots_by_strategy_id(strategy_id);
    calculate_strategy_yield(&snapshots, now)
}
