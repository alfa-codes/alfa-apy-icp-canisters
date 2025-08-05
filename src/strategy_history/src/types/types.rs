use candid::{CandidType, Deserialize, Nat, Principal};
use std::collections::HashMap;
use serde::Serialize;

use errors::response_error::error::ResponseError;

use crate::strategy_snapshot::strategy_snapshot::{StrategySnapshot, Pool};

pub type StrategyId = u16;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct SaveStrategySnapshotResult(pub Result<(), ResponseError>);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct FetchAndSaveStrategiesResult(pub Result<FetchAndSaveStrategiesResponse, ResponseError>);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct GetStrategiesHistoryResult(pub Result<Vec<StrategyHistory>, ResponseError>);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct GetStrategiesHistoryRequest {
    pub strategy_ids: Option<Vec<StrategyId>>,
    pub from_timestamp: Option<u64>,
    pub to_timestamp: Option<u64>,
}

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

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct FetchAndSaveStrategiesResponse {
    pub success_count: u64,
    pub errors: Vec<String>,
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
}
