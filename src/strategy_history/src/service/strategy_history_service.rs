use std::time::Duration;
use ic_cdk_timers::TimerId;
use std::cell::RefCell;

use errors::internal_error::error::{InternalError, build_error_code};
use validation::validation::Validation;

use crate::strategy_snapshot::strategy_snapshot::StrategySnapshot;
use crate::types::types::{FetchAndSaveStrategiesResponse, StrategyHistory};
use crate::repository::snapshots_repo;
use crate::vault::vault_service;

thread_local! {
    static STRATEGY_HISTORY_FETCHING_TIMER_ID: RefCell<Option<TimerId>> = RefCell::new(None);
}

fn set_timer_interval(
    interval: Duration,
    func: impl FnMut() + 'static,
) -> TimerId {
    ic_cdk_timers::set_timer_interval(interval, func)
}

pub fn start_fetching_timer(interval: u64) {
    let timer_id = set_timer_interval(Duration::from_secs(interval), || {
        ic_cdk::spawn(async {
            let _ =fetch_and_save_strategies().await;
        });
    });

    STRATEGY_HISTORY_FETCHING_TIMER_ID.with(|cell| {
        cell.replace(Some(timer_id));
    });
}

pub fn stop_fetching_timer() {
    STRATEGY_HISTORY_FETCHING_TIMER_ID.with(|timer_id| {
        if let Some(timer_id) = timer_id.borrow_mut().take() {
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}

pub fn save_strategy_snapshot(snapshot: StrategySnapshot) -> Result<(), InternalError> {
    // Validate snapshot
    snapshot.define_validations().validate()
        .map_err(|errors| {
            if let Some(first_error) = errors.first() {
                first_error.clone()
            } else {
                InternalError::business_logic(
                    build_error_code(0000, 0, 0),
                    "strategy_history_service::save_snapshot".to_string(),
                    "Validation failed".to_string(),
                    None
                )
            }
        })?;
    
    snapshots_repo::save_snapshot(snapshot);
    Ok(())
}

pub fn get_strategy_snapshots_count(strategy_id: u16) -> u64 {
    snapshots_repo::get_snapshots_count_by_strategy_id(strategy_id)
}

pub async fn fetch_and_save_strategies() -> Result<FetchAndSaveStrategiesResponse, InternalError> {
    let vault_actor = vault_service::get_vault_actor().await?;
    
    // Get strategies data from vault
    let vault_strategies = vault_actor.get_strategies().await
        .map_err(|e| {
            InternalError::business_logic(
                build_error_code(0000, 0, 0),
                    "strategy_history_service::fetch_and_save_strategies".to_string(),
                    format!("Failed to fetch strategies from vault: {:?}", e),
                    None,
            )
        })?;

    let mut errors = Vec::new();
    let mut success_count = 0;

    // Save snapshots for each strategy
    for vault_strategy in vault_strategies.clone() {
        let snapshot = StrategySnapshot::build(
            vault_strategy.id,
            vault_strategy.total_balance,
            vault_strategy.total_shares,
            vault_strategy.current_liquidity,
            vault_strategy.position_id,
            vault_strategy.users_count,
            vault_strategy.current_liquidity_updated_at,
            vault_strategy.current_pool,
            10.0 // TODO: calculate apy
        );

        match save_strategy_snapshot(snapshot) {
            Ok(_) => {
                success_count += 1;
            }
            Err(error) => {
                errors.push(format!(
                    "Failed to save snapshot for strategy {}: {:?}",
                    vault_strategy.id,
                    error
                ));
            }
        }
    }

    // If there were any errors, return a combined error message
    if !errors.is_empty() {
        let error_message = format!(
            "Failed to save {} out of {} snapshots. Errors: {}",
            errors.len(),
            vault_strategies.len(),
            errors.join("; ")
        );
        
        return Err(InternalError::business_logic(
            build_error_code(0000, 0, 0),
            "strategy_history_service::fetch_and_save_strategies".to_string(),
            error_message,
            None,
        ));
    }

    Ok(FetchAndSaveStrategiesResponse {
        success_count,
        errors,
    })
}

pub async fn get_strategies_history(
    strategy_ids: Option<Vec<u16>>,
    from_timestamp: Option<u64>,
    to_timestamp: Option<u64>,
) -> Result<Vec<StrategyHistory>, InternalError> {
    // Set default values for optional fields
    let strategy_ids = strategy_ids.unwrap_or_default();
    let from_timestamp = from_timestamp.unwrap_or(0); // Default to beginning of time
    let to_timestamp = to_timestamp.unwrap_or(u64::MAX); // Default to end of time

    if from_timestamp > to_timestamp {
        return Err(InternalError::business_logic(
            build_error_code(0000, 0, 0),
            "strategy_history_service::get_strategies_history".to_string(),
            "from_timestamp cannot be greater than to_timestamp".to_string(),
            None,
        ));
    }

    let snapshots_by_strategy = if strategy_ids.is_empty() {
        // If strategy_ids is empty, get all strategies
        snapshots_repo::get_all_snapshots_grouped_in_range(from_timestamp, to_timestamp)
    } else {
        // Get only specified strategies
        snapshots_repo::get_snapshots_grouped_by_strategy_ids_in_range(
            strategy_ids,
            from_timestamp,
            to_timestamp,
        )
    };
    let strategy_metrics = snapshots_by_strategy
        .iter()
        .map(|(strategy_id, snapshots)| {
            StrategyHistory {
                strategy_id: *strategy_id,
                snapshots: snapshots.clone(),
            }
        }).collect();

    Ok(strategy_metrics)
}