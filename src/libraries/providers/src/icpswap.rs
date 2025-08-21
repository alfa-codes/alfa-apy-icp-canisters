use types::CanisterId;
use candid::{Nat, Principal, Int, CandidType};
use serde::{Deserialize, Serialize};

use icpswap_swap_factory_canister::{ICPSwapToken, ICPSwapPool};
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;
use icpswap_swap_pool_canister::metadata::Metadata;
use icpswap_swap_pool_canister::getUserPosition::UserPosition;
use icpswap_swap_pool_canister::decreaseLiquidity::DecreaseLiquidityResponse;
use icpswap_swap_pool_canister::claim::ClaimResponse;
use icpswap_swap_pool_canister::getUserUnusedBalance::UserUnusedBalance;
use icpswap_swap_pool_canister::getUserPositionsByPrincipal::UserPositionWithId;
use icpswap_node_index_canister::getAllTokens::TokenData;
use icpswap_swap_calculator_canister::getTokenAmountByLiquidity::GetTokenAmountByLiquidityResponse;
use icpswap_tvl_storage_canister::getPoolChartTvl::PoolChartTvl;
use icpswap_swap_pool_canister::quote::Args as QuoteArgs;
use icpswap_swap_factory_canister::getPool::Args as GetPoolArgs;
use icpswap_swap_pool_canister::depositFrom::Args as DepositFromArgs;
use icpswap_swap_pool_canister::increaseLiquidity::Args as IncreaseLiquidityArgs;
use icpswap_swap_pool_canister::mint::Args as MintArgs;
use icpswap_swap_pool_canister::withdraw::Args as WithdrawArgs;
use icpswap_swap_pool_canister::getUserUnusedBalance::Args as GetUserUnusedBalanceArgs;
use icpswap_swap_pool_canister::decreaseLiquidity::Args as DecreaseLiquidityArgs;
use icpswap_swap_pool_canister::swap::Args as SwapArgs;
use icpswap_swap_pool_canister::claim::Args as ClaimArgs;
use types::exchange_id::ExchangeId;
use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    external_services as external_services_area,
    external_services::domains::icp_swap as icp_swap_domain,
    external_services::domains::icp_swap::components as icp_swap_domain_components,
};
use utils::constants::{
    ICP_TOKEN_PRINCIPAL,
    ICPSWAP_SWAP_FACTORY_CANISTER_ID,
    ICPSWAP_SWAP_CALCULATOR_CANISTER_ID,
    ICPSWAP_NODE_INDEX_CANISTER_ID,
    ICPSWAP_GLOBAL_INDEX_CANISTER_ID,
};

pub const SWAP_FEE: u128 = 3000; // 30%
pub const ICRC2_TOKEN_STANDARD: &str = "ICRC2";
pub const ICP_TOKEN_STANDARD: &str = "ICP";
pub const PROVIDER: ExchangeId = ExchangeId::ICPSwap;

// Module code: "01-02-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    external_services_area::AREA_CODE,  // Area code: "01"
    icp_swap_domain::DOMAIN_CODE,       // Domain code: "02"
    icp_swap_domain_components::CORE    // Component code: "01"
);

