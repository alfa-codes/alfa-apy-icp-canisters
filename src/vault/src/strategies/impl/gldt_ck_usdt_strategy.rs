use async_trait::async_trait;
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use std::collections::HashMap;

use types::CanisterId;
use types::strategies::{StrategyId, Pool};
use utils::constants::GLDT_TOKEN_CANISTER_ID;

use crate::impl_strategy_methods;
use crate::strategies::basic_strategy::BasicStrategy;
use crate::strategies::r#impl::description::STRATEGY_MAP;
use crate::strategies::strategy::IStrategy;
use crate::strategies::strategy_candid::StrategyCandid;

//TODO override deposit/withdraw to support ICPSWAP
impl_strategy_methods!(GldtCkUsdtStrategy);
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct GldtCkUsdtStrategy {
    id: StrategyId,
    base_token: CanisterId,
    current_pool: Option<Pool>,
    position_id: Option<u64>,
    total_balance: Nat,
    total_shares: Nat,
    user_shares: HashMap<Principal, Nat>,
    initial_deposit: HashMap<Principal, Nat>,
    current_liquidity: Option<Nat>,
    current_liquidity_updated_at: Option<u64>,
    enabled: bool,
}

impl GldtCkUsdtStrategy {
    pub fn new() -> Self {
        //TODO move to config
        GldtCkUsdtStrategy {
            id: 9,
            base_token: *GLDT_TOKEN_CANISTER_ID,
            current_pool: None,
            position_id: None,
            total_balance: Nat::from(0u64),
            total_shares: Nat::from(0u64),
            user_shares: HashMap::new(),
            initial_deposit: HashMap::new(),
            current_liquidity: None,
            current_liquidity_updated_at: None,
            enabled: false,
        }
    }
}

#[async_trait]
impl IStrategy for GldtCkUsdtStrategy {
    fn to_candid(&self) -> StrategyCandid {
        StrategyCandid::GldtCkUsdtStrategyV(self.clone())
    }

    fn clone_self(&self) -> Box<dyn IStrategy> {
        Box::new(self.clone())
    }
}
