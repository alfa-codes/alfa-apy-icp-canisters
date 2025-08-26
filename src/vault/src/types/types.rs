use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use types::CanisterId;
use types::strategies::StrategyId;
use types::strategies::Pool;
use errors::response_error::error::ResponseError;

use crate::event_records::event_record::EventRecord;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolData {
    pub pool: Pool,
    pub apy: f64,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyDepositArgs {
    pub strategy_id: StrategyId,
    pub ledger: CanisterId,
    pub amount: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyWithdrawArgs {
    pub strategy_id: StrategyId,
    pub ledger: CanisterId,
    pub percentage: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyDepositResponse {
    pub amount: Nat,
    pub shares: Nat,
    pub tx_id: u64,
    pub position_id: u64,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyWithdrawResponse {
    pub amount: Nat,
    pub current_shares: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyRebalanceResponse {
    pub previous_pool: Pool,
    pub current_pool: Pool,
    pub is_rebalanced: bool,
}

// TODO: rename to UserPositionResponse
#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct UserStrategyResponse {
    pub strategy_id: StrategyId,
    pub strategy_name: String,
    pub strategy_current_pool: Pool,
    pub total_shares: Nat,
    pub user_shares: Nat,
    pub initial_deposit: Nat,
    pub users_count: u32,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Icrc28TrustedOriginsResponse {
    pub trusted_origins: Vec<String>,
}

#[derive(CandidType, Deserialize, Eq, PartialEq, Debug)]
pub struct SupportedStandard {
    pub url: String,
    pub name: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyDepositResult(pub Result<StrategyDepositResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyWithdrawResult(pub Result<StrategyWithdrawResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetEventRecordsResult(pub Result<EventRecordsPaginationResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyRebalanceResult(pub Result<StrategyRebalanceResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EventRecordsPaginationResponse(pub ListItemsPaginationResponse<EventRecord>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ListItemsPaginationRequest {
    pub page: u64,
    pub page_size: u64,
    pub sort_order: SortOrder,
    pub search: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ListItemsPaginationResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