#[async_trait::async_trait]
pub trait ICPSwapProvider: Send + Sync + 'static {
    async fn get_pool(
        &self,
        token_in: CanisterId,
        token_out: CanisterId
    ) -> Result<ICPSwapPool, InternalError>;
    async fn quote(
        &self, canister_id: 
        CanisterId,
        amount_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat
    ) -> Result<Nat, InternalError>;
    async fn swap(
        &self,
        canister_id: CanisterId,
        amount_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat
    ) -> Result<Nat, InternalError>;
    async fn get_token_meta(&self, canister_id: CanisterId) -> Result<TokenMeta, InternalError>;
    async fn deposit_from(
        &self,
        canister_id: CanisterId,
        token_in: CanisterId,
        amount: Nat,
        token_fee: Nat
    ) -> Result<Nat, InternalError>;
    async fn withdraw(
        &self,
        canister_id: CanisterId,
        token_out: CanisterId,
        amount: Nat,
        token_fee: Nat
    ) -> Result<Nat, InternalError>;
    async fn metadata(&self, canister_id: CanisterId) -> Result<Metadata, InternalError>;
    async fn mint(
        &self,
        canister_id: CanisterId,
        token0: String,
        token1: String,
        amount0_desired: String,
        amount1_desired: String,
        fee: Nat,
        tick_lower: Int,
        tick_upper: Int
    ) -> Result<Nat, InternalError>;
    async fn get_user_position_ids_by_principal(
        &self,
        canister_id: CanisterId,
        principal: Principal
    ) -> Result<Vec<Nat>, InternalError>;
    async fn get_user_positions_by_principal(
        &self,
        canister_id: CanisterId,
        principal: Principal
    ) -> Result<Vec<UserPositionWithId>, InternalError>;
    async fn get_user_unused_balance(
        &self,
        canister_id: CanisterId,
        principal: String
    ) -> Result<UserUnusedBalance, InternalError>;
    async fn increase_liquidity(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
        amount0_desired: String,
        amount1_desired: String) -> Result<Nat, InternalError>;
    async fn decrease_liquidity(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
        liquidity: String
    ) -> Result<DecreaseLiquidityResponse, InternalError>;
    async fn get_user_position(
        &self,
        canister_id: CanisterId,
        position_id: Nat
    ) -> Result<UserPosition, InternalError>;
    async fn claim(
        &self,
        canister_id: CanisterId,
        position_id: Nat
    ) -> Result<ClaimResponse, InternalError>;
    async fn get_price(
        &self,
        sqrt_price_x96: Nat,
        token_0_decimals: Nat,
        token_1_decimals: Nat
    ) -> Result<f64, InternalError>;
    async fn get_token_amount_by_liquidity(
        &self,
        sqrt_price_x96: Nat,
        tick_lower: Int,
        tick_upper: Int,
        liquidity: Nat
    ) -> Result<GetTokenAmountByLiquidityResponse, InternalError>;
    async fn get_all_tokens(&self) -> Result<Vec<TokenData>, InternalError>;
    async fn get_tvl_storage_canister(&self) -> Result<Vec<String>, InternalError>;
    async fn get_pool_chart_tvl(
        &self,
        canister_id: CanisterId,
        pool_canister_id: String,
        offset: Nat,
        limit: Nat
    ) -> Result<Vec<PoolChartTvl>, InternalError>;
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct DefaultICPSwapProvider;

impl DefaultICPSwapProvider {
    fn token_icpswap_format(token: &CanisterId) -> ICPSwapToken {
        let standard = match token.to_text().as_str() {
            ICP_TOKEN_PRINCIPAL => ICP_TOKEN_STANDARD.to_string(),
            _ => ICRC2_TOKEN_STANDARD.to_string(),
        };

        ICPSwapToken {
            address: token.to_text(),
            standard,
        }
    }
}

#[async_trait::async_trait]
impl ICPSwapProvider for DefaultICPSwapProvider {
    // ================ Swap Factory canister ================

