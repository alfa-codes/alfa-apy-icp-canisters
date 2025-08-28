use candid::Nat;
use candid::Principal;

use ::utils::constants::ICP_TOKEN_CANISTER_ID;
use ::utils::util::current_timestamp_secs;
use ::types::CanisterId;
use swap::swap_service;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::strategy_history as strategy_history_domain,
    canisters::domains::strategy_history::components as strategy_history_domain_components,
};
use ::types::strategies::{StrategyId, StrategyResponse};

use crate::repository::strategy_states_repo;
use crate::vault::vault_service;
use crate::utils::service_resolver;
use crate::types::types::{
    StrategyState,
    InitializeStrategyStatesResponse,
    TestLiquidityData,
};
use crate::types::external_canister_types::{
    StrategyDepositArgs,
    StrategyDepositResponse,
};

const ICP_AMOUNT_FOR_DEPOSIT: u64 = 10_000_000; // 0.1 ICP

// Module code: "03-03-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,                    // Area code: "03"
    strategy_history_domain::DOMAIN_CODE,        // Domain code: "03"
    strategy_history_domain_components::CORE     // Component code: "01"
);

pub fn get_strategy_state(strategy_id: StrategyId) -> Option<StrategyState> {
    strategy_states_repo::get_strategy_state(strategy_id)
}

pub fn get_all_strategy_states() -> Vec<(StrategyId, StrategyState)> {
    strategy_states_repo::get_all_strategy_states()
}

pub fn delete_strategy_state(strategy_id: StrategyId) {
    strategy_states_repo::delete_strategy_state(strategy_id);
}

pub async fn initialize_strategy_states_with_list(
    vault_strategies: &Vec<StrategyResponse>,
    strategy_ids: Option<Vec<StrategyId>>,
) -> Result<InitializeStrategyStatesResponse, InternalError> {
    let filter_ids = strategy_ids.unwrap_or_default();
    let vault_strategies_iter = vault_strategies
        .iter()
        .cloned()
        .filter(|s| filter_ids.is_empty() || filter_ids.contains(&s.id));

    let mut initialized_strategy_states = Vec::new();
    let mut skipped_already_initialized_strategy_states = Vec::new();
    let mut failed_strategy_states = Vec::new();

    for vault_strategy in vault_strategies_iter {
        let strategy_state = strategy_states_repo::get_strategy_state(
            vault_strategy.id
        ).unwrap_or_default();

        if strategy_state.is_initialized() {
            skipped_already_initialized_strategy_states.push(vault_strategy.id);
            continue;
        }

        let deposit_icp_amount = Nat::from(ICP_AMOUNT_FOR_DEPOSIT);

        let deposit_test_liquidity_to_strategy_result =
            deposit_test_liquidity_to_strategy(&vault_strategy, deposit_icp_amount).await;

        match deposit_test_liquidity_to_strategy_result {
            Ok(deposit_response) => {
                strategy_states_repo::upsert_strategy_state(
                    vault_strategy.id,
                    |prev| {
                        let mut st = prev.unwrap_or_default();
                        st.initialized_at = Some(current_timestamp_secs());
                        st.test_liquidity_data = Some(build_test_liquidity_data(deposit_response));
                        st.initialize_attempts = st.initialize_attempts.saturating_add(1);
                        st.last_error = None;
                        st
                    }
                );
                initialized_strategy_states.push(vault_strategy.id);
            }
            Err(err) => {
                strategy_states_repo::upsert_strategy_state(
                    vault_strategy.id,
                    |prev| {
                        let mut st = prev.unwrap_or_default();
                        st.initialize_attempts = st.initialize_attempts.saturating_add(1);
                        st.last_error = Some(err.to_string());
                        st
                    }
                );
                failed_strategy_states.push(
                    (vault_strategy.id, err)
                );
            }
        }
    }

    Ok(InitializeStrategyStatesResponse {
        initialized_strategy_states,
        skipped_already_initialized_strategy_states,
        failed_strategy_states,
    })
}

