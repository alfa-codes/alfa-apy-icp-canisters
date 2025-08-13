pub mod repository;
pub mod strategy_snapshot;
pub mod services;
pub mod types;
pub mod utils;
pub mod vault;

use candid::export_service;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use ic_cdk_timers::TimerId;

use errors::response_error::error::ResponseError;


use crate::repository::stable_state;
use crate::services::strategy_history_service;
use crate::services::strategy_states_service;
use crate::services::scheduler_service;
use crate::services::strategy_snapshots_service;
use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::types::types::{
    SaveStrategySnapshotResult,
    InitializeStrategyStatesAndCreateSnapshotsResult,
    GetStrategiesHistoryRequest,
    GetStrategiesHistoryResult,
    StrategyState,
    GetAllStrategyStatesResult,
};

const STRATEGY_HISTORY_FETCHING_INTERVAL: u64 = 1800; // 30 minutes

thread_local! {
    static FETCHING_TIMER_ID: RefCell<Option<TimerId>> = RefCell::new(None);
}

// =============== Initialization ===============

#[init]
fn init() {
    scheduler_service::start_fetching_timer(STRATEGY_HISTORY_FETCHING_INTERVAL);
}

#[pre_upgrade]
fn pre_upgrade() {
    stable_state::stable_save();
}

#[post_upgrade]
fn post_upgrade() {
    stable_state::stable_restore();
    scheduler_service::start_fetching_timer(STRATEGY_HISTORY_FETCHING_INTERVAL);
}

// =============== API Methods ===============

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
    strategy_snapshots_service::get_strategy_snapshots_count(strategy_id)
}

#[query]
fn get_all_strategy_states() -> GetAllStrategyStatesResult {
    let result = strategy_states_service::get_all_strategy_states();

    GetAllStrategyStatesResult(result)
}

#[query]
fn get_strategy_state(strategy_id: u16) -> Option<StrategyState> {
    strategy_states_service::get_strategy_state(strategy_id)
}

/// Fetch and save current strategies from vault
#[update]
async fn test_initialize_strategy_states_and_create_snapshots() -> InitializeStrategyStatesAndCreateSnapshotsResult {
    let result =
        strategy_history_service::initialize_strategy_states_and_create_snapshots()
            .await
            .map_err(|e| ResponseError::from_internal_error(e));

    InitializeStrategyStatesAndCreateSnapshotsResult(result)
}

/// Save a strategy snapshot
#[update]
async fn test_save_strategy_snapshot(snapshot: StrategySnapshot) -> SaveStrategySnapshotResult {
    let result =
        strategy_snapshots_service::save_strategy_snapshot(snapshot)
            .map_err(|e| ResponseError::from_internal_error(e));

    SaveStrategySnapshotResult(result)
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
