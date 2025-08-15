use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use serde::Serialize;

use crate::repository::runtime_config_repo::{self, RuntimeConfig};
use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::repository::{snapshots_repo, strategy_states_repo};
use crate::types::external_canister_types::StrategyId;
use crate::types::types::StrategyState;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct StableState {
    pub runtime_config: RuntimeConfig,
    pub snapshots: Vec<StrategySnapshot>,
    pub strategy_states: Vec<(StrategyId, StrategyState)>,
}

pub fn stable_save() {
    let snapshots = snapshots_repo::get_all_snapshots();
    let strategy_states = strategy_states_repo::get_all_strategy_states();
    let runtime_config = runtime_config_repo::get_runtime_config();
    let state = StableState { snapshots, strategy_states, runtime_config };
    storage::stable_save((state,)).unwrap();
}

pub fn stable_restore() {
    let (state,): (StableState,) = storage::stable_restore().unwrap();

    runtime_config_repo::set_runtime_config(state.runtime_config.clone());

    for snapshot in state.snapshots {
        snapshots_repo::save_snapshot(snapshot);
    }

    for (strategy_id, strategy_state) in state.strategy_states {
        strategy_states_repo::set_strategy_state(strategy_id, strategy_state);
    }
}
