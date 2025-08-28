use async_trait::async_trait;
use candid::Nat;
use std::ops::{Div, Mul};   
use std::sync::Arc;

use types::{CanisterId, exchange_id::ExchangeId};
use service_resolver::ProviderImpls;
use providers::kongswap::KongSwapProvider;
use providers::icpswap::ICPSwapProvider;
use kongswap_canister::user_balances::UserBalancesReply;
use utils::util::{nat_to_f64, nat_to_u128};
use swap::swap_service;
use types::liquidity::{
    AddLiquidityResponse,
    WithdrawLiquidityResponse,
    GetPositionByIdResponse,
    GetPoolDataResponse
};
use icrc_ledger_client::ICRCLedgerClient;
use utils::constants::CKUSDT_TOKEN_CANISTER_ID;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    libraries as library_area,
    libraries::domains::liquidity as liquidity_domain,
    libraries::domains::liquidity::components as liquidity_domain_components,
};


use crate::liquidity_client::LiquidityClient;
use crate::liquidity_calculator::LiquidityCalculator;

pub const PROVIDER: ExchangeId = ExchangeId::KongSwap;

// Module code: "02-02-02"
errors::define_error_code_builder_fn!(
    build_error_code,
    library_area::AREA_CODE,                      // Area code: "02"
    liquidity_domain::DOMAIN_CODE,                // Domain code: "02"
    liquidity_domain_components::KONG_SWAP_CLIENT // Component code: "02"
);

pub struct KongSwapLiquidityClient {
    provider_impls: ProviderImpls,
    icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
    canister_id: CanisterId,
    // TODO: change to token0 and token1 to Pool
    token0: CanisterId,
    token1: CanisterId,
}

impl KongSwapLiquidityClient {
    pub fn new(
        provider_impls: ProviderImpls,
        icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
        canister_id: CanisterId,
        token0: CanisterId,
        token1: CanisterId,
    ) -> KongSwapLiquidityClient {
        KongSwapLiquidityClient {
            provider_impls,
            icrc_ledger_client,
            canister_id,
            token0,
            token1,
        }
    }

    fn token_kongswap_format(&self, token: CanisterId) -> String {
        format!("IC.{}", token.to_text())
    }

    fn kongswap_provider(&self) -> &Arc<dyn KongSwapProvider + Send + Sync> {
        &self.provider_impls.kongswap
    }

    fn icpswap_provider(&self) -> &Arc<dyn ICPSwapProvider + Send + Sync> {
        &self.provider_impls.icpswap
    }
}

