use candid::{CandidType, Deserialize, Nat, Principal};
use std::collections::HashMap;
use serde::Serialize;

use errors::response_error::error::ResponseError;

use crate::strategy_snapshot::strategy_snapshot::{StrategySnapshot, Pool};

pub type StrategyId = u16;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct SaveStrategySnapshotResult(pub Result<(), ResponseError>);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct InitializeStrategyStatesAndCreateSnapshotsResult(
    pub Result<InitializeStrategyStatesAndCreateSnapshotsResponse, ResponseError>
);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct GetStrategiesHistoryResult(pub Result<Vec<StrategyHistory>, ResponseError>);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct GetStrategiesHistoryRequest {
    pub strategy_ids: Option<Vec<StrategyId>>,
    pub from_timestamp: Option<u64>,
    pub to_timestamp: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct GetAllStrategyStatesResult(pub Vec<(StrategyId, StrategyState)>);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyHistory {
    pub strategy_id: StrategyId,
    pub snapshots: Vec<StrategySnapshot>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategySnapshotsResponse {
    pub strategy_id: StrategyId,
    pub snapshots: Vec<StrategySnapshot>,
    pub total_count: u64,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, Default)]
pub struct StrategyState {
    pub is_initialized: bool,
    pub initialized_at: Option<u64>,
    pub last_snapshot_at: Option<u64>,
    pub snapshot_cadence_secs: Option<u64>,
    pub test_liquidity_amount: Option<Nat>,
    pub bootstrap_attempts: u32,
    pub last_error: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct InitializeStrategyStatesAndCreateSnapshotsResponse {
    pub success_count: u64,
    pub errors: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct CreateStrategiesSnapshotsResponse {
    pub success_count: u64,
    pub errors: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct InitializeStrategyStatesResponse {
    pub initialized_strategy_states: Vec<StrategyId>,
    pub skipped_already_initialized_strategy_states: Vec<StrategyId>,
    pub failed_strategy_states: Vec<(StrategyId, String)>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyDepositArgs {
    pub ledger: ::types::CanisterId,
    pub amount: Nat,
    pub strategy_id: StrategyId,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyDepositResponse {
    pub amount: Nat,
    pub shares: Nat,
    pub tx_id: u64,
    pub position_id: u64,
}

// Struct from vault
#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct VaultStrategyResponse {
    pub name: String,
    pub id: StrategyId,
    pub description: String,
    pub pools: Vec<Pool>,
    pub current_pool: Option<Pool>,
    pub total_balance: Nat,
    pub total_shares: Nat,
    pub user_shares: HashMap<Principal, Nat>,
    pub initial_deposit: HashMap<Principal, Nat>,
    pub users_count: u32,
    pub current_liquidity: Option<Nat>,
    pub current_liquidity_updated_at: Option<u64>,
    pub position_id: Option<u64>,
    pub test: bool,
}
