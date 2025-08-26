use async_trait::async_trait;
use candid::{Nat, Principal};
use std::cell::RefMut;
use std::cmp::Ordering;
use std::collections::HashMap;

use liquidity::liquidity_calculator::LiquidityCalculator;
use types::exchange_id::ExchangeId;
use types::pool::PoolTrait;
use types::context::Context;
use utils::token_transfer::icrc1_transfer_to_user;
use ::types::strategies::{StrategyResponse, Pool};
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::vault as vault_domain,
    canisters::domains::vault::components as vault_domain_components,
};

use crate::repository::runtime_config_repo;
use crate::event_records::event_record::Event;
use crate::event_records::event_record_service;
use crate::repository::strategies_repo;
use crate::strategies::basic_strategy::BasicStrategy;
use crate::strategies::strategy_candid::StrategyCandid;
use crate::liquidity::liquidity_service;
use crate::strategies::stats::strategy_stats_service;
use crate::strategies::smart_rebalance_service;
use crate::types::types::{
    StrategyDepositResponse,
    StrategyRebalanceResponse,
    StrategyWithdrawResponse,
};

// Module code: "03-01-02"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,           // Area code: "03"
    vault_domain::DOMAIN_CODE,          // Domain code: "01"
    vault_domain_components::STRATEGIES // Component code: "02"
);

#[async_trait]
pub trait IStrategy: Send + Sync + BasicStrategy {
    /// Deposits an amount of tokens into the strategy
    ///
    /// # Arguments
    ///
    /// * `investor` - The Principal ID of the investor who is depositing tokens
    /// * `amount` - The amount of tokens to deposit
    ///
    /// # Returns
    ///
    /// A `StrategyDepositResponse` struct containing the following fields:
    /// - `amount`: The amount of tokens deposited
    /// - `shares`: The number of shares received
    /// - `tx_id`: The transaction ID (always 0 for this implementation)
    /// - `position_id`: The request ID from the deposit call
    ///
    /// # Details
    ///
    /// This function:
    /// 1. Retrieves the current pool from the strategy
    /// 2. Calculates the new shares for the investor's deposit
    /// 3. Updates the total balance and total shares
    /// 4. Updates the user shares mapping
    /// 5. Updates the initial deposit mapping
    /// 6. Adds liquidity to the pool
    /// 7. Saves the updated strategy state
    ///
    async fn deposit(
        &mut self,
        context: Context,
        amount: Nat,
    ) -> Result<StrategyDepositResponse, InternalError> {
        let strategy_id = self.get_id().to_string();
        let investor = context.user.unwrap();

        // Event: Strategy deposit started
        event_record_service::create_event_record(
            Event::strategy_deposit_started(strategy_id.clone(), None, Some(amount.clone())),
            context.correlation_id.clone(),
            Some(investor),
            context.strategy_id,
        );

        let mut current_pool = self.get_current_pool();

        // Set current pool to the best APY pool if not set
        if current_pool.is_none() {
            // Find the best APY pool
            let best_apy_pool = self.get_best_apy_pool().await;

            if best_apy_pool.is_none() {
                let error = InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 1), // Error code: "03-01-02 01 01"
                    "Strategy::deposit".to_string(),
                    "No pool found to deposit".to_string(),
                    errors::error_extra! {
                        "context" => context,
                        "strategy_id" => strategy_id,
                        "amount" => amount,
                    },
                );

                // Event: Strategy deposit failed
                event_record_service::create_event_record(
                    Event::strategy_deposit_failed(
                        strategy_id.clone(), 
                        None, 
                        Some(amount.clone()), 
                        error.clone()
                    ),
                    context.correlation_id,
                    Some(investor),
                    context.strategy_id,
                );

                return Err(error);
            }

