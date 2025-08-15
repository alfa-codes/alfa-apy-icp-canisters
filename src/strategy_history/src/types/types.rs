use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use errors::response_error::error::ResponseError;

use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::types::external_canister_types::StrategyId;

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
    pub initialized_at: Option<u64>,
    pub initialize_attempts: u32,
    pub snapshot_cadence_secs: Option<u64>,
    pub test_liquidity_data: Option<TestLiquidityData>,
    pub last_snapshot_at: Option<u64>,
    pub last_error: Option<String>,
}

impl StrategyState {
    pub fn is_initialized(&self) -> bool {
        self.test_liquidity_data.is_some()
    }
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
pub struct TestLiquidityData {
    pub amount: Nat,
    pub shares: Nat,
    pub tx_id: u64,
    pub position_id: u64,
}
