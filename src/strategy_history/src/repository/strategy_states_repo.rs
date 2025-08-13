use std::cell::RefCell;
use std::collections::HashMap;

use crate::types::types::{StrategyId, StrategyState};

thread_local! {
    static STRATEGY_STATES: RefCell<HashMap<StrategyId, StrategyState>> = RefCell::new(HashMap::new());
}

pub fn set_strategy_state(strategy_id: StrategyId, state: StrategyState) {
    STRATEGY_STATES.with(|states| {
        states.borrow_mut().insert(strategy_id, state);
    });
}

pub fn get_strategy_state(strategy_id: StrategyId) -> Option<StrategyState> {
    STRATEGY_STATES.with(|states| {
        states.borrow().get(&strategy_id).cloned()
    })
}

pub fn upsert_strategy_state<F>(strategy_id: StrategyId, updater: F)
where
    F: FnOnce(Option<StrategyState>) -> StrategyState,
{
    STRATEGY_STATES.with(|states| {
        let mut states_mut = states.borrow_mut();
        let current = states_mut.get(&strategy_id).cloned();
        let next = updater(current);
        states_mut.insert(strategy_id, next);
    });
}

pub fn get_all_strategy_states() -> Vec<(StrategyId, StrategyState)> {
    STRATEGY_STATES.with(|states| {
        states.borrow().iter().map(|(k, v)| (*k, v.clone())).collect()
    })
}

pub fn get_all_initialized_strategy_states() -> Vec<(StrategyId, StrategyState)> {
    STRATEGY_STATES.with(|states| {
        states.borrow()
            .iter()
            .filter(|(_, v)| v.is_initialized)
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    })
}

pub fn delete_strategy_state(strategy_id: StrategyId) {
    STRATEGY_STATES.with(|states| {
        states.borrow_mut().remove(&strategy_id);
    });
}

pub fn delete_all_strategy_states() {
    STRATEGY_STATES.with(|states| {
        states.borrow_mut().clear();
    });
}
