use candid::Principal;
use ic_cdk::api::call::CallResult;

use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    canisters as canister_area,
    canisters::domains::strategy_history as strategy_history_domain,
    canisters::domains::strategy_history::components as strategy_history_components,
};
use ::types::strategies::StrategyResponse;
use crate::utils::service_resolver::get_service_resolver;

use crate::types::external_canister_types::{
    StrategyDepositArgs,
    StrategyDepositResponse,
    StrategyWithdrawArgs,
    StrategyWithdrawResponse,
    ResponseError,
};

// Module code: "03-03-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    canister_area::AREA_CODE,             // Area code: "03"
    strategy_history_domain::DOMAIN_CODE, // Domain code: "03"
    strategy_history_components::CORE     // Component code: "01"
);

pub struct VaultActor {
    principal: Principal,
}

impl VaultActor {
    pub async fn get_principal(&self) -> Principal {
        self.principal
    }

    pub async fn get_strategies(&self) -> CallResult<Vec<StrategyResponse>> {
        let (strategies,): (Vec<StrategyResponse>,) = 
            ic_cdk::call(self.principal, "get_strategies", ()).await?;

        Ok(strategies)
    }

    pub async fn deposit(
        &self,
        args: StrategyDepositArgs
    ) -> Result<StrategyDepositResponse, InternalError> {
        let (result,): (Result<StrategyDepositResponse, ResponseError>,) =
            ic_cdk::call(self.principal, "deposit", (args,))
                .await
                .map_err(|e| InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 9), // Error code: "03-03-01 04 09"
                    "vault_service::deposit call".to_string(),
                    format!("IC error: {:?}", e),
                    None,
                ))?;

        match result {
            Ok(response) => Ok(response),
            Err(err) => Err(InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 10), // Error code: "03-03-01 03 10"
                "vault_service::deposit".to_string(),
                format!("Vault returned error: {}", err.message),
                None,
            )),
        }
    }

    pub async fn withdraw(
        &self,
        args: StrategyWithdrawArgs
    ) -> Result<StrategyWithdrawResponse, InternalError> {
        let (result,): (Result<StrategyWithdrawResponse, ResponseError>,) =
            ic_cdk::call(self.principal, "withdraw", (args,))
                .await
                .map_err(|e| InternalError::external_service(
                    build_error_code(InternalErrorKind::ExternalService, 12), // Error code: "03-03-01 04 12"
                    "vault_service::withdraw call".to_string(),
                    format!("IC error: {:?}", e),
                    None,
                ))?;

        match result {
            Ok(response) => Ok(response),
            Err(err) => Err(InternalError::business_logic(
                build_error_code(InternalErrorKind::BusinessLogic, 13), // Error code: "03-03-01 03 13"
                "vault_service::withdraw".to_string(),
                format!("Vault returned error: {}", err.message),
                None,
            )),
        }
    }
}

pub async fn get_vault_actor() -> Result<VaultActor, InternalError> {
    let service_resolver = get_service_resolver();
    let vault_principal = service_resolver.vault_canister_id().unwrap();

    Ok(VaultActor {
        principal: vault_principal,
    })
}
