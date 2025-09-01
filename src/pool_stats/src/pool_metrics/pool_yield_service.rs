use candid::Nat;
use yield_calculator::{YieldSnapshot, TimePeriod};

use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::pool_metrics::pool_metrics::ApyValue;

const DEFAULT_PERIOD: TimePeriod = TimePeriod::Week1;

impl YieldSnapshot for PoolSnapshot {
    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

pub fn calculate_pool_yield(snapshots: &[PoolSnapshot], now: u64) -> ApyValue {
    // Calculate USD APY for all time period
    let usd_apy = yield_calculator::calculate_snapshot_yield_for_period(
        snapshots,
        DEFAULT_PERIOD,
        now,
        |snapshot: &PoolSnapshot| {
            snapshot.position_data
                .as_ref()
                .map_or(Nat::from(0u64), |position| {
                    position.usd_amount0.clone() + position.usd_amount1.clone()
                })
        }
    );

    // Calculate tokens APY for all time period
    let tokens_apy = if snapshots.len() >= 2 {
        let apy_token0 = yield_calculator::calculate_snapshot_yield_for_period(
            snapshots,
            DEFAULT_PERIOD,
            now,
            |snapshot: &PoolSnapshot| {
                snapshot.position_data.as_ref().unwrap().amount0.clone()
            }
        );
        let apy_token1 = yield_calculator::calculate_snapshot_yield_for_period(
            snapshots,
            DEFAULT_PERIOD,
            now,
            |snapshot: &PoolSnapshot| {
                snapshot.position_data.as_ref().unwrap().amount1.clone()
            }
        );

        match (apy_token0 > 0.0, apy_token1 > 0.0) {
            (true, true) => (apy_token0 + apy_token1) / 2.0,  // average if both tokens are present
            (true, false) => apy_token0,                      // only first token
            (false, true) => apy_token1,                      // only second token
            (false, false) => 0.0,                            // no tokens
        }
    } else {
        0.0
    };

    ApyValue {
        tokens_apy,
        usd_apy,
    }
}
