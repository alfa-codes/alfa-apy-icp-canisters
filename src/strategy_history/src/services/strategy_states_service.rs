use candid::Nat;
use candid::Principal;

use ::utils::constants::{ICP_TOKEN_CANISTER_ID, VAULT_PRINCIPAL_DEV};
use ::utils::util::nat_to_f64;
use swap::swap_service;
use errors::internal_error::error::{InternalError, build_error_code};

use crate::repository::strategy_states_repo;
use crate::vault::vault_service;
use crate::utils::service_resolver;
use crate::strategy_snapshot::strategy_snapshot::Pool;
use crate::types::types::{StrategyState, InitializeStrategyStatesResponse, TestLiquidityData};
use crate::types::external_canister_types::{
    StrategyId,
    StrategyDepositArgs,
    StrategyDepositResponse,
    VaultStrategyResponse,
};

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
    vault_strategies: &Vec<VaultStrategyResponse>,
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

        match deposit_test_liquidity_to_strategy(&vault_strategy).await {
            Ok(deposit_response) => {
                strategy_states_repo::upsert_strategy_state(
                    vault_strategy.id,
                    |prev| {
                        let mut st = prev.unwrap_or_default();
                        st.initialized_at = Some(ic_cdk::api::time() / 1_000_000);
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
                        st.last_error = Some(format!("{:?}", err));
                        st
                    }
                );
                failed_strategy_states.push(
                    (vault_strategy.id, "Failed to deposit test liquidity".to_string())
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

// =============== Private methods ===============

async fn deposit_test_liquidity_to_strategy(
    vault_strategy: &VaultStrategyResponse
) -> Result<StrategyDepositResponse, InternalError> {
    // Pick base token ledger (token0) from current pool or first pool
    let pool = vault_strategy.current_pool.clone().unwrap_or_else(|| {
        vault_strategy.pools[0].clone()
    });

    // Compute minimal safe deposit for this strategy/pool
    let minimal_token0_deposit = match compute_liquidity_amount_for_deposit(
        pool.clone()
    ).await {
        Some(amount) => amount,
        None => {
            return Err(InternalError::business_logic(
                build_error_code(0000, 0, 0),
                "strategy_history_service::deposit_test_liquidity_to_strategy".to_string(),
                "Failed to compute minimal deposit".to_string(),
                None,
            ));
        }
    };

    // If base token is not ICP, first swap ICP -> base token, then deposit acquired amount
    let (deposit_token_ledger, deposit_amount) =
        if pool.token0 != *ICP_TOKEN_CANISTER_ID {
            let swapped = swap_icp_to_target_token_for_amount(
                pool.token0,
                minimal_token0_deposit.clone()
            ).await?;

            (pool.token0, swapped)
        } else {
            (pool.token0, minimal_token0_deposit.clone())
        };

    let args = StrategyDepositArgs {
        ledger: deposit_token_ledger,
        amount: deposit_amount.clone(),
        strategy_id: vault_strategy.id,
    };

    let vault_actor = vault_service::get_vault_actor().await?;

    // Ensure allowance for vault to pull funds from this canister on the selected ledger
    approve_deposit_allowance(
        Principal::from_text(VAULT_PRINCIPAL_DEV).unwrap(),
        deposit_token_ledger,
        deposit_amount.clone(),
    ).await?;

    // Call vault deposit
    match vault_actor.deposit(args).await {
        Ok(response) => Ok(response),
        Err(err) => {
            return Err(InternalError::business_logic(
                build_error_code(0000, 0, 0),
                "strategy_history_service::deposit_test_liquidity_to_strategy".to_string(),
                format!("Deposit call failed: {:?}", err),
                None,
            ));
        }
    }
}

async fn compute_liquidity_amount_for_deposit(pool: Pool) -> Option<Nat> {
    let service_resolver = service_resolver::get_service_resolver();
    let icrc_ledger_client = service_resolver.icrc_ledger_client();

    let decimals_token0 = icrc_ledger_client.icrc1_decimals(pool.token0).await.ok()?;

    let base_unit_token0 = Nat::from(10u128.pow(decimals_token0 as u32));

    // Fees
    let transfer_fee_token0 = icrc_ledger_client
        .icrc1_fee(pool.token0)
        .await
        .unwrap_or_else(|_| Nat::from(0u64));

    // Minimum per-side requirements in native subunits
    let required_token0 = 
        transfer_fee_token0.clone() + transfer_fee_token0.clone() + base_unit_token0.clone();

    let safety_coefficient = Nat::from(1u64);
    let total_required_token0 = required_token0.clone() * Nat::from(2u64) * safety_coefficient;

    Some(total_required_token0)
}

async fn swap_icp_to_target_token_for_amount(
    target_token: ::types::CanisterId,
    target_amount_out: Nat,
) -> Result<Nat, InternalError> {
    let service_resolver = service_resolver::get_service_resolver();
    let icrc_ledger_client = service_resolver.icrc_ledger_client();

    // Estimate price on small sample to compute required ICP amount with safety margin
    let icp_decimals = icrc_ledger_client
        .icrc1_decimals(*ICP_TOKEN_CANISTER_ID)
        .await
        .unwrap_or(8);

    let icp_base_unit = Nat::from(10u128.pow(icp_decimals as u32));
    let sample_icp_amount = icp_base_unit.clone() * Nat::from(10u64);

    let quote_icp_to_target = swap_service::quote_swap_icrc2_optimal(
        service_resolver.provider_impls(),
        *ICP_TOKEN_CANISTER_ID,
        target_token,
        sample_icp_amount.clone(),
    )
    .await
    .map_err(|e| InternalError::business_logic(
        build_error_code(0000, 0, 0),
        "strategy_history_service::swap_icp_to_target_for_amount: quote ICP->target".to_string(),
        format!("Quote failed: {:?}", e),
        None,
    ))?;

    let target_per_icp_price = 
        (quote_icp_to_target.amount_out as f64) / nat_to_f64(&sample_icp_amount).max(1.0);

    let required_icp_amount_f64 =
        (nat_to_f64(&target_amount_out) / target_per_icp_price) * 1.2; // 20% safety

    let required_icp_amount = Nat::from(required_icp_amount_f64.ceil() as u128);

    let swap_response = swap_service::swap_icrc2_optimal(
        service_resolver.provider_impls(),
        service_resolver.icrc_ledger_client(),
        *ICP_TOKEN_CANISTER_ID,
        target_token,
        required_icp_amount,
    )
    .await
    .map_err(|e| InternalError::business_logic(
        build_error_code(0000, 0, 0),
        "strategy_history_service::swap_icp_to_target_for_amount: swap ICP->target".to_string(),
        format!("Swap failed: {:?}", e),
        None,
    ))?;

    Ok(Nat::from(swap_response.amount_out))
}

fn build_test_liquidity_data(deposit_response: StrategyDepositResponse) -> TestLiquidityData {
    TestLiquidityData {
        amount: deposit_response.amount,
        shares: deposit_response.shares,
        tx_id: deposit_response.tx_id,
        position_id: deposit_response.position_id,
    }
}

async fn approve_deposit_allowance(
    spender: Principal,
    ledger: ::types::CanisterId,
    amount: Nat,
) -> Result<Nat, InternalError> {
    let service_resolver = service_resolver::get_service_resolver();
    let icrc_ledger_client = service_resolver.icrc_ledger_client();

    icrc_ledger_client
        .icrc2_approve(spender, ledger, amount)
        .await
}
