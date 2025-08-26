use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use std::collections::HashMap;

use crate::CanisterId;
use crate::pool::PoolTrait;
use crate::exchange_id::ExchangeId;


pub type StrategyId = u16;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyResponse {
    pub name: String,
    pub id: StrategyId,
    pub base_token: CanisterId,
    pub description: String,
    pub pools: Vec<Pool>,
    pub current_pool: Option<Pool>,
    pub total_balance: Nat,
    pub total_shares: Nat,
    pub user_shares: HashMap<Principal, Nat>,
    pub initial_deposit: HashMap<Principal, Nat>,
    pub users_count: u32,
    pub current_liquidity: Option<Nat>,
    pub current_liquidity_updated_at: Option<u64>,
    pub position_id: Option<u64>,
    pub enabled: bool,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq)]
pub struct Pool {
    pub id: String,
    pub token0: CanisterId,
    pub token1: CanisterId,
    pub provider: ExchangeId,
}

impl PoolTrait for Pool {
    fn get_id(&self) -> String { self.id.clone() }
    fn get_token0(&self) -> CanisterId { self.token0 }
    fn get_token1(&self) -> CanisterId { self.token1 }
    fn get_provider(&self) -> ExchangeId { self.provider }
    fn is_same_pool(&self, compared_pool: &Self) -> bool {
        let (token0, token1, provider) = Self::decode_pool_id(&compared_pool.id).unwrap();
        self.provider == provider && (
            (self.token0 == token0 && self.token1 == token1) ||
            (self.token0 == token1 && self.token1 == token0)
        )
    }

    fn new(id: String, token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self {
        Self {
            id,
            token0,
            token1,
            provider,
        }
    }

    fn build(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self {
        let id = Self::generate_pool_id(&token0, &token1, &provider);
        Self::new(id, token0, token1, provider)
    }
}
