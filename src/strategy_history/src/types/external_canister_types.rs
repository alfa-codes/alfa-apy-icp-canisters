use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use types::strategies::StrategyId;
use types::CanisterId;
use errors::types::error_codes::ErrorCode;

// Types must exactly mirror the vault.did to ensure on-wire compatibility
#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub enum ResponseErrorKind {
    NotFound,
    Validation,
    BusinessLogic,
    ExternalService,
    AccessDenied,
    Infrastructure,
    Timeout,
    Unknown,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct ResponseError {
    pub code: ErrorCode,
    pub kind: ResponseErrorKind,
    pub message: String,
    pub details: Option<Vec<(String, String)>>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyDepositResult(pub Result<StrategyDepositResponse, ResponseError>);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyWithdrawResult(pub Result<StrategyWithdrawResponse, ResponseError>);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyDepositArgs {
    pub strategy_id: StrategyId,
    pub ledger: ::types::CanisterId,
    pub amount: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyDepositResponse {
    pub amount: Nat,
    pub shares: Nat,
    pub tx_id: u64,
    pub position_id: u64,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyWithdrawArgs {
    pub strategy_id: StrategyId,
    pub ledger: CanisterId,
    pub percentage: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyWithdrawResponse {
    pub amount: Nat,
    pub current_shares: Nat,
}
