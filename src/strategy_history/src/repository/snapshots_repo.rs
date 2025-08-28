use std::cell::RefCell;
use std::collections::HashMap;

use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;

thread_local! {
    static STRATEGY_SNAPSHOTS: RefCell<HashMap<u16, Vec<StrategySnapshot>>> = RefCell::new(HashMap::new());
}

pub fn save_snapshot(snapshot: StrategySnapshot) {
    let strategy_id = snapshot.strategy_id;

    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        let mut strategy_snapshots = strategy_snapshots.borrow_mut();
        let snapshots = strategy_snapshots
            .entry(strategy_id)
            .or_insert_with(Vec::new);
        
        snapshots.push(snapshot);
    });
}

pub fn get_snapshot_by_id(snapshot_id: String) -> Option<StrategySnapshot> {
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        let strategy_snapshots = strategy_snapshots.borrow();
        for snapshots in strategy_snapshots.values() {
            if let Some(snapshot) = snapshots
                .iter()
                .find(|s| s.id == snapshot_id) {
                    return Some(snapshot.clone());
                }
        }
        None
    })
}

pub fn get_snapshots_by_strategy_id(strategy_id: u16) -> Vec<StrategySnapshot> {
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        strategy_snapshots.borrow()
            .get(&strategy_id)
            .cloned()
            .unwrap_or_default()
    })
}

pub fn get_snapshots_by_strategy_id_in_range(
    strategy_id: u16,
    from_timestamp: u64,
    to_timestamp: u64,
) -> Vec<StrategySnapshot> {
    get_snapshots_by_strategy_id(strategy_id)
        .into_iter()
        .filter(|snapshot| {
            snapshot.timestamp >= from_timestamp && snapshot.timestamp <= to_timestamp
        })
        .collect()
}

pub fn get_latest_snapshot_by_strategy_id(strategy_id: u16) -> Option<StrategySnapshot> {
    let snapshots = get_snapshots_by_strategy_id(strategy_id);
    snapshots.into_iter().max_by_key(|snapshot| snapshot.timestamp)
}

pub fn get_snapshots_count_by_strategy_id(strategy_id: u16) -> u64 {
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        strategy_snapshots.borrow()
            .get(&strategy_id)
            .map(|snapshots| snapshots.len() as u64)
            .unwrap_or(0)
    })
}

pub fn get_all_snapshots() -> Vec<StrategySnapshot> {
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        strategy_snapshots.borrow()
            .values()
            .flatten()
            .cloned()
            .collect()
    })
}

pub fn get_all_snapshots_in_range(
    from_timestamp: u64,
    to_timestamp: u64,
) -> Vec<StrategySnapshot> {
    get_all_snapshots()
        .into_iter()
        .filter(|snapshot| {
            snapshot.timestamp >= from_timestamp && snapshot.timestamp <= to_timestamp
        })
        .collect()
}

pub fn get_snapshots_by_strategy_ids_in_range(
    strategy_ids: Vec<u16>,
    from_timestamp: u64,
    to_timestamp: u64,
) -> Vec<StrategySnapshot> {
    get_all_snapshots_in_range(from_timestamp, to_timestamp)
        .into_iter()
        .filter(|snapshot| strategy_ids.contains(&snapshot.strategy_id))
        .collect()
}

pub fn get_all_snapshots_grouped() -> HashMap<u16, Vec<StrategySnapshot>> {
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        strategy_snapshots.borrow().clone()
    })
}

pub fn get_all_snapshots_grouped_in_range(
    from_timestamp: u64,
    to_timestamp: u64,
) -> HashMap<u16, Vec<StrategySnapshot>> {
    get_all_snapshots_grouped()
        .into_iter()
        .fold(HashMap::new(), |mut acc, (strategy_id, snapshots)| {
            let filtered_snapshots: Vec<StrategySnapshot> = snapshots
                .into_iter()
                .filter(|snapshot| {
                    snapshot.timestamp >= from_timestamp && snapshot.timestamp <= to_timestamp
                })
                .collect();
            
            if !filtered_snapshots.is_empty() {
                acc.insert(strategy_id, filtered_snapshots);
            }
            acc
        })
}

pub fn get_snapshots_grouped_by_strategy_ids_in_range(
    strategy_ids: Vec<u16>,
    from_timestamp: u64,
    to_timestamp: u64,
) -> HashMap<u16, Vec<StrategySnapshot>> {
    let strategy_ids_set: std::collections::HashSet<u16> = strategy_ids.into_iter().collect();
    
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        strategy_snapshots.borrow()
            .iter()
            .filter(|(strategy_id, _)| strategy_ids_set.contains(strategy_id))
            .fold(HashMap::new(), |mut acc, (strategy_id, snapshots)| {
                let filtered_snapshots: Vec<StrategySnapshot> = snapshots
                    .iter()
                    .filter(|snapshot| {
                        snapshot.timestamp >= from_timestamp && snapshot.timestamp <= to_timestamp
                    })
                    .cloned()
                    .collect();
                
                if !filtered_snapshots.is_empty() {
                    acc.insert(*strategy_id, filtered_snapshots);
                }
                acc
            })
    })
}

pub fn delete_snapshots_by_strategy_id(strategy_id: u16) {
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        strategy_snapshots.borrow_mut().remove(&strategy_id);
    });
}

pub fn delete_snapshot_by_id(snapshot_id: String) {
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        let mut strategy_snapshots = strategy_snapshots.borrow_mut();
        for snapshots in strategy_snapshots.values_mut() {
            snapshots.retain(|snapshot| snapshot.id != snapshot_id);
        }
    });
}

pub fn delete_all_snapshots() {
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        strategy_snapshots.borrow_mut().clear();
    });
}

pub fn delete_all_snapshots_for_strategy(strategy_id: u16) {
    STRATEGY_SNAPSHOTS.with(|strategy_snapshots| {
        strategy_snapshots.borrow_mut().remove(&strategy_id);
    });
}