            current_pool = best_apy_pool;
        }

        let current_pool = current_pool.unwrap();

        // Add liquidity to pool
        let add_liquidity_response = liquidity_service::add_liquidity_to_pool(
            context.clone(),
            amount.clone(),
            current_pool.clone()
        ).await?;

        let token0_equivalent_total = add_liquidity_response.token0_equivalent_total.clone();

        self.update_strategy_state_after_deposit(
            investor,
            token0_equivalent_total.clone(),
            current_pool.clone(),
            add_liquidity_response.position_id,
        );

        // Event: Strategy deposit completed
        event_record_service::create_event_record(
            Event::strategy_deposit_completed(
                strategy_id, 
                Some(current_pool.get_id()), 
                Some(token0_equivalent_total.clone())
            ),
            context.correlation_id,
            Some(investor),
            context.strategy_id,
        );

        Ok(StrategyDepositResponse {
            amount: token0_equivalent_total,
            shares: self.get_user_shares().get(&investor).unwrap().clone(),
            tx_id: 0,
            position_id: add_liquidity_response.position_id,
        })
    }

    /// Withdraws shares from the strategy and returns the corresponding tokens to the investor
    ///
    /// # Arguments
    ///
    /// * `shares` - The number of shares to withdraw
    ///
    /// # Returns
    ///
    /// * `StrategyWithdrawResponse` - Contains the amount of tokens withdrawn and remaining shares
    ///
    /// # Details
    ///
    /// This function:
    /// 1. Verifies the caller has sufficient shares
    /// 2. Gets the current pool and token information
    /// 3. Removes liquidity from the pool proportional to shares
    /// 4. Swaps secondary token to base token
    /// 5. Transfers total tokens to caller
    /// 6. Updates total shares, user shares and initial deposit
    /// 7. Saves updated strategy state
    ///
    /// TODO: Rename `shares` to `percentage`
    async fn withdraw(
        &mut self,
        context: Context,
        percentage: Nat
    ) -> Result<StrategyWithdrawResponse, InternalError> {
        let strategy_id = self.get_id().to_string();
        let investor = context.user.unwrap();
        let user_shares = self.get_user_shares_by_principal(investor.clone());
        let shares = user_shares.clone() * percentage.clone() / Nat::from(100u64); // TODO: Check this operation

        // Event: Strategy withdraw started
        event_record_service::create_event_record(
            Event::strategy_withdraw_started(strategy_id.clone(), None, Some(shares.clone())),
            context.correlation_id.clone(),
            Some(investor),
            context.strategy_id,
        );

        let current_pool = self.get_current_pool().clone();
        let current_pool_id = current_pool.clone().unwrap().get_id();

        if user_shares == Nat::from(0u8) {
            let error = InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 3), // Error code: "03-01-02 03 03"
                "Strategy::withdraw".to_string(),
                "No shares found for user".to_string(),
                errors::error_extra! {
                    "context" => context,
                    "strategy_id" => strategy_id,
                    "percentage" => percentage,
                    "user_shares" => user_shares,
                    "shares" => shares,
                },
            );

            // Event: Strategy withdraw failed
            event_record_service::create_event_record(
                Event::strategy_withdraw_failed(
                    strategy_id,
                    Some(current_pool_id),
                    Some(shares.clone()),
                    error.clone(),
                ),
                context.correlation_id,
                Some(investor),
                context.strategy_id,
            );

            return Err(error);
        }

        // Check if user has enough shares
        if shares > user_shares {
            let error = InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 4), // Error code: "03-01-02 03 04"
                "Strategy::withdraw".to_string(),
                "Not sufficient shares for user".to_string(),
                errors::error_extra! {
                    "context" => context,
                    "strategy_id" => strategy_id,
                    "percentage" => percentage,
                    "user_shares" => user_shares,
                    "shares" => shares,
                },
            );

            // Event: Strategy withdraw failed
            event_record_service::create_event_record(
                Event::strategy_withdraw_failed(
                    strategy_id,
                    Some(current_pool_id),
                    Some(shares.clone()),
                    error.clone(),
                ),
                context.correlation_id,
                Some(investor),
                context.strategy_id,
            );

            return Err(error);
        }

        if current_pool.is_none() {
            let error = InternalError::not_found(
                build_error_code(InternalErrorKind::NotFound, 5), // Error code: "03-01-02 01 05"
                "Strategy::withdraw".to_string(),
                "No current pool found in strategy".to_string(),
                errors::error_extra! {
                    "context" => context,
                    "strategy_id" => strategy_id,
                    "percentage" => percentage,
                    "user_shares" => user_shares,
                    "shares" => shares,
                },
            );

            // Event: Strategy withdraw failed
            event_record_service::create_event_record(
                Event::strategy_withdraw_failed(
                    strategy_id,
                    None,
                    Some(shares.clone()),
                    error.clone(),
                ),
                context.correlation_id,
                Some(investor),
                context.strategy_id,
            );

            return Err(error);
        }

        let current_pool = current_pool.unwrap();

        // Withdraw liquidity from pool and swap token_1 to token_0 (base token)
        let amount_0_to_withdraw = liquidity_service::withdraw_liquidity_from_pool_and_swap(
            context.clone(),
            self.get_total_shares(),
            shares.clone(),
            current_pool.clone(),
        ).await?;

        let environment = runtime_config_repo::get_current_env();

        // Transfer amount of token_0 (base token) to user
        icrc1_transfer_to_user(
            &environment,
            investor,
            current_pool.token0,
            amount_0_to_withdraw.clone(),
        ).await
            .map_err(|error| {
                // Event: Strategy withdraw failed
                event_record_service::create_event_record(
                    Event::strategy_withdraw_failed(
                        strategy_id.clone(),
                        Some(current_pool_id.clone()),
                        Some(shares.clone()),
                        error.clone(),
                    ),
                    context.correlation_id.clone(),
                    Some(investor),
                    context.strategy_id,
                );

                error
            })?;

        let new_user_shares = self.update_strategy_state_after_withdraw(
            investor,
            shares.clone(),
        );

        // Event: Strategy withdraw completed
        event_record_service::create_event_record(
            Event::strategy_withdraw_completed(
                strategy_id,
                Some(current_pool_id),
                Some(shares.clone()),
                Some(amount_0_to_withdraw.clone()),
            ),
            context.correlation_id,
            Some(investor),
            context.strategy_id,
        );

        Ok(StrategyWithdrawResponse {
            amount: amount_0_to_withdraw,
            current_shares: new_user_shares.clone(),
        })
    }

    /// Rebalances the strategy using Smart Rebalance scoring/decision
    ///
    /// # Details
    ///
    /// 1. Gets data for all available pools
    /// 2. Finds the pool with highest APY
    /// 3. If current pool is different from highest APY pool:
    ///    - Withdraws liquidity from current pool
    ///    - Swaps token_1 to token_0 (base token)
    ///    - Adds liquidity to new pool
    ///    - Updates current pool
    ///
    /// # Returns
    ///
    /// * `StrategyRebalanceResponse` - Contains:
    ///   * `pool` - The pool being used after rebalancing
    ///
    async fn rebalance(&mut self) -> Result<StrategyRebalanceResponse, InternalError> {
        let context = Context::generate(
            None,
            Some(self.get_id())
        );

        let strategy_id = self.get_id().to_string();

        // Event: Strategy rebalance started
        event_record_service::create_event_record(
            Event::strategy_rebalance_started(strategy_id.clone(), None),
            context.correlation_id.clone(),
            context.user,
            context.strategy_id,
        );

        let current_pool = self.get_current_pool();

        if current_pool.is_none() {
            let error = InternalError::not_found(
                build_error_code(InternalErrorKind::NotFound, 6), // Error code: "03-01-02 01 06"
                "Strategy::rebalance".to_string(),
                "No current pool found in strategy".to_string(),
                errors::error_extra! {
                    "context" => context,
                    "strategy_id" => strategy_id,
                },
            );

            event_record_service::create_event_record(
                Event::strategy_rebalance_failed(strategy_id, None, None, error.clone()),
                context.correlation_id,
                context.user,
                context.strategy_id,
            );

            return Err(error);
        }

        let current_pool = current_pool.unwrap();

        let position_value_usd = strategy_stats_service::get_strategy_current_liquidity_usd(
            self.clone_self()
        ).await?;

        let rebalance_decision = smart_rebalance_service::decide_rebalance(
            smart_rebalance_service::RebalanceInputs {
                current_pool: current_pool.clone(),
                pools: self.get_pools(),
                profile: smart_rebalance::types::StrategyProfile::Balanced,
                last_rebalance_at: None, // TODO: track in state
                position_value_usd,
            }
        ).await;

        if !rebalance_decision.should_move {
            return Ok(StrategyRebalanceResponse {
                previous_pool: current_pool.clone(),
                current_pool: current_pool.clone(),
                is_rebalanced: false,
            });
        }

        let target_pool_id = rebalance_decision.target_pool_id.unwrap();
        let max_apy_pool = self.get_pools()
            .into_iter()
            .find(|p| p.get_id() == target_pool_id)
            .unwrap();

        if current_pool.is_same_pool(&max_apy_pool) {
            // Event: Strategy rebalance completed
            event_record_service::create_event_record(
                Event::strategy_rebalance_completed(
                    strategy_id,
                    Some(current_pool.get_id()),
                    Some(max_apy_pool.get_id()),
                ),
                context.correlation_id,
                None,
                context.strategy_id,
            );

            return Ok(StrategyRebalanceResponse {
                previous_pool: current_pool.clone(),
                current_pool: current_pool.clone(),
                is_rebalanced: false,
            });
        }

        // Withdraw liquidity from current pool and swap token_1 to token_0 (base token)
        let token_0_to_pool_amount = liquidity_service::withdraw_liquidity_from_pool_and_swap(
            context.clone(),
            self.get_total_shares(),
            self.get_total_shares(),
            current_pool.clone(),
        ).await?;

        // Add liquidity to new pool
        let add_liquidity_response = liquidity_service::add_liquidity_to_pool(
            context.clone(),
            token_0_to_pool_amount.clone(),
            max_apy_pool.clone(),
        ).await?;

        // Event: Strategy rebalance completed
        event_record_service::create_event_record(
            Event::strategy_rebalance_completed(
                strategy_id,
                Some(current_pool.get_id()),
                Some(max_apy_pool.get_id()),
            ),
            context.correlation_id,
            context.user,
            context.strategy_id,
        );

        // Update current pool
        self.set_current_pool(Some(max_apy_pool));

        // Update position id
        self.set_position_id(Some(add_liquidity_response.position_id));

        Ok(StrategyRebalanceResponse {
            previous_pool: current_pool.clone(),
            current_pool: self.get_current_pool().unwrap(),
            is_rebalanced: true,
        })
    }

    fn update_user_shares(&mut self, user: Principal, shares: Nat) {
        let mut user_shares_map = self.get_user_shares();
        if shares == Nat::from(0u64) {
            user_shares_map.remove(&user);
        } else {
            user_shares_map.insert(user, shares);
        }
        self.set_user_shares(user_shares_map);
    }

    fn increase_user_shares(&mut self, user: Principal, shares: Nat) {
        let user_shares = self.get_user_shares().get(&user).cloned().unwrap_or(Nat::from(0u64));
        let new_user_shares = user_shares + shares.clone();
        self.update_user_shares(user, new_user_shares);
    }

    fn decrease_user_shares(&mut self, user: Principal, shares: Nat) {
        let user_shares = self.get_user_shares().get(&user).cloned().unwrap_or(Nat::from(0u64));
        let new_user_shares = user_shares - shares.clone();
        self.update_user_shares(user, new_user_shares);
    }

    fn update_initial_deposit(&mut self, user: Principal, initial_deposit: Nat) {
        let mut initial_deposit_map = self.get_initial_deposit();
        if initial_deposit == Nat::from(0u64) {
            initial_deposit_map.remove(&user);
        } else {
            initial_deposit_map.insert(user, initial_deposit);
        }
        self.set_initial_deposit(initial_deposit_map);
    }

    fn increase_initial_deposit(&mut self, user: Principal, initial_deposit: Nat) {
        let user_initial_deposit = self.get_initial_deposit().get(&user).cloned().unwrap_or(Nat::from(0u64));
        let new_initial_deposit = user_initial_deposit + initial_deposit.clone();
        self.update_initial_deposit(user, new_initial_deposit);
    }

    fn decrease_initial_deposit(&mut self, user: Principal, initial_deposit: Nat) {
        let user_initial_deposit = self.get_initial_deposit().get(&user).cloned().unwrap_or(Nat::from(0u64));
        let new_initial_deposit = user_initial_deposit - initial_deposit.clone();
        self.update_initial_deposit(user, new_initial_deposit);
    }

    fn increase_total_shares(&mut self, shares: Nat) {
        let new_total_shares = self.get_total_shares() + shares.clone();
        self.set_total_shares(new_total_shares);
    }

    fn decrease_total_shares(&mut self, shares: Nat) {
        let new_total_shares = self.get_total_shares() - shares.clone();
        self.set_total_shares(new_total_shares);
    }

    fn increase_total_balance(&mut self, balance: Nat) {
        let new_total_balance = self.get_total_balance() + balance.clone();
        self.set_total_balance(new_total_balance);
    }

    fn decrease_total_balance(&mut self, balance: Nat) {
        let new_total_balance = self.get_total_balance() - balance.clone();
        self.set_total_balance(new_total_balance);
    }

    // TODO: Test function. Remove after testing.
    async fn test_reset_strategy(&mut self) {
        self.set_user_shares(HashMap::new());
        self.set_total_shares(Nat::from(0u64));

        self.set_total_balance(Nat::from(0u64));
        self.set_initial_deposit(HashMap::new());

        self.set_current_pool(None);
        self.set_position_id(None);

        self.set_current_liquidity(None);
        self.set_current_liquidity_updated_at(None);

        strategies_repo::save_strategy(self.clone_self());
    }

    async fn get_best_apy_pool(&self) -> Option<Pool> {
        let strategy_pools = self.get_pools();
        let pools_data = liquidity_service::get_pools_data(strategy_pools).await; // TODO: handle error

        pools_data
            .iter()
            .filter(|x| x.pool.provider == ExchangeId::ICPSwap)
            // .max_by_key(|x| x.apy) // TODO: uncomment this after testing
            .map(|x| x.pool.clone())
            .next()
    }

    fn update_strategy_state_after_deposit(
        &mut self,
        investor: Principal,
        amount: Nat,
        pool: Pool,
        position_id: u64,
    ) -> Nat {
        // Calculate new shares for investor's deposit
        let new_user_shares = LiquidityCalculator::calculate_shares_for_deposit(
            amount.clone(),
            self.get_total_balance().clone(),
            self.get_total_shares().clone(),
        );

        // Update strategy state with new shares, initial deposit and total balance
        self.increase_total_shares(new_user_shares.clone());
        self.increase_user_shares(investor, new_user_shares.clone());
        self.increase_initial_deposit(investor, amount.clone());
        self.increase_total_balance(amount.clone());

        // Update current pool and position id
        self.set_current_pool(Some(pool.clone()));
        self.set_position_id(Some(position_id));

        strategies_repo::save_strategy(self.clone_self());

        // Update strategy current liquidity
        strategy_stats_service::spawn_update_strategy_liquidity(self.clone_self());

        new_user_shares
    }

    fn update_strategy_state_after_withdraw(
        &mut self,
        investor: Principal,
        shares: Nat,
    ) -> Nat {
        // Update total shares
        self.decrease_total_shares(shares.clone());

        // Update user shares
        let previous_user_shares = self.get_user_shares_by_principal(investor);
        let new_user_shares = previous_user_shares.clone() - shares.clone();
        self.update_user_shares(investor.clone(), new_user_shares.clone());

        // Update initial deposit proportional to the new shares
        let mut initial_deposit = self.get_initial_deposit();
        let user_initial_deposit = initial_deposit
            .get(&investor)
            .cloned()
            .unwrap_or(Nat::from(0u64));

        let new_user_initial_deposit = if previous_user_shares == Nat::from(0u64) {
            Nat::from(0u64)
        } else {
            user_initial_deposit.clone() * new_user_shares.clone() / previous_user_shares.clone()
        };

        if new_user_initial_deposit == Nat::from(0u64) {
            initial_deposit.remove(&investor);
        } else {
            initial_deposit.insert(investor.clone(), new_user_initial_deposit.clone());
        }
        self.set_initial_deposit(initial_deposit);

        // Update total balance
        let total_balance = self.get_total_balance().clone();
        let new_total_balance = total_balance - user_initial_deposit + new_user_initial_deposit.clone();
        self.set_total_balance(new_total_balance.clone());

        if self.get_total_shares() == Nat::from(0u64) {
            self.set_current_liquidity(None);
            self.set_position_id(None);
        }

        strategies_repo::save_strategy(self.clone_self());

        // Update strategy current liquidity
        strategy_stats_service::spawn_update_strategy_liquidity(self.clone_self());

        new_user_shares
    }

    fn to_candid(&self) -> StrategyCandid;

    /// Converts the strategy into a StrategyResponse struct that can be returned to clients
    ///
    /// # Returns
    ///
    /// * `StrategyResponse` - A struct containing:
    ///   * `name` - Name of the strategy
    ///   * `id` - Unique identifier for the strategy
    ///   * `description` - Description of what the strategy does
    ///   * `pools` - List of pool symbols this strategy can invest in
    ///   * `current_pool` - The pool currently being used, if any
    ///   * `total_shares` - Total number of shares issued by this strategy
    ///   * `user_shares` - Mapping of user principals to their share amounts
    ///   * `initial_deposit` - Mapping of user principals to their initial deposits
    fn to_response(&self) -> StrategyResponse {
        StrategyResponse {
            name: self.get_name(),
            id: self.get_id(),
            base_token: self.get_base_token(),
            description: self.get_description(),
            pools: self.get_pools(),
            current_pool: self.get_current_pool(),
            total_balance: self.get_total_balance(),
            total_shares: self.get_total_shares(),
            user_shares: self.get_user_shares(),
            initial_deposit: self.get_initial_deposit(),
            users_count: self.get_user_shares().len() as u32,
            current_liquidity: self.get_current_liquidity(),
            current_liquidity_updated_at: self.get_current_liquidity_updated_at(),
            position_id: self.get_position_id(),
            enabled: self.get_enabled(),
        }
    }

    fn clone_self(&self) -> Box<dyn IStrategy>;
}

impl Clone for Box<dyn IStrategy> {
    fn clone(&self) -> Box<dyn IStrategy> {
        self.as_ref().clone_self()
    }
}

pub struct StrategyIterator<'a> {
    inner: RefMut<'a, Vec<Box<dyn IStrategy>>>,
    index: usize,
}

impl<'a> StrategyIterator<'a> {
    pub fn new(trs: RefMut<'a, Vec<Box<dyn IStrategy>>>) -> Self {
        StrategyIterator {
            inner: trs,
            index: 0,
        }
    }
}

impl<'a> Iterator for StrategyIterator<'a> {
    type Item = Box<dyn IStrategy>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.inner.len() {
            // Clone the item at the current index
            let cloned_item = self.inner[self.index].clone();
            self.index += 1;
            Some(cloned_item)
        } else {
            None
        }
    }
}

impl Eq for dyn IStrategy {}

impl PartialEq for dyn IStrategy {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Ord for dyn IStrategy {
    fn cmp(&self, other: &Self) -> Ordering {
        other.get_id().cmp(&self.get_id())
    }
}

impl PartialOrd for dyn IStrategy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.get_id().cmp(&self.get_id()))
    }
}
