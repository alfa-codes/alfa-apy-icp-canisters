pub mod repository;
pub mod strategy_snapshot;
pub mod services;
pub mod types;
pub mod utils;
pub mod vault;

use candid::export_service;
use ic_cdk::{call, id, trap};
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use ic_cdk_timers::TimerId;
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use errors::response_error::error::ResponseError;
use ::types::strategies::StrategyId;

use crate::repository::stable_state;
use crate::repository::runtime_config_repo::{self, RuntimeConfig};
use crate::services::strategy_history_service;
use crate::services::strategy_states_service;
use crate::services::scheduler_service;
use crate::services::strategy_snapshots_service;
use crate::services::test_snapshots_service;
use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::types::types::{
    SaveStrategySnapshotResult,
    InitializeStrategyStatesAndCreateSnapshotsResult,
    GetStrategiesHistoryRequest,
    GetStrategiesHistoryResult,
    StrategyState,
    GetAllStrategyStatesResult,
    CreateTestSnapshotsRequest,
    CreateTestSnapshotsResult,
    InitializeStrategyStatesResult,
};

const STRATEGY_HISTORY_FETCHING_INTERVAL: u64 = 3600; // 1 hour

// Macro for operator authorization check
macro_rules! trap_if_not_authenticated {
    () => {
        let caller = ic_cdk::caller();
        let operator = CONFIG.with(|config| config.borrow().operator);

        if operator.is_none() || operator.unwrap() != caller {
            trap("Unauthorized: caller is not a operator");
        }
    }
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct CanisterIdRequest {
    #[serde(rename = "canister_id")]
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
struct Config {
    operator: Option<Principal>,
}

thread_local! {
    static FETCHING_TIMER_ID: RefCell<Option<TimerId>> = RefCell::new(None);
    static CONFIG: RefCell<Config> = RefCell::new(
        Config {
            operator: None
        },
    );
}


// =============== Initialization ===============

#[init]
fn init(runtime_config: RuntimeConfig) {
    runtime_config_repo::set_runtime_config(runtime_config);

    scheduler_service::start_fetching_timer(STRATEGY_HISTORY_FETCHING_INTERVAL);
}

#[pre_upgrade]
fn pre_upgrade() {
    stable_state::stable_save();
    scheduler_service::stop_fetching_timer();
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
fn get_strategy_snapshots_count(strategy_id: StrategyId) -> u64 {
    strategy_snapshots_service::get_strategy_snapshots_count(strategy_id)
}

#[query]
fn get_all_strategy_states() -> GetAllStrategyStatesResult {
    let result = strategy_states_service::get_all_strategy_states();

    GetAllStrategyStatesResult(result)
}

#[query]
fn get_strategy_state(strategy_id: StrategyId) -> Option<StrategyState> {
    strategy_states_service::get_strategy_state(strategy_id)
}

/// Fetch and save current strategies from vault
#[update]
async fn test_initialize_strategy_states_and_create_snapshots(strategy_ids: Option<Vec<StrategyId>>) -> InitializeStrategyStatesAndCreateSnapshotsResult {
    trap_if_not_authenticated!();

    let result =
        strategy_history_service::initialize_strategy_states_and_create_snapshots(strategy_ids)
            .await
            .map_err(|e| ResponseError::from_internal_error(e));

    InitializeStrategyStatesAndCreateSnapshotsResult(result)
}

#[update]
async fn test_initialize_strategy_states(strategy_ids: Option<Vec<StrategyId>>) -> InitializeStrategyStatesResult {
    trap_if_not_authenticated!();

    let result =
        strategy_history_service::initialize_strategy_states(strategy_ids)
            .await
            .map_err(|e| ResponseError::from_internal_error(e));

    InitializeStrategyStatesResult(result)
}

/// Save a strategy snapshot
#[update]
async fn test_save_strategy_snapshot(snapshot: StrategySnapshot) -> SaveStrategySnapshotResult {
    trap_if_not_authenticated!();

    let result =
        strategy_snapshots_service::save_strategy_snapshot(snapshot)
            .map_err(|e| ResponseError::from_internal_error(e));

    SaveStrategySnapshotResult(result)
}

/// Create test snapshots for a strategy with controlled APY
#[update]
async fn test_create_snapshots(request: CreateTestSnapshotsRequest) -> CreateTestSnapshotsResult {
    trap_if_not_authenticated!();

    let result =
        test_snapshots_service::create_test_snapshots(request)
            .await
            .map_err(|e| ResponseError::from_internal_error(e));

    CreateTestSnapshotsResult(result)
}

#[update]
async fn test_remove_zero_liquidity_snapshots() {
    trap_if_not_authenticated!();

    repository::snapshots_repo::remove_zero_liquidity_snapshots();
}

#[update]
fn test_delete_strategy_state(strategy_id: StrategyId) {
    trap_if_not_authenticated!();

    strategy_states_service::delete_strategy_state(strategy_id);
}

#[update]
fn test_delete_all_snapshots() {
    trap_if_not_authenticated!();

    repository::snapshots_repo::delete_all_snapshots();
}

#[update]
fn test_delete_all_snapshots_for_strategy(strategy_id: StrategyId) {
    trap_if_not_authenticated!();

    repository::snapshots_repo::delete_all_snapshots_for_strategy(strategy_id);
}

#[query]
fn get_runtime_config() -> RuntimeConfig {
    runtime_config_repo::get_runtime_config()
}


// Sets the operator principal.
#[update]
async fn set_operator(operator: Principal) {
    let controllers = get_controllers().await;
    if !controllers.contains(&ic_cdk::caller()) {
        trap("Unauthorized: caller is not a controller");
    }
    CONFIG.with(|config| {
        let mut config = config.borrow_mut();
        config.operator = Some(operator);
    });
}

async fn get_controllers() -> Vec<Principal> {
    let res: CallResult<(ic_cdk::api::management_canister::main::CanisterStatusResponse,)> = call(
        Principal::management_canister(),
        "canister_status",
        (CanisterIdRequest { canister_id: id() },),
    )
        .await;
    res
        .expect(
            "Get controllers function exited unexpectedly:\n\
            inter-canister call to management canister for canister_status returned an empty result."
        )
        .0.settings.controllers
}

// =============== Candid Export ===============

export_service!();

#[ic_cdk_macros::query(name = "export_candid")]
fn export_candid() -> String {
    __export_service()
}