pub async fn deposit_test_liquidity_to_strategy(
    vault_strategy: &StrategyResponse,
    deposit_icp_amount: Nat,
) -> Result<StrategyDepositResponse, InternalError> {
    let service_resolver = service_resolver::get_service_resolver();
    let icrc_ledger_client = service_resolver.icrc_ledger_client();
    let vault_actor = vault_service::get_vault_actor().await?;

    let base_token = vault_strategy.base_token.clone();

    // Swap ICP → base_token or return ICP as is
    let deposit_amount = if base_token != *ICP_TOKEN_CANISTER_ID {
        swap_icp_to_base_token(base_token.clone(), deposit_icp_amount).await?
    } else {
        deposit_icp_amount
    };

    let base_token_fee = icrc_ledger_client.icrc1_fee(base_token).await?;

    let available_for_deposit = if deposit_amount > base_token_fee {
        deposit_amount - base_token_fee
    } else {
        Nat::from(0u64)
    };

    // Ensure allowance for vault to pull funds from this canister on the selected ledger
    approve_icrc2_allowance_for_deposit(
        vault_actor.get_principal().await,
        base_token,
        available_for_deposit.clone(),
    ).await?;

    let args = StrategyDepositArgs {
        strategy_id: vault_strategy.id,
        ledger: base_token,
        amount: available_for_deposit.clone(),
    };

    // Call vault deposit
    match vault_actor.deposit(args).await {
        Ok(response) => Ok(response),
        Err(err) => {
            return Err(InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 6), // Error code: "03-03-01 03 06"
                "strategy_history_service::deposit_test_liquidity_to_strategy".to_string(),
                format!("Deposit call failed: {:?}", err),
                errors::error_extra! {
                    "strategy_id" => vault_strategy.id,
                    "base_token" => base_token.to_text(),
                    "amount" => available_for_deposit,
                },
            ));
        }
    }
}

// =============== Private methods ===============

fn build_test_liquidity_data(deposit_response: StrategyDepositResponse) -> TestLiquidityData {
    TestLiquidityData {
        amount: deposit_response.amount,
        shares: deposit_response.shares,
        tx_id: deposit_response.tx_id,
        position_id: deposit_response.position_id,
    }
}

async fn approve_icrc2_allowance_for_deposit(
    spender: Principal,
    ledger: CanisterId,
    amount: Nat,
) -> Result<Nat, InternalError> {
    let service_resolver = service_resolver::get_service_resolver();
    let icrc_ledger_client = service_resolver.icrc_ledger_client();

    icrc_ledger_client.icrc2_approve(
        spender,
        ledger,
        amount
    ).await
}

async fn swap_icp_to_base_token(
    base_token: CanisterId,
    icp_amount: Nat,
) -> Result<Nat, InternalError> {
    if base_token == *ICP_TOKEN_CANISTER_ID {
        return Ok(icp_amount.clone());
    }

    let service_resolver = service_resolver::get_service_resolver();

    let quote_response = swap_service::quote_swap_icrc2_optimal(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        *ICP_TOKEN_CANISTER_ID,
        base_token,
        icp_amount.clone(),
    ).await?;

    let swap_response = swap_service::swap_icrc2(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        *ICP_TOKEN_CANISTER_ID,
        base_token,
        icp_amount.clone(),
        quote_response.provider,
    )
    .await
    .map_err(|e| {
        InternalError::business_logic(
            build_error_code(InternalErrorKind::BusinessLogic, 7), // Error code: "03-03-01 03 07"
            "strategy_history_service::swap_icp_to_base_token".to_string(),
            format!("Swap failed: {:?}", e),
            errors::error_extra! {
                "base_token" => base_token.to_text(),
                "icp_amount" => icp_amount,
                "quote_response" => quote_response,
            },
        )
    })?;

    Ok(Nat::from(swap_response.amount_out))
}
