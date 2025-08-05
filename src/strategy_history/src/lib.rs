pub mod repository;
pub mod strategy_snapshot;
pub mod service;
pub mod types;
pub mod utils;
pub mod vault;

use candid::export_service;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use ic_cdk_timers::TimerId;

use errors::response_error::error::ResponseError;

use crate::repository::stable_state;
use crate::service::strategy_history_service;
use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::types::types::{
    SaveStrategySnapshotResult,
    FetchAndSaveStrategiesResult,
    GetStrategiesHistoryRequest,
    GetStrategiesHistoryResult,
};

const STRATEGY_HISTORY_FETCHING_INTERVAL: u64 = 1800; // 30 minutes

thread_local! {
    static FETCHING_TIMER_ID: RefCell<Option<TimerId>> = RefCell::new(None);
}

// =============== Initialization ===============

#[init]
fn init() {
    strategy_history_service::start_fetching_timer(STRATEGY_HISTORY_FETCHING_INTERVAL);
}

#[pre_upgrade]
fn pre_upgrade() {
    stable_state::stable_save();
}

#[post_upgrade]
fn post_upgrade() {
    stable_state::stable_restore();
    strategy_history_service::start_fetching_timer(STRATEGY_HISTORY_FETCHING_INTERVAL);
}

// =============== API Methods ===============

/// Save a strategy snapshot
#[update]
async fn save_strategy_snapshot(snapshot: StrategySnapshot) -> SaveStrategySnapshotResult {
    let result =
        strategy_history_service::save_strategy_snapshot(snapshot)
            .map_err(|e| ResponseError::from_internal_error(e));

    SaveStrategySnapshotResult(result)
}

#[query]
async fn get_strategies_history(arg: GetStrategiesHistoryRequest) -> GetStrategiesHistoryResult {
    let result =
        strategy_history_service::get_strategies_history(
            arg.strategy_ids,
            arg.from_timestamp,
            arg.to_timestamp
        ).await.map_err(|e| ResponseError::from_internal_error(e));

    GetStrategiesHistoryResult(result)
}

/// Get the count of snapshots for a strategy
#[query]
fn get_strategy_snapshots_count(strategy_id: u16) -> u64 {
    strategy_history_service::get_strategy_snapshots_count(strategy_id)
}

/// Fetch and save current strategies from vault
#[update]
async fn fetch_and_save_strategies() -> FetchAndSaveStrategiesResult {
    let result =
        strategy_history_service::fetch_and_save_strategies()
            .await
            .map_err(|e| ResponseError::from_internal_error(e));

    FetchAndSaveStrategiesResult(result)
}

#[update]
fn test_delete_all_snapshots() {
    repository::snapshots_repo::delete_all_snapshots();
}

// =============== Candid Export ===============

export_service!();

#[ic_cdk_macros::query(name = "export_candid")]
fn export_candid() -> String {
    __export_service()
}
