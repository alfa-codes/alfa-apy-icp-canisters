use candid::{CandidType, Deserialize, Nat};
use core::fmt::Display;
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

impl Display for ResponseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseErrorKind::NotFound => write!(f, "NotFound"),
            ResponseErrorKind::Validation => write!(f, "Validation"),
            ResponseErrorKind::BusinessLogic => write!(f, "BusinessLogic"),
            ResponseErrorKind::ExternalService => write!(f, "ExternalService"),
            ResponseErrorKind::AccessDenied => write!(f, "AccessDenied"),
            ResponseErrorKind::Infrastructure => write!(f, "Infrastructure"),
            ResponseErrorKind::Timeout => write!(f, "Timeout"),
            ResponseErrorKind::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct ResponseError {
    pub code: ErrorCode,
    pub kind: ResponseErrorKind,
    pub message: String,
    pub details: Option<Vec<(String, String)>>,
}

impl ResponseError {
    pub fn to_text(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.code.to_string(),
            self.kind.to_string(),
            self.message,
            self.details.as_ref().map(|details| {
                details.iter().map(|(key, value)| {
                    format!("{}:{}", key, value)
                }).collect::<Vec<String>>().join(", ")
            }).unwrap_or_default()
        )
    }
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