#[async_trait]
impl LiquidityClient for KongSwapLiquidityClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn add_liquidity_to_pool(
        &self,
        amount: Nat
    ) -> Result<AddLiquidityResponse, InternalError> {
        let provider_add_liquidity_amounts =
            self.kongswap_provider().add_liquidity_amounts(
                self.token_kongswap_format(self.token0.clone()),
                amount.clone(),
                self.token_kongswap_format(self.token1.clone()),
            ).await?;

        let provider_suggested_token0_for_pool = provider_add_liquidity_amounts.amount_0;
        let provider_suggested_token1_for_pool = provider_add_liquidity_amounts.amount_1;

        // Fetch token fees to reserve for subsequent transfers
        // Note: approvals and provider pulls may require balance to cover fee per token
        let token0_transfer_fee = self.icrc_ledger_client.icrc1_fee(self.token0.clone()).await?;
        let token1_transfer_fee = self.icrc_ledger_client.icrc1_fee(self.token1.clone()).await?;

        // Get quote for token swap
        let optimal_quote = swap_service::quote_swap_icrc2_optimal(
            self.provider_impls.clone(),
            self.icrc_ledger_client.clone(),
            self.token0.clone(),
            self.token1.clone(),
            amount.clone(),
        ).await?;

        let quoted_token1_out_for_full_amount = optimal_quote.amount_out;
        let selected_swap_provider = optimal_quote.provider;

        // Calculate pool ratio and swap price for better swap proposition 
        // to make equal amount of token0 and token1 in pool
        let provider_pool_target_ratio =
            nat_to_f64(&provider_suggested_token1_for_pool) 
            / nat_to_f64(&provider_suggested_token0_for_pool); // TODO: Change f64 -> Nat

        let quoted_swap_price_token0_to_token1 =
            (quoted_token1_out_for_full_amount as f64) 
            / (nat_to_f64(&amount) as f64);

        // Calculate how much token_0 and token_1 to swap and add to pool (initial estimate)
        let initial_split = 
            LiquidityCalculator::calculate_token_amounts_for_deposit(
                nat_to_f64(&amount),
                provider_pool_target_ratio.clone(),
                quoted_swap_price_token0_to_token1.clone(),
            );

        let planned_token0_for_swap = initial_split.token_0_for_swap;
        let planned_token0_for_pool = initial_split.token_0_for_pool;

        // Swap token0 for token1 with the best exchange provider
        let swap_response = swap_service::swap_icrc2(
            self.provider_impls.clone(),
            self.icrc_ledger_client.clone(),
            self.token0.clone(),
            self.token1.clone(),
            Nat::from(planned_token0_for_swap as u128),
            selected_swap_provider,
        ).await?;

        // Actual token1 received from swap
        let token1_received_u128 = swap_response.amount_out;

        // Compute balanced pair using integer math based on provider suggested ratio
        // ratio = provider_suggested_token1_for_pool / provider_suggested_token0_for_pool
        let target_ratio_token1_per_token0_num = nat_to_u128(
            &provider_suggested_token1_for_pool
        ); // numerator

        let target_ratio_token1_per_token0_den = nat_to_u128(
            &provider_suggested_token0_for_pool
        ); // denominator

        // Reserve fees so transfers can succeed: reduce amounts to leave fee headroom
        let token0_transfer_fee_u128 = nat_to_u128(&token0_transfer_fee);
        let token1_transfer_fee_u128 = nat_to_u128(&token1_transfer_fee);

        // Variables to store final amounts for pool
        let token0_amount_for_pool_u128: u128;
        let token1_amount_for_pool_u128: u128;

        // Get the target ratio
        let target_ratio_token1_per_token0 = nat_to_f64(&provider_suggested_token1_for_pool) 
            / nat_to_f64(&provider_suggested_token0_for_pool);

        // Calculate how much token1 we can afford with our available token0
        let available_token0_for_pool = planned_token0_for_pool as u128;
        let required_token1_for_pool = (available_token0_for_pool as f64 * target_ratio_token1_per_token0) as u128;

        // Check if we have enough token1
        if required_token1_for_pool > token1_received_u128 {
            // We don't have enough token1, so recalculate both amounts maintaining the ratio
            let final_token1_for_pool = token1_received_u128;
            let final_token0_for_pool = (final_token1_for_pool as f64 / target_ratio_token1_per_token0) as u128;

            // Subtract fees
            token0_amount_for_pool_u128 = final_token0_for_pool.saturating_sub(token0_transfer_fee_u128);
            token1_amount_for_pool_u128 = final_token1_for_pool.saturating_sub(token1_transfer_fee_u128);
        } else {
            // We have enough token1, use the planned amounts
            token0_amount_for_pool_u128 = available_token0_for_pool.saturating_sub(token0_transfer_fee_u128);
            token1_amount_for_pool_u128 = required_token1_for_pool.saturating_sub(token1_transfer_fee_u128);
        }

        // Guard against zero amounts
        if token0_amount_for_pool_u128 == 0 || token1_amount_for_pool_u128 == 0 {
            return Err(InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 4), // Error code: "02-02-02 03 04"
                "KongSwapLiquidityClient::add_liquidity_to_pool".to_string(),
                "Insufficient amounts after swap/fees to add liquidity".to_string(),
                errors::error_extra! {
                    "provider" => PROVIDER,
                    "token0_amount_for_pool_u128" => token0_amount_for_pool_u128,
                    "token1_amount_for_pool_u128" => token1_amount_for_pool_u128,
                },
            ));
        }

        // Add token0 and token1 liquidity to pool using final balanced amounts
        let response = self.kongswap_provider().add_liquidity(
            self.token_kongswap_format(self.token0.clone()),
            Nat::from(token0_amount_for_pool_u128),
            self.token_kongswap_format(self.token1.clone()),
            Nat::from(token1_amount_for_pool_u128),
            self.token0,
            self.token1,
        ).await?;

        // panic!("response: {:?}", response);

        // Compute token0-equivalent total using the ratio we used to finalize the pair
        let token0_equivalent_total = Nat::from(
            token0_amount_for_pool_u128
                .saturating_add(
                    token1_amount_for_pool_u128
                        .saturating_mul(target_ratio_token1_per_token0_den)
                        / target_ratio_token1_per_token0_num
                )
        );

        Ok(AddLiquidityResponse {
            token_0_amount: Nat::from(token0_amount_for_pool_u128),
            token_1_amount: Nat::from(token1_amount_for_pool_u128),
            position_id: response.request_id,
            token0_equivalent_total,
        })
    }

    async fn withdraw_liquidity_from_pool(
        &self,
        total_shares: Nat,
        shares: Nat
    ) -> Result<WithdrawLiquidityResponse, InternalError> {
        let canister_id = ic_cdk::id();

        // Fetch LP positions in pool
        let user_balances_response = self.kongswap_provider()
            .user_balances(canister_id.to_string())
            .await?;

        // Get user balance in pool
        let balance = user_balances_response
            .into_iter()
            .filter_map(|reply| match reply {
                UserBalancesReply::LP(lp) => Some(lp),
                _ => None,
            })
            .find(|balance|
                (
                    balance.address_0 == self.token0.to_text() 
                        && balance.address_1 == self.token1.to_text()
                ) || (
                    balance.address_0 == self.token1.to_text() 
                        && balance.address_1 == self.token0.to_text()
                )
            )
            .map(|balance_reply| balance_reply.balance)
            .ok_or_else(|| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 1), // Error code: "02-02-02 03 01"
                    "KongSwapLiquidityClient::withdraw_liquidity_from_pool".to_string(),
                    "No user LP balance".to_string(),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "token0" => self.token0,
                        "token1" => self.token1,
                        "total_shares" => total_shares,
                        "shares" => shares,
                    },
                )
            })?;

        // Calculate how much LP tokens to withdraw
        let lp_tokens_to_withdraw: f64 = balance
            .mul(nat_to_f64(&shares))
            .div(nat_to_f64(&total_shares))
            .mul(100000000.0);

        // Remove liquidity from pool
        let remove_liquidity_response = self.kongswap_provider()
            .remove_liquidity(
                self.token_kongswap_format(self.token0.clone()),
                self.token_kongswap_format(self.token1.clone()),
                Nat::from(lp_tokens_to_withdraw.round() as u128),
            ).await?;

        Ok(WithdrawLiquidityResponse {
            token_0_amount: remove_liquidity_response.amount_0,
            token_1_amount: remove_liquidity_response.amount_1,
        })
    }

    async fn get_position_by_id(
        &self,
        position_id: u64
    ) -> Result<GetPositionByIdResponse, InternalError> {
        let canister_id = ic_cdk::id();

        // Fetch user positions in pool
        let user_balances_response = self.kongswap_provider().user_balances(
            canister_id.to_string()
        ).await?;

        let user_balance = user_balances_response
            .into_iter()
            .filter_map(|reply| match reply {
                UserBalancesReply::LP(lp) => Some(lp),
                _ => None,
            })
            .find(|balance|
                (balance.address_0 == self.token0.to_text() && balance.address_1 == self.token1.to_text()) ||
                (balance.address_0 == self.token1.to_text() && balance.address_1 == self.token0.to_text())
            )
            .ok_or_else(|| InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 2), // Error code: "02-02-02 03 02"
                "KongSwapLiquidityClient::get_position_by_id".to_string(),
                "No user LP balance".to_string(),
                errors::error_extra! {
                    "provider" => PROVIDER,
                    "token0" => self.token0,
                    "token1" => self.token1,
                    "position_id" => position_id,
                },
            ))?;

        let token0_decimals = self.icrc_ledger_client.icrc1_decimals(self.token0.clone()).await?;
        let token1_decimals = self.icrc_ledger_client.icrc1_decimals(self.token1.clone()).await?;
        let usdt_decimals = self.icrc_ledger_client.icrc1_decimals(*CKUSDT_TOKEN_CANISTER_ID).await?;

        let token0_position_balance = Nat::from(
            (user_balance.amount_0 * 10f64.powi(token0_decimals as i32)).round() as u128
        );
        let token1_position_balance = Nat::from(
            (user_balance.amount_1 * 10f64.powi(token1_decimals as i32)).round() as u128
        );

        let token0_usd_amount = Nat::from(
            (user_balance.usd_amount_0 * 10f64.powi(usdt_decimals as i32)).round() as u128
        );
        let token1_usd_amount = Nat::from(
            (user_balance.usd_amount_1 * 10f64.powi(usdt_decimals as i32)).round() as u128
        );

        Ok(GetPositionByIdResponse {
            position_id: position_id,
            token_0_amount: token0_position_balance,
            token_1_amount: token1_position_balance,
            usd_amount_0: token0_usd_amount,
            usd_amount_1: token1_usd_amount,
        })
    }

    async fn get_pool_data(&self) -> Result<GetPoolDataResponse, InternalError> {
        let pools = self.kongswap_provider().pools().await?;

        let pool_data = pools
            .iter()
            .find(|pool|
                (pool.address_0 == self.token0.to_text() && pool.address_1 == self.token1.to_text()) ||
                (pool.address_0 == self.token1.to_text() && pool.address_1 == self.token0.to_text())
            )
            .ok_or_else(|| InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 3), // Error code: "02-02-02 03 03"
                "KongSwapLiquidityClient::get_pool_data".to_string(),
                "No pool data".to_string(),
                errors::error_extra! {
                    "provider" => PROVIDER,
                    "token0" => self.token0,
                    "token1" => self.token1,
                },
            ))?;

        let token0_balance = pool_data.balance_0.clone() + pool_data.lp_fee_0.clone();
        let token1_balance = pool_data.balance_1.clone() + pool_data.lp_fee_1.clone();

        let decimals_token0 = self.icrc_ledger_client.icrc1_decimals(self.token0.clone()).await?;
        let decimals_token1 = self.icrc_ledger_client.icrc1_decimals(self.token1.clone()).await?;
        let decimals_usdt = self.icrc_ledger_client.icrc1_decimals(*CKUSDT_TOKEN_CANISTER_ID).await?;

        let token0_base_unit = Nat::from(10u32.pow(decimals_token0 as u32)); // 10^decimals_token0
        let token1_base_unit = Nat::from(10u32.pow(decimals_token1 as u32)); // 10^decimals_token1
        let usdt_base_unit = Nat::from(10u32.pow(decimals_usdt as u32)); // 10^decimals_usdt

        // Multiply by multiplier to get more accurate result in TVL calculation
        let multiplier = Nat::from(1000u128);
        let token0_base_unit_multiplied = token0_base_unit.clone().mul(multiplier.clone());
        let token1_base_unit_multiplied = token1_base_unit.clone().mul(multiplier.clone());

        // Get quote for token0 swap to USDT
        let swap_amount0_reply = self.kongswap_provider().swap_amounts(
            self.token0.clone(),
            token0_base_unit_multiplied.clone(),
            *CKUSDT_TOKEN_CANISTER_ID
        ).await?;

        // Get quote for token1 swap to USDT
        let swap_amount1_reply = self.kongswap_provider().swap_amounts(
            self.token1,
            token1_base_unit_multiplied.clone(),
            *CKUSDT_TOKEN_CANISTER_ID
        ).await?;

        let token0_usdt_price = swap_amount0_reply.receive_amount.div(multiplier.clone());
        let token1_usdt_price = swap_amount1_reply.receive_amount.div(multiplier);

        let token0_usdt_balance = token0_balance
            .mul(token0_usdt_price.clone())
            .div(token0_base_unit)
            .div(usdt_base_unit.clone());

        let token1_usdt_balance = token1_balance
            .mul(token1_usdt_price.clone())
            .div(token1_base_unit)
            .div(usdt_base_unit);

        let tvl = token0_usdt_balance.clone() + token1_usdt_balance.clone();

        Ok(GetPoolDataResponse {
            tvl: tvl,
        })
    }
}

