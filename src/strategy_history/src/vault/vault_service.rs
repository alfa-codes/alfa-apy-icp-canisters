use candid::Principal;
use ic_cdk::api::call::CallResult;

use utils::constants::VAULT_PRINCIPAL_DEV;
use errors::internal_error::error::InternalError;

use crate::types::types::VaultStrategyResponse;

pub struct VaultActor {
    principal: Principal,
}

impl VaultActor {
    pub async fn get_strategies(&self) -> CallResult<Vec<VaultStrategyResponse>> {
        let (strategies,): (Vec<VaultStrategyResponse>,) = 
            ic_cdk::call(self.principal, "get_strategies", ()).await?;

        Ok(strategies)
    }
}

pub async fn get_vault_actor() -> Result<VaultActor, InternalError> {
    Ok(VaultActor {
        principal: Principal::from_text(VAULT_PRINCIPAL_DEV).unwrap(),
    })
} 