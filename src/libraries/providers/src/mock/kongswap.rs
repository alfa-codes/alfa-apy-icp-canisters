use std::collections::HashMap;
use candid::{Nat, Principal, CandidType};
use types::CanisterId;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use kongswap_canister::add_liquidity::AddLiquidityReply;
use kongswap_canister::remove_liquidity::RemoveLiquidityReply;
use kongswap_canister::remove_liquidity_amounts::RemoveLiquidityAmountsReply;
use kongswap_canister::queries::pools::PoolReply;
use kongswap_canister::queries::add_liquidity_amounts::AddLiquidityAmountsReply;
use kongswap_canister::swap_amounts::SwapAmountsReply;
use kongswap_canister::user_balances::UserBalancesReply;
use kongswap_canister::swap::SwapReply;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    libraries as library_area,
    libraries::domains::provider as provider_domain,
    libraries::domains::provider::components as provider_domain_components,
};

use crate::kongswap::KongSwapProvider;

// Module code: "02-04-51"
errors::define_error_code_builder_fn!(
    build_error_code,
    library_area::AREA_CODE,                   // Area code: "02"
    provider_domain::DOMAIN_CODE,              // Domain code: "04"
    provider_domain_components::MOCK_KONG_SWAP // Component code: "51"
);

// Converts Option<f64> to String for use as a HashMap key since f64 doesn't implement Hash and Eq traits.
// We need this because floating point numbers can't be used directly as HashMap keys.
// The precision is fixed at 8 decimal places to ensure consistent string representation.
// Returns "none" for None values to maintain a unique string representation.
fn slippage_to_string(slippage: Option<f64>) -> String {
    slippage.map_or("none".to_string(), |v| format!("{:.8}", v))
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct MockKongSwapProvider {
    pub pools_response: Result<Vec<PoolReply>, InternalError>,
    pub swap_amounts_responses: HashMap<(String, String, String), Result<SwapAmountsReply, InternalError>>,
    pub swap_responses: HashMap<(String, String, String, String), Result<SwapReply, InternalError>>,
    pub add_liquidity_amounts_responses: HashMap<(String, String, String), Result<AddLiquidityAmountsReply, InternalError>>,
    pub add_liquidity_responses: HashMap<(String, String, String, String, String, String), Result<AddLiquidityReply, InternalError>>,
    pub user_balances_responses: HashMap<String, Result<Vec<UserBalancesReply>, InternalError>>,
    pub remove_liquidity_amounts_responses: HashMap<(String, String, String), Result<RemoveLiquidityAmountsReply, InternalError>>,
    pub remove_liquidity_responses: HashMap<(String, String, String), Result<RemoveLiquidityReply, InternalError>>,
}

impl Default for MockKongSwapProvider {
    fn default() -> Self {
        Self {
            pools_response: Err(InternalError::not_found(
                build_error_code(InternalErrorKind::NotFound, 8), // Error code: "02-04-51 01 08"
                "mock_error".to_string(),
                "Mock response not set for pools".to_string(),
                None
            )),
            swap_amounts_responses: HashMap::new(),
            swap_responses: HashMap::new(),
            add_liquidity_amounts_responses: HashMap::new(),
            add_liquidity_responses: HashMap::new(),
            user_balances_responses: HashMap::new(),
            remove_liquidity_amounts_responses: HashMap::new(),
            remove_liquidity_responses: HashMap::new(),
        }
    }
}

impl MockKongSwapProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mock_pools(&mut self, response: Result<Vec<PoolReply>, InternalError>) {
        self.pools_response = response;
    }

    pub fn mock_swap_amounts(
        &mut self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
        response: Result<SwapAmountsReply, InternalError>,
    ) {
        self.swap_amounts_responses.insert(
            (token_in.to_text(), amount.to_string(), token_out.to_text()),
            response
        );
    }

    pub fn mock_swap(
        &mut self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
        max_slippage: Option<f64>,
        response: Result<SwapReply, InternalError>,
    ) {
        self.swap_responses.insert(
            (
                token_in.to_text(),
                amount.to_string(),
                token_out.to_text(),
                slippage_to_string(max_slippage)
            ),
            response
        );
    }

    pub fn mock_add_liquidity_amounts(
        &mut self,
        token_0: String,
        amount: Nat,
        token_1: String,
        response: Result<AddLiquidityAmountsReply, InternalError>,
    ) {
        self.add_liquidity_amounts_responses.insert(
            (token_0, amount.to_string(), token_1),
            response
        );
    }

    pub fn mock_add_liquidity(
        &mut self,
        token_0: String,
        amount_0: Nat,
        token_1: String,
        amount_1: Nat,
        ledger0: Principal,
        ledger1: Principal,
        response: Result<AddLiquidityReply, InternalError>,
    ) {
        self.add_liquidity_responses.insert(
            (
                token_0,
                amount_0.to_string(),
                token_1,
                amount_1.to_string(),
                ledger0.to_text(),
                ledger1.to_text()
            ),
            response
        );
    }

    pub fn mock_user_balances(
        &mut self,
        principal_id: String,
        response: Result<Vec<UserBalancesReply>, InternalError>,
    ) {
        self.user_balances_responses.insert(principal_id, response);
    }

    pub fn mock_remove_liquidity_amounts(
        &mut self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat,
        response: Result<RemoveLiquidityAmountsReply, InternalError>,
    ) {
        self.remove_liquidity_amounts_responses.insert(
            (token_0, token_1, remove_lp_token_amount.to_string()),
            response
        );
    }

    pub fn mock_remove_liquidity(
        &mut self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat,
        response: Result<RemoveLiquidityReply, InternalError>,
    ) {
        self.remove_liquidity_responses.insert(
            (token_0, token_1, remove_lp_token_amount.to_string()),
            response
        );
    }
}

