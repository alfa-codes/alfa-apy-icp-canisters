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
}

#[async_trait]
impl SwapClient for KongSwapSwapClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn swap(&self, amount: Nat) -> Result<SwapSuccess, InternalError> {
        let result = self.provider_impl.swap(
            self.token_in,
            amount.clone(),
            self.token_out,
            Some(SLIPPAGE_PERCENTAGE),
        ).await?;

        Ok(SwapSuccess {
            amount_out: nat_to_u128(&result.receive_amount),
            withdrawal_success: Some(result.claim_ids.is_empty()),
        })
    }

    async fn quote(&self, amount: Nat) -> Result<QuoteSuccess, InternalError> {
        let result = self.provider_impl.swap_amounts(
            self.token_in,
            amount.clone(),
            self.token_out,
        ).await?;

        Ok(QuoteSuccess {
            amount_out: nat_to_u128(&result.receive_amount),
        })
    }
}