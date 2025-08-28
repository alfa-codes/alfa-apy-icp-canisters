use candid::{Nat, Principal, CandidType};
use types::CanisterId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use kongswap_canister::add_liquidity::{Args as AddLiquidityArgs, AddLiquidityReply};
use kongswap_canister::remove_liquidity::{Args as RemoveLiquidityArgs, RemoveLiquidityReply};
use kongswap_canister::remove_liquidity_amounts::{Args as RemoveLiquidityAmountsArgs, RemoveLiquidityAmountsReply};
use kongswap_canister::queries::pools::PoolReply;
use kongswap_canister::queries::add_liquidity_amounts::AddLiquidityAmountsReply;
use kongswap_canister::swap_amounts::SwapAmountsReply;
use kongswap_canister::user_balances::UserBalancesReply;
use kongswap_canister::swap::SwapReply;
use kongswap_canister::swap::Args as SwapArgs;
use icrc_ledger_client::{DefaultICRCLedgerClient, ICRCLedgerClient};
use utils::constants::KONGSWAP_CANISTER_ID;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    external_services as external_services_area,
    external_services::domains::kong_swap as kong_swap_domain,
    external_services::domains::kong_swap::components as kong_swap_domain_components,
};

// Module code: "01-01-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    external_services_area::AREA_CODE,  // Area code: "01"
    kong_swap_domain::DOMAIN_CODE,       // Domain code: "01"   
    kong_swap_domain_components::CORE    // Component code: "01"
);

#[async_trait::async_trait]
pub trait KongSwapProvider: Send + Sync + 'static {
    async fn pools(&self) -> Result<Vec<PoolReply>, InternalError>;
    async fn swap_amounts(
        &self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId
    ) -> Result<SwapAmountsReply, InternalError>;
    async fn swap(
        &self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
        max_slippage: Option<f64>
    ) -> Result<SwapReply, InternalError>;
    async fn add_liquidity_amounts(
        &self,
        token_0: String,
        amount: Nat,
        token_1: String
    ) -> Result<AddLiquidityAmountsReply, InternalError>;
    async fn add_liquidity(
        &self,
        token_0: String,
        amount_0: Nat,
        token_1: String,
        amount_1: Nat,
        ledger0: Principal,
        ledger1: Principal
    ) -> Result<AddLiquidityReply, InternalError>;
    async fn user_balances(
        &self,
        principal_id: String
    ) -> Result<Vec<UserBalancesReply>, InternalError>;
    async fn remove_liquidity_amounts(
        &self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat
    ) -> Result<RemoveLiquidityAmountsReply, InternalError>;
    async fn remove_liquidity(
        &self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat
    ) -> Result<RemoveLiquidityReply, InternalError>;
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct DefaultKongSwapProvider;

impl DefaultKongSwapProvider {
    fn format_to_kongswap_token(token: &CanisterId) -> String {
        format!("IC.{}", token.to_text())
    }

    fn icrc_ledger_client(&self) -> Arc<dyn ICRCLedgerClient> {
        Arc::new(DefaultICRCLedgerClient)
    }
}