#[async_trait]
impl KongSwapProvider for MockKongSwapProvider {
    async fn pools(&self) -> Result<Vec<PoolReply>, InternalError> {
        self.pools_response.clone()
    }

    async fn swap_amounts(
        &self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
    ) -> Result<SwapAmountsReply, InternalError> {
        self.swap_amounts_responses
            .get(&(token_in.to_text(), amount.to_string(), token_out.to_text()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 1), // Error code: "02-04-51 01 01"
                    "MockKongSwapProvider::swap_amounts".to_string(),
                    "Mock response not set for swap_amounts".to_string(),
                    errors::error_extra! {
                        "token_in" => token_in,
                        "amount" => amount,
                        "token_out" => token_out,
                    }
                )),
                |r| r.to_owned()
            )
    }

    async fn swap(
        &self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
        max_slippage: Option<f64>,
    ) -> Result<SwapReply, InternalError> {
        let key = (
            token_in.to_text(),
            amount.to_string(),
            token_out.to_text(),
            slippage_to_string(max_slippage)
        );

        self.swap_responses
            .get(&key)
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 2), // Error code: "02-04-51 01 02"
                    "MockKongSwapProvider::swap".to_string(),
                    "Mock response not set for swap".to_string(),
                    errors::error_extra! {
                        "token_in" => token_in,
                        "amount" => amount,
                        "token_out" => token_out,
                        "max_slippage" => slippage_to_string(max_slippage),
                    }
                )),
                |r| r.to_owned()
            )
    }

    async fn add_liquidity_amounts(
        &self,
        token_0: String,
        amount: Nat,
        token_1: String,
    ) -> Result<AddLiquidityAmountsReply, InternalError> {
        self.add_liquidity_amounts_responses
            .get(&(token_0.clone(), amount.to_string(), token_1.clone()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 3), // Error code: "02-04-51 01 03"
                    "MockKongSwapProvider::add_liquidity_amounts".to_string(),
                    "Mock response not set for add_liquidity_amounts".to_string(),
                    errors::error_extra! {
                        "token_0" => token_0,
                        "amount" => amount,
                        "token_1" => token_1,
                    }
                )),
                |r| r.to_owned()
            )
    }

    async fn add_liquidity(
        &self,
        token_0: String,
        amount_0: Nat,
        token_1: String,
        amount_1: Nat,
        ledger0: Principal,
        ledger1: Principal,
    ) -> Result<AddLiquidityReply, InternalError> {
        self.add_liquidity_responses
            .get(&(
                token_0.clone(),
                amount_0.to_string(),
                token_1.clone(),
                amount_1.to_string(),
                ledger0.to_text(),
                ledger1.to_text())
            )
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 4), // Error code: "02-04-51 01 04"
                    "MockKongSwapProvider::add_liquidity".to_string(),
                    "Mock response not set for add_liquidity".to_string(),
                    errors::error_extra! {
                        "token_0" => token_0,
                        "amount_0" => amount_0,
                        "token_1" => token_1,
                        "amount_1" => amount_1,
                        "ledger0" => ledger0,
                        "ledger1" => ledger1,
                    }
                )),
                |r| r.to_owned()
            )
    }

    async fn user_balances(
        &self,
        principal_id: String,
    ) -> Result<Vec<UserBalancesReply>, InternalError> {
        self.user_balances_responses
            .get(&principal_id)
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 5), // Error code: "02-04-51 01 05"
                    "MockKongSwapProvider::user_balances".to_string(),
                    "Mock response not set for user_balances".to_string(),
                    errors::error_extra! {
                        "principal_id" => principal_id,
                    }
                )),
                |r| r.to_owned()
            )
    }

    async fn remove_liquidity_amounts(
        &self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat,
    ) -> Result<RemoveLiquidityAmountsReply, InternalError> {
        self.remove_liquidity_amounts_responses
            .get(&(token_0.clone(), token_1.clone(), remove_lp_token_amount.to_string()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 6), // Error code: "02-04-51 01 06"
                    "MockKongSwapProvider::remove_liquidity_amounts".to_string(),
                    "Mock response not set for remove_liquidity_amounts".to_string(),
                    errors::error_extra! {
                        "token_0" => token_0,
                        "token_1" => token_1,
                        "remove_lp_token_amount" => remove_lp_token_amount,
                    }
                )),
                |r| r.to_owned()
            )
    }

    async fn remove_liquidity(
        &self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat,
    ) -> Result<RemoveLiquidityReply, InternalError> {
        self.remove_liquidity_responses
            .get(&(token_0.clone(), token_1.clone(), remove_lp_token_amount.to_string()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(InternalErrorKind::NotFound, 7), // Error code: "02-04-51 01 07"
                    "MockKongSwapProvider::remove_liquidity".to_string(),
                    "Mock response not set for remove_liquidity".to_string(),
                    errors::error_extra! {
                        "token_0" => token_0,
                        "token_1" => token_1,
                        "remove_lp_token_amount" => remove_lp_token_amount,
                    }
                )),
                |r| r.to_owned()
            )
    }
}
