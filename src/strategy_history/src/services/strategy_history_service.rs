use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::strategy_history as strategy_history_domain,
    canisters::domains::strategy_history::components as strategy_history_domain_components,
};

use crate::repository::strategy_states_repo;
use crate::repository::snapshots_repo;
use crate::vault::vault_service;
use crate::services::strategy_states_service;
use crate::services::strategy_snapshots_service;
use crate::types::types::{
    InitializeStrategyStatesAndCreateSnapshotsResponse,
    StrategyHistory,
};

// Module code: "03-03-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,                    // Area code: "03"
    strategy_history_domain::DOMAIN_CODE,        // Domain code: "03"
    strategy_history_domain_components::CORE     // Component code: "01"
);

pub async fn initialize_strategy_states_and_create_snapshots() -> Result<InitializeStrategyStatesAndCreateSnapshotsResponse, InternalError> {
    // Get strategies data from vault
    let vault_strategies = vault_service::get_vault_actor().await?
        .get_strategies().await
        .map_err(|e| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 1), // Error code: "03-03-01 03 01"
                "strategy_history_service::initialize_strategy_states_and_create_snapshots".to_string(),
                format!("Failed to fetch strategies from vault: {:?}", e),
                None,
            )
        })?;

    // Filter test strategies for now
    let vault_strategies = vault_strategies.iter()
        .filter(|s| s.test)
        .cloned()
        .collect::<Vec<_>>();

    // Ensure strategies are initialized before snapshotting using already fetched list
    let _initialize_strategy_states_response = 
        strategy_states_service::initialize_strategy_states_with_list(
            &vault_strategies, 
            None
        ).await?;

    let initialized_strategy_states = 
        strategy_states_repo::get_all_initialized_strategy_states();

    // Save snapshots for each strategy (only when initialized and with liquidity)
    let create_strategies_snapshots_response = 
        strategy_snapshots_service::create_strategies_snapshots(
            &initialized_strategy_states,
            &vault_strategies,
        )?;

    let success_count = create_strategies_snapshots_response.success_count;
    let errors = create_strategies_snapshots_response.errors;

    // If there were any errors, return a combined error message
    if !errors.is_empty() {
        let error_message = format!(
            "Failed to save {} out of {} snapshots. Errors: {}",
            errors.len(),
            vault_strategies.len(),
            errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("; ")
        );

        return Err(InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 2), // Error code: "03-03-01 03 02"
            "strategy_history_service::initialize_and_snapshot_strategies".to_string(),
            error_message,
            None,
        ));
    }

    Ok(InitializeStrategyStatesAndCreateSnapshotsResponse {
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
            build_error_code(InternalErrorKind::BusinessLogic, 3), // Error code: "03-03-01 03 03"
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
