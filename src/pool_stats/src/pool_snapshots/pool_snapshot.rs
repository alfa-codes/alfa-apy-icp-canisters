use candid::{CandidType, Deserialize};
use serde::Serialize;

use errors::internal_error::error::InternalError;
use utils::util::current_timestamp_secs;

use crate::repository::pools_repo;
use crate::pool_snapshots::position_data::position_data::PositionData;
use crate::pool_snapshots::pool_data::pool_data::PoolData;
use crate::pool_metrics::pool_metrics::ApyValue;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolSnapshot {
    pub id: String,
    pub pool_id: String,
    pub timestamp: u64,
    pub position_data: Option<PositionData>,
    pub pool_data: Option<PoolData>,
}

impl PoolSnapshot {
    pub fn new(
        id: String,
        pool_id: String, 
        timestamp: u64, 
        position_data: Option<PositionData>,
        pool_data: Option<PoolData>,
    ) -> Self {
        Self {
            id,
            pool_id,
            timestamp,
            position_data,
            pool_data,
        }
    }

    pub fn build(pool_id: String, position_data: Option<PositionData>, pool_data: Option<PoolData>) -> Self {
        let id = (pools_repo::get_pool_snapshots_count(pool_id.clone()) + 1).to_string();
        let timestamp = current_timestamp_secs();

        Self::new(
            id,
            pool_id,
            timestamp,
            position_data,
            pool_data,
        )
    }

    pub fn create(
        pool_id: String,
        position_data: Option<PositionData>,
        pool_data: Option<PoolData>,
    ) -> Result<Self, InternalError> {
        let snapshot = Self::build(pool_id, position_data, pool_data);
        snapshot.save();
        Ok(snapshot)
    }

    pub fn save(&self) {
        pools_repo::save_pool_snapshot(self.clone());
    }
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq)]
pub struct PoolSnapshotResponse {
    pub id: String,
    pub pool_id: String,
    pub timestamp: u64,
    pub position_data: Option<PositionData>,
    pub pool_data: Option<PoolData>,
    pub apy: ApyValue,
}

impl From<PoolSnapshot> for PoolSnapshotResponse {
    fn from(snapshot: PoolSnapshot) -> Self {
        Self {
            id: snapshot.id,
            pool_id: snapshot.pool_id,
            timestamp: snapshot.timestamp,
            position_data: snapshot.position_data,
            pool_data: snapshot.pool_data,
            apy: ApyValue::default(),
        }
    }
}

impl PoolSnapshotResponse {
    pub fn from_snapshot_with_apy(snapshot: PoolSnapshot, apy: ApyValue) -> Self {
        Self {
            id: snapshot.id,
            pool_id: snapshot.pool_id,
            timestamp: snapshot.timestamp,
            position_data: snapshot.position_data,
            pool_data: snapshot.pool_data,
            apy,
        }
    }
}