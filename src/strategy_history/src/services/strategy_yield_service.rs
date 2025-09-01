use candid::Nat;

use yield_calculator::{YieldSnapshot, TimePeriod};
use types::strategies::StrategyId;

use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::repository::snapshots_repo;

const DEFAULT_PERIOD: TimePeriod = TimePeriod::Week1;

impl YieldSnapshot for StrategySnapshot {
    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

pub fn calculate_strategy_yield(snapshots: &[StrategySnapshot], now: u64) -> f64 {
    yield_calculator::calculate_snapshot_yield_for_period(
        snapshots,
        DEFAULT_PERIOD,
        now,
        |snapshot: &StrategySnapshot| {
            snapshot.test_liquidity_amount.clone().unwrap_or_else(|| Nat::from(0u64))
        },
    )
}

pub fn calculate_strategy_yield_by_id(strategy_id: StrategyId, now: u64) -> f64 {
    let snapshots = snapshots_repo::get_snapshots_by_strategy_id(strategy_id);
    calculate_strategy_yield(&snapshots, now)
}
