use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use candid::Nat;
use std::sync::Arc;

use types::CanisterId;
use providers::icpswap::ICPSwapProvider;
use icpswap_swap_factory_canister::ICPSwapPool;
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;
use types::liquidity::TokensFee;
use utils::util::nat_to_u128;
use icrc_ledger_client::ICRCLedgerClient;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    libraries as library_area,
    libraries::domains::swap as swap_domain,
    libraries::domains::swap::components as swap_domain_components,
};

use crate::token_swaps::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};

pub const SLIPPAGE_TOLERANCE_POINTS: u128 = 50; // 50 slippage tolerance points == 5%

// Module code: "02-01-03"
errors::define_error_code_builder_fn!(
    build_error_code,
    library_area::AREA_CODE,         // Area code: "02"
    swap_domain::DOMAIN_CODE,        // Domain code: "01"
    swap_domain_components::ICP_SWAP // Component code: "03"
);

pub struct ICPSwapSwapClient {
    provider_impl: Arc<dyn ICPSwapProvider + Send + Sync>,
    icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
    canister_id: Option<CanisterId>,
    token0: CanisterId,
    token1: CanisterId,
    pool: Option<ICPSwapPool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepositFromSuccess {
    pub deposited_amount: u128,
}

impl ICPSwapSwapClient {
    pub fn new(
        provider_impl: Arc<dyn ICPSwapProvider + Send + Sync>,
        icrc_ledger_client: Arc<dyn ICRCLedgerClient>,
        token0: CanisterId,
        token1: CanisterId,
    ) -> Self {
        Self {
            provider_impl,
            icrc_ledger_client,
            canister_id: None,
            token0, // token0 may be token1 in the pool and vice versa
            token1, // token1 may be token0 in the pool and vice versa
            pool: None,
        }
    }

    pub async fn with_pool(mut self) -> Result<Self, InternalError> {
        let pool = self.provider_impl.get_pool(self.token0.clone(), self.token1.clone()).await?;

        self.pool = Some(pool.clone());
        self.canister_id = Some(pool.canisterId);

        Ok(self)
    }

    fn is_zero_for_one_swap_direction(&self) -> Result<bool, InternalError> {
        let token0_str = self.token0.to_text();
        let token1_str = self.token1.to_text();

        let pool = self.pool.as_ref().unwrap();

        match (pool.token0.address.as_str(), pool.token1.address.as_str()) {
            (t0, t1) if t0 == token0_str && t1 == token1_str => Ok(true),
            (t0, t1) if t0 == token1_str && t1 == token0_str => Ok(false),
            (t0, t1) => Err(InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 1), // Error code: "02-01-03 03 01"
                "ICPSwapSwapClient::is_zero_for_one_swap_direction".to_string(),
                "Invalid token configuration for ICPSwap pool".to_string(),
                errors::error_extra! {
                    "token0" => self.token0,
                    "token1" => self.token1,
                    "t0" => t0,
                    "t1" => t1,
                },
            )),
        }
    }

    fn get_tokens_fee(&self, token_meta: &TokenMeta) -> Result<TokensFee, InternalError> {
        let token0_str = self.token0.to_text();
        let token1_str = self.token1.to_text();

        let pool = self.pool.as_ref().unwrap();

        match (pool.token0.address.as_str(), pool.token1.address.as_str()) {
            (t0, t1) if t0 == token0_str && t1 == token1_str => Ok(TokensFee {
                token0_fee: token_meta.token0Fee.clone(),
                token1_fee: token_meta.token1Fee.clone(),
            }),
            (t0, t1) if t0 == token1_str && t1 == token0_str => Ok(TokensFee {
                token0_fee: token_meta.token1Fee.clone(),
                token1_fee: token_meta.token0Fee.clone(),
            }),
            (t0, t1) => Err(InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 2), // Error code: "02-01-03 03 02"
                "ICPSwapSwapClient::get_tokens_fee".to_string(),
                "Invalid token configuration for ICPSwap pool".to_string(),
                errors::error_extra! {
                    "token0" => self.token0,
                    "token1" => self.token1,
                    "t0" => t0,
                    "t1" => t1,
                },
            )),
        }
    }

    async fn get_pool(&self, token0: CanisterId, token1: CanisterId) -> Result<ICPSwapPool, InternalError> {
        self.provider_impl.get_pool(token0.clone(), token1.clone()).await
    }

    async fn get_token_meta(&self) -> Result<TokenMeta, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        self.provider_impl.get_token_meta(canister_id.clone()).await
    }
    
    async fn deposit_from(&self, amount: Nat, token_fee: Nat) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        self.provider_impl.deposit_from(
            canister_id.clone(),
            self.token0.clone(),
            amount.clone(),
            token_fee.clone()
        ).await
    }

    async fn withdraw(&self, amount: Nat, token_fee: Nat) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        self.provider_impl.withdraw(
            canister_id.clone(),
            self.token1.clone(),
            amount.clone(),
            token_fee.clone()
        ).await
    }

    async fn quote_internal(&self, amount: Nat) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();
        let is_zero_for_one_swap_direction = self.is_zero_for_one_swap_direction()?;

        self.provider_impl.quote(
            canister_id.clone(),
            amount.clone(),
            is_zero_for_one_swap_direction,
            amount.clone()
        ).await
    }

    async fn swap_internal(&self, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        self.provider_impl.swap(
            canister_id.clone(),
            amount_in.clone(),
            zero_for_one,
            amount_out_minimum.clone()
        ).await
    }
}

#[async_trait]
impl SwapClient for ICPSwapSwapClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id.as_ref().unwrap().clone()
    }

    async fn swap(&self, amount: Nat) -> Result<SwapSuccess, InternalError> {
        // Flow:
        // 1. Get token fees
        // 2. Deposit from token0 to ICPSwap
        // 3. Quote
        // 4. Swap
        // 5. Withdraw from ICPSwap to token1

        // 1. Get token fees
        let token0_fee = self.icrc_ledger_client.icrc1_fee(self.token0.clone()).await?;
        let token1_fee = self.icrc_ledger_client.icrc1_fee(self.token1.clone()).await?;

        // 2. Deposit
        let deposited_amount = self.deposit_from(
            amount.clone(),
            token0_fee.clone()
        ).await?;

        // 3. Quote
        let expected_out = self.quote_internal(deposited_amount.clone()).await?;

        // 4. Swap
        let expected_out_u128 = nat_to_u128(&expected_out);
        // Сonsider slippage tolerance
        let amount_out_minimum = Nat::from(
            expected_out_u128 * (1000 - SLIPPAGE_TOLERANCE_POINTS) / 1000u128
        );

        let amount_out = self.swap_internal(
            deposited_amount.clone(),
            self.is_zero_for_one_swap_direction()?,
            amount_out_minimum.clone(),
        ).await?;

        // 5. Withdraw
        let withdrawn_amount = self.withdraw(amount_out, token1_fee.clone()).await?;

        Ok(SwapSuccess {
            amount_out: nat_to_u128(&withdrawn_amount),
            withdrawal_success: Some(true),
        })
    }

    async fn quote(&self, amount: Nat) -> Result<QuoteSuccess, InternalError> {
        let quote_amount = self.quote_internal(amount.clone()).await?;

        Ok(QuoteSuccess {
            amount_out: nat_to_u128(&quote_amount),
        })
    }
}