    async fn get_pool(
        &self,
        token_in: CanisterId,
        token_out: CanisterId
    ) -> Result<ICPSwapPool, InternalError> {
        let pool_args = GetPoolArgs {
            fee: candid::Nat::from(SWAP_FEE as u128),
            token0: Self::token_icpswap_format(&token_in),
            token1: Self::token_icpswap_format(&token_out),
        };

        icpswap_swap_factory_canister_c2c_client::getPool(
            *ICPSWAP_SWAP_FACTORY_CANISTER_ID,
            &pool_args
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 1), // Error code: "01-02-01 04 01"
                    "ICPSwapProvider::get_pool".to_string(),
                    format!("IC error calling 'icpswap_swap_factory_canister_c2c_client::getPool': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "token_in" => token_in,
                        "token_out" => token_out,
                        "fee" => pool_args.fee,
                        "swap_factory_canister" => ICPSWAP_SWAP_FACTORY_CANISTER_ID,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 2), // Error code: "01-02-01 03 02"
                    "ICPSwapProvider::get_pool".to_string(),
                    format!("Error calling 'icpswap_swap_factory_canister_c2c_client::getPool': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "token_in" => token_in,
                        "token_out" => token_out,
                        "fee" => pool_args.fee,
                        "swap_factory_canister" => ICPSWAP_SWAP_FACTORY_CANISTER_ID,
                    }
                )
            })
            .into_std()
    }

    // ================ Swap Pool canister ================

    async fn quote(
        &self,
        canister_id: CanisterId,
        amount_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat
    ) -> Result<Nat, InternalError> {
        let quote_args = &QuoteArgs {
            amountIn: amount_in.to_string(),
            zeroForOne: zero_for_one,
            amountOutMinimum: amount_out_minimum.to_string(),
        };

        icpswap_swap_pool_canister_c2c_client::quote(canister_id, quote_args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 3), // Error code: "01-02-01 04 03"
                    "ICPSwapProvider::quote".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::quote': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "amount_in" => amount_in,
                        "zero_for_one" => zero_for_one,
                        "amount_out_minimum" => amount_out_minimum,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 4), // Error code: "01-02-01 03 04"
                    "ICPSwapProvider::quote".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::quote': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "amount_in" => amount_in,
                        "zero_for_one" => zero_for_one,
                        "amount_out_minimum" => amount_out_minimum,
                    }
                )
            })
            .into_std()
    }

    async fn swap(
        &self,
        canister_id: CanisterId,
        amount_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat
    ) -> Result<Nat, InternalError> {
        let args = SwapArgs {
            amountIn: amount_in.to_string(),
            zeroForOne: zero_for_one,
            amountOutMinimum: amount_out_minimum.to_string(),
        };

        icpswap_swap_pool_canister_c2c_client::swap(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 5), // Error code: "01-02-01 04 05"
                    "ICPSwapProvider::swap".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::swap': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "amount_in" => amount_in,
                        "zero_for_one" => zero_for_one,
                        "amount_out_minimum" => amount_out_minimum,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 6), // Error code: "01-02-01 03 06"
                    "ICPSwapProvider::swap".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::swap': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "amount_in" => amount_in,
                        "zero_for_one" => zero_for_one,
                        "amount_out_minimum" => amount_out_minimum,
                    }
                )
            })
            .into_std()
    }

    async fn get_token_meta(
        &self,
        canister_id: CanisterId
    ) -> Result<TokenMeta, InternalError> {
        icpswap_swap_pool_canister_c2c_client::getTokenMeta(canister_id).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 7), // Error code: "01-02-01 04 07"
                    "ICPSwapProvider::get_token_meta".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getTokenMeta': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 8), // Error code: "01-02-01 03 08"
                    "ICPSwapProvider::get_token_meta".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getTokenMeta': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                    }
                )
            })
            .into_std()
    }

    async fn deposit_from(
        &self,
        canister_id: CanisterId,
        token_in: CanisterId,
        amount: Nat,
        token_fee: Nat
    ) -> Result<Nat, InternalError> {
        let args = DepositFromArgs {
            token: token_in.to_text(),
            amount: amount.clone(),
            fee: token_fee.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::depositFrom(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 9), // Error code: "01-02-01 04 09"
                    "ICPSwapProvider::deposit_from".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::depositFrom': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "token_in" => token_in,
                        "amount" => amount,
                        "token_fee" => token_fee,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 10), // Error code: "01-02-01 03 10"
                    "ICPSwapProvider::deposit_from".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::depositFrom': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "token_in" => token_in,
                        "amount" => amount,
                        "token_fee" => token_fee,
                    }
                )
            })
            .into_std()
    }

    async fn withdraw(
        &self,
        canister_id: CanisterId,
        token_out: CanisterId,
        amount: Nat,
        token_fee: Nat
    ) -> Result<Nat, InternalError> {
        let args = WithdrawArgs {
            token: token_out.to_text(),
            amount: amount.clone(),
            fee: token_fee.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::withdraw(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 11), // Error code: "01-02-01 04 11"
                    "ICPSwapProvider::withdraw".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::withdraw': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "token_out" => token_out,
                        "amount" => amount,
                        "token_fee" => token_fee,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 12), // Error code: "01-02-01 03 12"
                    "ICPSwapProvider::withdraw".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::withdraw': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "token_out" => token_out,
                        "amount" => amount,
                        "token_fee" => token_fee,
                    }
                )
            })
            .into_std()
    }

    async fn metadata(
        &self,
        canister_id: CanisterId
    ) -> Result<Metadata, InternalError> {
        icpswap_swap_pool_canister_c2c_client::metadata(canister_id).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 13), // Error code: "01-02-01 04 13"
                    "ICPSwapProvider::metadata".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::metadata': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 14), // Error code: "01-02-01 03 14"
                    "ICPSwapProvider::metadata".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::metadata': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                    }
                )
            })
            .into_std()
    }

    async fn mint(
        &self,
        canister_id: CanisterId,
        token0: String,
        token1: String,
        amount0_desired: String,
        amount1_desired: String,
        fee: Nat, 
        tick_lower: Int,
        tick_upper: Int
    ) -> Result<Nat, InternalError> {
        let args = MintArgs {
            fee: fee.clone(),
            tickUpper: tick_upper.clone(),
            token0: token0.clone(),
            token1: token1.clone(),
            amount0Desired: amount0_desired.clone(),
            amount1Desired: amount1_desired.clone(),
            tickLower: tick_lower.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::mint(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 15), // Error code: "01-02-01 04 15"
                    "ICPSwapProvider::mint".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::mint': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "token0" => token0.clone(),
                        "token1" => token1.clone(),
                        "amount0_desired" => amount0_desired.clone(),
                        "amount1_desired" => amount1_desired.clone(),
                        "fee" => fee,
                        "tick_lower" => tick_lower,
                        "tick_upper" => tick_upper,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 16), // Error code: "01-02-01 03 16"
                    "ICPSwapProvider::mint".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::mint': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "token0" => token0,
                        "token1" => token1,
                        "amount0_desired" => amount0_desired,
                        "amount1_desired" => amount1_desired,
                        "fee" => fee,
                        "tick_lower" => tick_lower,
                        "tick_upper" => tick_upper,
                    }
                )
            })
            .into_std()
    }

    async fn get_user_position_ids_by_principal(
        &self,
        canister_id: CanisterId,
        principal: Principal
    ) -> Result<Vec<Nat>, InternalError> {
        let (result,) = icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal(
            canister_id,
            (principal,)
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 17), // Error code: "01-02-01 04 17"
                    "ICPSwapProvider::get_user_position_ids_by_principal".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "principal" => principal,
                    }
                )
            })?;

        result.map_err(|error| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 18), // Error code: "01-02-01 03 18"
                "ICPSwapProvider::get_user_position_ids_by_principal".to_string(),
                format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal': {error:?}"),
                errors::error_extra! {
                    "provider" => PROVIDER,
                    "canister_id" => canister_id,
                    "principal" => principal,
                }
            )
        })
        .into_std()
    }

    async fn get_user_positions_by_principal(
        &self,
        canister_id: CanisterId,
        principal: Principal
    ) -> Result<Vec<UserPositionWithId>, InternalError> {
        let (result,) = icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal(
            canister_id,
            (principal,)
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 19), // Error code: "01-02-01 04 19"
                    "ICPSwapProvider::get_user_positions_by_principal".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "principal" => principal,
                    }
                )
            })?;

        result.map_err(|error| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 20), // Error code: "01-02-01 03 20"
                "ICPSwapProvider::get_user_positions_by_principal".to_string(),
                format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal': {error:?}"),
                errors::error_extra! {
                    "provider" => PROVIDER,
                    "canister_id" => canister_id,
                    "principal" => principal,
                }
            )
        })
        .into_std()
    }

    async fn get_user_unused_balance(
        &self,
        canister_id: CanisterId,
        principal: String,
    ) -> Result<UserUnusedBalance, InternalError> {
        let args = GetUserUnusedBalanceArgs {
            principal: principal.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 21), // Error code: "01-02-01 04 21"
                    "ICPSwapProvider::get_user_unused_balance".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "principal" => principal,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 22), // Error code: "01-02-01 03 22"
                    "ICPSwapProvider::get_user_unused_balance".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "principal" => principal,
                    }
                )
            })
            .into_std()
    }

    async fn increase_liquidity(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
        amount0_desired: String,
        amount1_desired: String
    ) -> Result<Nat, InternalError> {
        let args = IncreaseLiquidityArgs {
            positionId: position_id.clone(),
            amount0Desired: amount0_desired.clone(),
            amount1Desired: amount1_desired.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::increaseLiquidity(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 23), // Error code: "01-02-01 04 23"
                    "ICPSwapProvider::increase_liquidity".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::increaseLiquidity': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "position_id" => position_id,
                        "amount0_desired" => amount0_desired.clone(),
                        "amount1_desired" => amount1_desired.clone(),
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 24), // Error code: "01-02-01 03 24"
                    "ICPSwapProvider::increase_liquidity".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::increaseLiquidity': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "position_id" => position_id,
                        "amount0_desired" => amount0_desired,
                        "amount1_desired" => amount1_desired,
                    }
                )
            })
            .into_std()
    }

    async fn decrease_liquidity(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
        liquidity: String,
    ) -> Result<DecreaseLiquidityResponse, InternalError> {
        let args = DecreaseLiquidityArgs {
            positionId: position_id.clone(),
            liquidity: liquidity.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::decreaseLiquidity(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 25), // Error code: "01-02-01 04 25"
                    "ICPSwapProvider::decrease_liquidity".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::decreaseLiquidity': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "position_id" => position_id,
                        "liquidity" => liquidity.clone(),
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 26), // Error code: "01-02-01 03 26"
                    "ICPSwapProvider::decrease_liquidity".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::decreaseLiquidity': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "position_id" => position_id,
                        "liquidity" => liquidity,
                    }
                )
            })
            .into_std()
    }

    async fn get_user_position(
        &self,
        canister_id: CanisterId,
        position_id: Nat
    ) -> Result<UserPosition, InternalError> {
        let args = (position_id.clone(),);
        let (result,) = icpswap_swap_pool_canister_c2c_client::getUserPosition(
            canister_id,
            args
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 27), // Error code: "01-02-01 04 27"
                    "ICPSwapProvider::get_user_position".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserPosition': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "position_id" => position_id,
                    }
                )
            })?;

        result.map_err(|error| {
            InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 28), // Error code: "01-02-01 03 28"
                "ICPSwapProvider::get_user_position".to_string(),
                format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getUserPosition': {error:?}"),
                errors::error_extra! {
                    "provider" => PROVIDER,
                    "canister_id" => canister_id,
                    "position_id" => position_id,
                }
            )
        })
        .into_std()
    }

    async fn claim(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
    ) -> Result<ClaimResponse, InternalError> {
        let args = ClaimArgs {
            positionId: position_id.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::claim(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 29), // Error code: "01-02-01 04 29"
                    "ICPSwapProvider::claim".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::claim': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "position_id" => position_id,
                    }
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 30), // Error code: "01-02-01 03 30"
                    "ICPSwapProvider::claim".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::claim': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "position_id" => position_id,
                    }
                )
            })
            .into_std()
    }

    // ================ Swap Calculator canister ================

    async fn get_price(
        &self,
        sqrt_price_x96: Nat,
        token_0_decimals: Nat,
        token_1_decimals: Nat
    ) -> Result<f64, InternalError> {
        let (price,) = icpswap_swap_calculator_canister_c2c_client::getPrice(
            *ICPSWAP_SWAP_CALCULATOR_CANISTER_ID,
            (sqrt_price_x96.clone(), token_0_decimals.clone(), token_1_decimals.clone())
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 31), // Error code: "01-02-01 04 31"
                    "ICPSwapProvider::get_price".to_string(),
                    format!("IC error calling 'icpswap_swap_calculator_canister_c2c_client::getPrice': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "sqrt_price_x96" => sqrt_price_x96,
                        "token_0_decimals" => token_0_decimals,
                        "token_1_decimals" => token_1_decimals,
                        "swap_calculator_canister" => ICPSWAP_SWAP_CALCULATOR_CANISTER_ID,
                    }
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 32), // Error code: "01-02-01 03 32"
                    "ICPSwapProvider::get_price".to_string(),
                    format!("Error calling 'icpswap_swap_calculator_canister_c2c_client::getPrice': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "sqrt_price_x96" => sqrt_price_x96,
                        "token_0_decimals" => token_0_decimals,
                        "token_1_decimals" => token_1_decimals,
                        "swap_calculator_canister" => ICPSWAP_SWAP_CALCULATOR_CANISTER_ID.to_text(),
                    }
                )
            })?;

        Ok(price)
    }

    async fn get_token_amount_by_liquidity(
        &self,
        sqrt_price_x96: Nat,
        tick_lower: Int,
        tick_upper: Int,
        liquidity: Nat
    ) -> Result<GetTokenAmountByLiquidityResponse, InternalError> {
        let (result,) = icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity(
            *ICPSWAP_SWAP_CALCULATOR_CANISTER_ID,
            (sqrt_price_x96.clone(), tick_lower.clone(), tick_upper.clone(), liquidity.clone())
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 33), // Error code: "01-02-01 04 33"
                    "ICPSwapProvider::get_token_amount_by_liquidity".to_string(),
                    format!("IC error calling 'icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "sqrt_price_x96" => sqrt_price_x96,
                        "tick_lower" => tick_lower,
                        "tick_upper" => tick_upper,
                        "liquidity" => liquidity,
                        "swap_calculator_canister" => ICPSWAP_SWAP_CALCULATOR_CANISTER_ID,
                    }
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 34), // Error code: "01-02-01 03 34"
                    "ICPSwapProvider::get_token_amount_by_liquidity".to_string(),
                    format!("Error calling 'icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "sqrt_price_x96" => sqrt_price_x96,
                        "tick_lower" => tick_lower,
                        "tick_upper" => tick_upper,
                        "liquidity" => liquidity,
                        "swap_calculator_canister" => ICPSWAP_SWAP_CALCULATOR_CANISTER_ID,
                    }
                )
            })?;

        Ok(result)
    }

    // ================ Node Index canister ================

    async fn get_all_tokens(
        &self,
    ) -> Result<Vec<TokenData>, InternalError> {
        let response = icpswap_node_index_canister_c2c_client::getAllTokens(
            *ICPSWAP_NODE_INDEX_CANISTER_ID
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 35), // Error code: "01-02-01 04 35"
                    "ICPSwapProvider::get_all_tokens".to_string(),
                    format!("IC error calling 'icpswap_node_index_canister_c2c_client::getAllTokens': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "node_index_canister" => ICPSWAP_NODE_INDEX_CANISTER_ID,
                    }
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 36), // Error code: "01-02-01 03 36"
                    "ICPSwapProvider::get_all_tokens".to_string(),
                    format!("Error calling 'icpswap_node_index_canister_c2c_client::getAllTokens': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "node_index_canister" => ICPSWAP_NODE_INDEX_CANISTER_ID,
                    }
                )
            })?;

        Ok(response)
    }

    async fn get_tvl_storage_canister(
        &self,
    ) -> Result<Vec<String>, InternalError> {
        let response = icpswap_global_index_canister_c2c_client::tvlStorageCanister(
            *ICPSWAP_GLOBAL_INDEX_CANISTER_ID
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 37), // Error code: "01-02-01 04 37"
                    "ICPSwap provider::get_tvl_storage_canister".to_string(),
                    format!("IC error calling 'icpswap_global_index_canister_c2c_client::tvlStorageCanister': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "global_index_canister" => ICPSWAP_GLOBAL_INDEX_CANISTER_ID,
                    }
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 38), // Error code: "01-02-01 03 38"
                    "ICPSwapProvider::get_tvl_storage_canister".to_string(),
                    format!("Error calling 'icpswap_global_index_canister_c2c_client::tvlStorageCanister': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "global_index_canister" => ICPSWAP_GLOBAL_INDEX_CANISTER_ID,
                    }
                )
            })?;

        Ok(response)
    }

    // ================ TVL Storage canister ================

    async fn get_pool_chart_tvl(
        &self,
        canister_id: CanisterId,
        pool_canister_id: String,
        offset: Nat,
        limit: Nat
    ) -> Result<Vec<PoolChartTvl>, InternalError> {
        let (result,) = icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl(
            canister_id.clone(),
            (pool_canister_id.clone(), offset.clone(), limit.clone())
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 39), // Error code: "01-02-01 04 39"
                    "ICPSwapProvider::get_pool_chart_tvl".to_string(),
                    format!("IC error calling 'icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "pool_canister_id" => pool_canister_id.clone(),
                        "offset" => offset,
                        "limit" => limit,
                    }
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(InternalErrorKind::BusinessLogic, 40), // Error code: "01-02-01 03 40"
                    "ICPSwapProvider::get_pool_chart_tvl".to_string(),
                    format!("Error calling 'icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl': {error:?}"),
                    errors::error_extra! {
                        "provider" => PROVIDER,
                        "canister_id" => canister_id,
                        "pool_canister_id" => pool_canister_id,
                        "offset" => offset,
                        "limit" => limit,
                    }
                )
            })?;

        Ok(result)
    }
}
