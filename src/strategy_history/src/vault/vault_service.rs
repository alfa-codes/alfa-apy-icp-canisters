use candid::Principal;
use ic_cdk::api::call::CallResult;

use utils::constants::VAULT_PRINCIPAL_DEV;
use errors::internal_error::error::{InternalError, build_error_code};

use crate::types::external_canister_types::{
    StrategyDepositArgs,
    StrategyDepositResponse,
    StrategyDepositResult,
    VaultStrategyResponse,
};

pub struct VaultActor {
    principal: Principal,
}

impl VaultActor {
    pub async fn get_strategies(&self) -> CallResult<Vec<VaultStrategyResponse>> {
        let (strategies,): (Vec<VaultStrategyResponse>,) = 
            ic_cdk::call(self.principal, "get_strategies", ()).await?;

        Ok(strategies)
    }

    pub async fn deposit(
        &self,
        args: StrategyDepositArgs
    ) -> Result<StrategyDepositResponse, InternalError> {
        let (result,): (StrategyDepositResult,) =
            ic_cdk::call(self.principal, "deposit", (args,)).await
                .map_err(|e| InternalError::external_service(
                    build_error_code(0000, 4, 0),
                    "vault_service::deposit call".to_string(),
                    format!("IC error: {:?}", e),
                    None,
                ))?;

        match result.0 {
            Ok(response) => Ok(response),
            Err(err) => Err(InternalError::business_logic(
                build_error_code(0000, 3, 0),
                "vault_service::deposit".to_string(),
                format!("Vault returned error: {}", err.message),
                None,
            )),
        }
    }
}

pub async fn get_vault_actor() -> Result<VaultActor, InternalError> {
    Ok(VaultActor {
        principal: Principal::from_text(VAULT_PRINCIPAL_DEV).unwrap(),
    })
}
