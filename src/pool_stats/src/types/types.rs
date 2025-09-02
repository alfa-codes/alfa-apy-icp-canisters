use std::collections::HashMap;
use candid::{CandidType, Deserialize};
use serde::Serialize;

use ::types::liquidity::{AddLiquidityResponse, WithdrawLiquidityResponse};
use errors::response_error::error::ResponseError;

use crate::pools::pool::Pool;
use crate::pool_metrics::pool_metrics::PoolMetrics;
use crate::pool_snapshots::pool_snapshot::{PoolSnapshot, PoolSnapshotResponse};
use crate::event_records::event_record::EventRecord;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WithdrawLiquidityResult(pub Result<WithdrawLiquidityResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AddLiquidityResult(pub Result<AddLiquidityResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AddPoolResult(pub Result<String, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct DeletePoolResult(pub Result<(), ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetPoolsResult(pub Result<Vec<Pool>, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetPoolByIdResult(pub Result<Pool, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetPoolMetricsResult(pub HashMap<String, PoolMetrics>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetEventRecordsResult(pub Result<Vec<EventRecord>, ResponseError>);

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct GetPoolsHistoryRequest {
    pub pool_ids: Option<Vec<String>>,
    pub from_timestamp: Option<u64>,
    pub to_timestamp: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolHistory {
    pub pool_id: String,
    pub snapshots: Vec<PoolSnapshotResponse>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct GetPoolsHistoryResult(pub Result<Vec<PoolHistory>, ResponseError>);