#[async_trait::async_trait]
impl KongSwapProvider for DefaultKongSwapProvider {
    async fn pools(&self) -> Result<Vec<PoolReply>, InternalError> {
        kongswap_canister_c2c_client::pools(*KONGSWAP_CANISTER_ID).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 1), // Error code: "01-01-01 04 01"
                    "KongSwapProvider::pools".to_string(),
                    format!("IC error calling 'kongswap_canister_c2c_client::pools': {error:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                    }
                )
            })?
            .map_err(|error_message| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 2), // Error code: "01-01-01 03 02"
                    "KongSwapProvider::pools".to_string(),
                    format!("Error calling 'kongswap_canister_c2c_client::pools': {error_message:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                    }
                )
            })
    }

    async fn swap_amounts(
        &self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
    ) -> Result<SwapAmountsReply, InternalError> {
        let token_in = Self::format_to_kongswap_token(&token_in.clone());
        let token_out = Self::format_to_kongswap_token(&token_out.clone());

        let (result,) = kongswap_canister_c2c_client::swap_amounts(
            *KONGSWAP_CANISTER_ID,
            (token_in.clone(), amount.clone(), token_out.clone())
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 3), // Error code: "01-01-01 04 03"
                    "KongSwapProvider::swap_amounts".to_string(),
                    format!("IC error calling 'kongswap_canister_c2c_client::swap_amounts': {error:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                        "token_in" => token_in.clone(),
                        "token_out" => token_out.clone(),
                        "amount" => amount,
                    }
                )
            })?;

        result.map_err(|error_message| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 4), // Error code: "01-01-01 03 04"
                "KongSwapProvider::swap_amounts".to_string(),
                format!("Error calling 'kongswap_canister_c2c_client::swap_amounts': {error_message:?}"),
                errors::error_extra! {
                    "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                    "token_in" => token_in,
                    "token_out" => token_out,
                    "amount" => amount,
                }
            )
        })
    }

    async fn swap(
        &self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
        max_slippage: Option<f64>
    ) -> Result<SwapReply, InternalError> {
        let args = SwapArgs {
            pay_amount: amount.into(),
            pay_token: Self::format_to_kongswap_token(&token_in.clone()),
            receive_token: Self::format_to_kongswap_token(&token_out.clone()),
            max_slippage,
        };

        let result = kongswap_canister_c2c_client::swap(
            *KONGSWAP_CANISTER_ID,
            &args
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 5), // Error code: "01-01-01 04 05"
                    "KongSwapProvider::swap".to_string(),
                    format!("Error calling 'kongswap_canister_c2c_client::swap': {error:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                        "pay_amount" => args.pay_amount,
                        "pay_token" => args.pay_token,
                        "receive_token" => args.receive_token,
                        "max_slippage" => args.max_slippage.unwrap_or(0.0),
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 6), // Error code: "01-01-01 03 06"
                    "KongSwapProvider::swap".to_string(),
                    format!("Error calling 'kongswap_canister_c2c_client::swap': {error:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                        "pay_amount" => args.pay_amount,
                        "pay_token" => args.pay_token,
                        "receive_token" => args.receive_token,
                        "max_slippage" => args.max_slippage.unwrap_or(0.0),
                    }
                )
            })?;

        Ok(result)
    }

    async fn add_liquidity_amounts(
        &self,
        token_0: String,
        amount: Nat,
        token_1: String,
    ) -> Result<AddLiquidityAmountsReply, InternalError> {
        let (result,) = kongswap_canister_c2c_client::add_liquidity_amounts(
            *KONGSWAP_CANISTER_ID,
            (token_0.clone(), amount.clone(), token_1.clone())
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 7), // Error code: "01-01-01 04 07"
                    "KongSwapProvider::add_liquidity_amounts".to_string(),
                    format!("IC error calling 'kongswap_canister_c2c_client::add_liquidity_amounts': {error:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                        "token0" => token_0.clone(),
                        "amount" => amount,
                        "token1" => token_1.clone(),
                    }
                )
            })?;

        result.map_err(|error_message| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 8), // Error code: "01-01-01 03 08"
                "KongSwapProvider::add_liquidity_amounts".to_string(),
                format!("Error calling 'kongswap_canister_c2c_client::add_liquidity_amounts': {error_message:?}"),
                errors::error_extra! {
                    "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                    "token0" => token_0,
                    "amount" => amount,
                    "token1" => token_1,
                }
            )
        })
    }

    async fn add_liquidity(
        &self,
        token_0: String,
        amount_0: Nat,
        token_1: String,
        amount_1: Nat,
        ledger0: Principal,
        ledger1: Principal
    ) -> Result<AddLiquidityReply, InternalError> {
        self.icrc_ledger_client().icrc2_approve(
            KONGSWAP_CANISTER_ID.clone().into(),
            ledger0,
            amount_0.clone()
        ).await?;

        self.icrc_ledger_client().icrc2_approve(
            KONGSWAP_CANISTER_ID.clone().into(),
            ledger1,
            amount_1.clone()
        ).await?;

        let args = AddLiquidityArgs {
            token_0: token_0.clone(),
            amount_0: amount_0.clone(),
            tx_id_0: None, //use icrc2
            token_1: token_1.clone(),
            amount_1: amount_1.clone(),
            tx_id_1: None,
        };

        let result = kongswap_canister_c2c_client::add_liquidity(
            *KONGSWAP_CANISTER_ID,
            &args
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 9), // Error code: "01-01-01 04 09"
                    "KongSwapProvider::add_liquidity".to_string(),
                    format!("IC error calling 'kongswap_canister_c2c_client::add_liquidity': {error:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                        "token_0" => token_0.clone(),
                        "amount_0" => amount_0,
                        "token_1" => token_1.clone(),
                        "amount_1" => amount_1,
                        "ledger0" => ledger0.to_text(),
                        "ledger1" => ledger1.to_text(),
                    }
                )
            })?;

        result.map_err(|error_message| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 10), // Error code: "01-01-01 03 10"
                "KongSwapProvider::add_liquidity".to_string(),
                format!("Error calling 'kongswap_canister_c2c_client::add_liquidity': {error_message:?}"),
                errors::error_extra! {
                    "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                    "token_0" => token_0,
                    "amount_0" => amount_0,
                    "token_1" => token_1,
                    "amount_1" => amount_1,
                    "ledger0" => ledger0.to_text(),
                    "ledger1" => ledger1.to_text(),
                }
            )
        })
    }

    async fn user_balances(
        &self,
        principal_id: String,
    ) -> Result<Vec<UserBalancesReply>, InternalError> {
        let (result,) = kongswap_canister_c2c_client::user_balances(
            *KONGSWAP_CANISTER_ID,
            (principal_id.clone(),)
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 11), // Error code: "01-01-01 04 11"
                    "KongSwapProvider::user_balances".to_string(),
                    format!("IC error calling 'kongswap_canister_c2c_client::user_balances': {error:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                        "principal_id" => principal_id.clone(),
                    }
                )
            })?;

        result.map_err(|error_message| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 12), // Error code: "01-01-01 03 12"
                "KongSwapProvider::user_balances".to_string(),
                format!("Error calling 'kongswap_canister_c2c_client::user_balances': {error_message:?}"),
                errors::error_extra! {
                    "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                    "principal_id" => principal_id,
                }
            )
        })
    }

    async fn remove_liquidity_amounts(
        &self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat
    ) -> Result<RemoveLiquidityAmountsReply, InternalError> {
        let args = RemoveLiquidityAmountsArgs {
            token_0: token_0.clone(),
            token_1: token_1.clone(),
            remove_lp_token_amount: remove_lp_token_amount.clone(),
        };

        let result = kongswap_canister_c2c_client::remove_liquidity_amounts(
            *KONGSWAP_CANISTER_ID,
            &args
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 13), // Error code: "01-01-01 04 13"
                    "KongSwapProvider::remove_liquidity_amounts".to_string(),
                    format!("IC error calling 'kongswap_canister_c2c_client::remove_liquidity_amounts': {error:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                        "token_0" => token_0.clone(),
                        "token_1" => token_1.clone(),
                        "remove_lp_token_amount" => remove_lp_token_amount,
                    }
                )
            })?;

        result.map_err(|error_message| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 14), // Error code: "01-01-01 03 14"
                "KongSwapProvider::remove_liquidity_amounts".to_string(),
                format!("Error calling 'kongswap_canister_c2c_client::remove_liquidity_amounts': {error_message:?}"),
                errors::error_extra! {
                    "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                    "token_0" => token_0,
                    "token_1" => token_1,
                    "remove_lp_token_amount" => remove_lp_token_amount,
                }
            )
        })
    }

    async fn remove_liquidity(
        &self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat
    ) -> Result<RemoveLiquidityReply, InternalError> {
        let args = RemoveLiquidityArgs {
            token_0: token_0.clone(),
            token_1: token_1.clone(),
            remove_lp_token_amount: remove_lp_token_amount.clone(),
        };

        let result = kongswap_canister_c2c_client::remove_liquidity(
            *KONGSWAP_CANISTER_ID,
            &args
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 15), // Error code: "01-01-01 04 15"
                    "KongSwapProvider::remove_liquidity".to_string(),
                    format!("IC error calling 'kongswap_canister_c2c_client::remove_liquidity': {error:?}"),
                    errors::error_extra! {
                        "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                        "token_0" => token_0.clone(),
                        "token_1" => token_1.clone(),
                        "remove_lp_token_amount" => remove_lp_token_amount,
                    }
                )
            })?;

        result.map_err(|error_message| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 16), // Error code: "01-01-01 03 16"
                "KongSwapProvider::remove_liquidity".to_string(),
                format!("Error calling 'kongswap_canister_c2c_client::remove_liquidity': {error_message:?}"),
                errors::error_extra! {
                    "kongswap_canister_id" => KONGSWAP_CANISTER_ID.to_text(),
                    "token_0" => token_0,
                    "token_1" => token_1,
                    "remove_lp_token_amount" => remove_lp_token_amount,
                }
            )
        })
    }
}
