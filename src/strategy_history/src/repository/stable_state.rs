use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use serde::Serialize;

use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::repository::snapshots_repo;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct StableState {
    pub snapshots: Vec<StrategySnapshot>,
}

pub fn stable_save() {
    let snapshots = snapshots_repo::get_all_snapshots();
    let state = StableState { snapshots };
    storage::stable_save((state,)).unwrap();
}

pub fn stable_restore() {
    let (state,): (StableState,) = storage::stable_restore().unwrap();
    
    for snapshot in state.snapshots {
        snapshots_repo::save_snapshot(snapshot);
    }
}
