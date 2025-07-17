use std::sync::Arc;

use utils::environment::Environment;
use providers::icpswap::{ICPSwapProvider, DefaultICPSwapProvider};
use providers::kongswap::{KongSwapProvider, DefaultKongSwapProvider};
use providers::mock::{icpswap::MockICPSwapProvider, kongswap::MockKongSwapProvider};
use icrc_ledger_client::{ICRCLedgerClient, DefaultICRCLedgerClient};
use icrc_ledger_client::mock::MockICRCLedgerClient;

pub struct ServiceResolver {
    env: Environment,
}

#[derive(Clone)]
pub struct ProviderImpls {
    pub kongswap: Arc<dyn KongSwapProvider + Send + Sync>,
    pub icpswap: Arc<dyn ICPSwapProvider + Send + Sync>,
}

impl ServiceResolver {
    pub fn new(env: Environment) -> Self {
        Self { env }
    }

    pub fn icrc_ledger_client(&self) -> Arc<dyn ICRCLedgerClient> {
        if self.env.should_use_mock_services() {
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
        if self.env.should_use_mock_services() {
            Arc::new(MockKongSwapProvider::new())
        } else {
            Arc::new(DefaultKongSwapProvider)
        }
    }

    pub fn icpswap_provider_impl(&self) -> Arc<dyn ICPSwapProvider> {
        if self.env.should_use_mock_services() {
            Arc::new(MockICPSwapProvider::new())
        } else {
            Arc::new(DefaultICPSwapProvider)
        }
    }
}
