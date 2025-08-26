use async_trait::async_trait;
use types::CanisterId;
use candid::Nat;
use std::sync::Arc;

use super::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};
use providers::kongswap::KongSwapProvider;
use utils::util::nat_to_u128;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    libraries as library_area,
    libraries::domains::swap as swap_domain,
    libraries::domains::swap::components as swap_domain_components,
};

const SLIPPAGE_PERCENTAGE: f64 = 40.0; // TODO: Improve slippage settings

// Module code: "02-01-02"
errors::define_error_code_builder_fn!(
    build_error_code,
    library_area::AREA_CODE,          // Area code: "02"
    swap_domain::DOMAIN_CODE,         // Domain code: "01"
    swap_domain_components::KONG_SWAP // Component code: "02"
);

pub struct KongSwapSwapClient {
    provider_impl: Arc<dyn KongSwapProvider + Send + Sync>,
    canister_id: CanisterId,
    token_in: CanisterId,
    token_out: CanisterId,
}

impl KongSwapSwapClient {
    pub fn new(
        provider_impl: Arc<dyn KongSwapProvider + Send + Sync>,
        canister_id: CanisterId,
        token_in: CanisterId,
        token_out: CanisterId,
    ) -> Self {
        Self {
            provider_impl,
            canister_id,
            token_in,
            token_out,
        }
    }

    /// Checks and fixes the token order for KongSwap
    async fn check_and_fix_token_order(&self) -> Result<(CanisterId, CanisterId), InternalError> {
        let pools = self.provider_impl.pools().await?;
        
        // Find the pool with our tokens
        let pool = pools.iter().find(|pool| {
            let pool_token0 = pool.address_0.clone();
            let pool_token1 = pool.address_1.clone();
            
            let token_in_str = self.token_in.to_text();
            let token_out_str = self.token_out.to_text();
            
            // Check both token order variants
            (pool_token0 == token_in_str && pool_token1 == token_out_str) ||
            (pool_token0 == token_out_str && pool_token1 == token_in_str)
        }).ok_or_else(|| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 1), // Error code: "02-01-02 03 01"
                "KongSwapSwapClient::check_and_fix_token_order".to_string(),
                "Pool not found for the specified tokens".to_string(),
                errors::error_extra! {
                    "token_in" => self.token_in,
                    "token_out" => self.token_out,
                },
            )
        })?;

        let pool_token0 = pool.address_0.clone();
        let pool_token1 = pool.address_1.clone();
        let token_in_str = self.token_in.to_text();
        let token_out_str = self.token_out.to_text();

        // Determine the correct token order
        if pool_token0 == token_in_str && pool_token1 == token_out_str {
            // Tokens are already in the correct order
            Ok((self.token_in.clone(), self.token_out.clone()))
        } else if pool_token0 == token_out_str && pool_token1 == token_in_str {
            // Need to change the token order
            Ok((self.token_out.clone(), self.token_in.clone()))
        } else {
            Err(InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 2), // Error code: "02-01-02 03 02"
                "KongSwapSwapClient::check_and_fix_token_order".to_string(),
                "Invalid token configuration for KongSwap pool".to_string(),
                errors::error_extra! {
                    "token_in" => self.token_in,
                    "token_out" => self.token_out,
                    "pool_token0" => pool_token0,
                    "pool_token1" => pool_token1,
                },
            ))
        }
    }
}

#[async_trait]
impl SwapClient for KongSwapSwapClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn swap(&self, amount: Nat) -> Result<SwapSuccess, InternalError> {
        // Check and fix the token order
        let (corrected_token_in, corrected_token_out) = 
            self.check_and_fix_token_order().await?;
        
        let result = self.provider_impl.swap(
            corrected_token_in,
            amount.clone(),
            corrected_token_out,
            Some(SLIPPAGE_PERCENTAGE),
        ).await?;

        Ok(SwapSuccess {
            amount_out: nat_to_u128(&result.receive_amount),
            withdrawal_success: Some(result.claim_ids.is_empty()),
        })
    }

    async fn quote(&self, amount: Nat) -> Result<QuoteSuccess, InternalError> {
        // Check and fix the token order
        let (corrected_token_in, corrected_token_out) = 
            self.check_and_fix_token_order().await?;
        
        let result = self.provider_impl.swap_amounts(
            corrected_token_in,
            amount.clone(),
            corrected_token_out,
        ).await?;

        Ok(QuoteSuccess {
            amount_out: nat_to_u128(&result.receive_amount),
        })
    }
}
