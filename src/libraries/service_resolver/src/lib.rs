use std::sync::Arc;

use utils::environment::Environment;
use providers::icpswap::{ICPSwapProvider, DefaultICPSwapProvider};
use providers::kongswap::{KongSwapProvider, DefaultKongSwapProvider};
use providers::mock::{icpswap::MockICPSwapProvider, kongswap::MockKongSwapProvider};
use icrc_ledger_client::{ICRCLedgerClient, DefaultICRCLedgerClient};
use icrc_ledger_client::mock::MockICRCLedgerClient;
use types::CanisterId;
use utils::constants::{
    VAULT_CANISTER_ID_STAGING,
    VAULT_CANISTER_ID_DEV,
    VAULT_CANISTER_ID_PRODUCTION,
    POOL_STATS_CANISTER_ID_STAGING,
    POOL_STATS_CANISTER_ID_DEV,
    POOL_STATS_CANISTER_ID_PRODUCTION
};

pub struct ServiceResolver {
    environment: Environment,
}

#[derive(Clone)]
pub struct ProviderImpls {
    pub kongswap: Arc<dyn KongSwapProvider + Send + Sync>,
    pub icpswap: Arc<dyn ICPSwapProvider + Send + Sync>,
}

impl ServiceResolver {
    pub fn new(environment: Environment) -> Self {
        Self { environment }
    }

    pub fn vault_canister_id(&self) -> Option<CanisterId> {
        match self.environment {
            Environment::Staging => Some(*VAULT_CANISTER_ID_STAGING),
            Environment::Production => Some(*VAULT_CANISTER_ID_PRODUCTION),
            Environment::Dev => Some(*VAULT_CANISTER_ID_DEV),
            Environment::Test => None,
        }
    }

    pub fn pool_stats_canister_id(&self) -> Option<CanisterId> {
        match self.environment {
            Environment::Staging => Some(*POOL_STATS_CANISTER_ID_STAGING),
            Environment::Production => Some(*POOL_STATS_CANISTER_ID_PRODUCTION),
            Environment::Dev => Some(*POOL_STATS_CANISTER_ID_DEV),
            Environment::Test => None,
        }
    }

    pub fn icrc_ledger_client(&self) -> Arc<dyn ICRCLedgerClient> {
        if self.environment.should_use_mock_services() {
            Arc::new(MockICRCLedgerClient::new())
        } else {
            Arc::new(DefaultICRCLedgerClient)
        }
    }

    pub fn provider_impls(&self) -> ProviderImpls {
        ProviderImpls {
            kongswap: self.kongswap_provider_impl(),
            icpswap: self.icpswap_provider_impl(),
        }
    }

    pub fn kongswap_provider_impl(&self) -> Arc<dyn KongSwapProvider> {
        if self.environment.should_use_mock_services() {
            Arc::new(MockKongSwapProvider::new())
        } else {
            Arc::new(DefaultKongSwapProvider)
        }
    }

    pub fn icpswap_provider_impl(&self) -> Arc<dyn ICPSwapProvider> {
        if self.environment.should_use_mock_services() {
            Arc::new(MockICPSwapProvider::new())
        } else {
            Arc::new(DefaultICPSwapProvider)
        }
    }
}
