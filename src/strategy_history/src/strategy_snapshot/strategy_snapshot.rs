use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use ::utils::util::current_timestamp_secs;
use types::exchange_id::ExchangeId;
use types::CanisterId;
use validation::validation::Validation;
use validation::fields_validator::FieldsValidator;

use crate::types::external_canister_types::StrategyId;
use crate::repository::snapshots_repo;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq)]
pub struct StrategySnapshot {
    pub id: String,
    pub strategy_id: StrategyId,
    pub timestamp: u64,
    pub total_balance: Nat,
    pub total_shares: Nat,
    pub current_liquidity: Option<Nat>,
    pub current_liquidity_updated_at: Option<u64>,
    pub position_id: Option<u64>,
    pub users_count: u32,
    pub current_pool: Option<Pool>,
    pub test_liquidity_amount: Option<Nat>,
    pub apy: f64,
}

impl StrategySnapshot {
    pub fn new(
        id: String,
        strategy_id: StrategyId,
        timestamp: u64,
        total_balance: Nat,
        total_shares: Nat,
        current_liquidity: Option<Nat>,
        position_id: Option<u64>,
        users_count: u32,
        current_liquidity_updated_at: Option<u64>,
        current_pool: Option<Pool>,
        test_liquidity_amount: Option<Nat>,
        apy: f64,
    ) -> Self {
        Self {
            id,
            strategy_id,
            timestamp,
            total_balance,
            total_shares,
            current_liquidity,
            position_id,
            users_count,
            current_liquidity_updated_at,
            current_pool,
            test_liquidity_amount,
            apy,
        }
    }

    pub fn build(
        strategy_id: StrategyId,
        total_balance: Nat,
        total_shares: Nat,
        current_liquidity: Option<Nat>,
        position_id: Option<u64>,
        users_count: u32,
        current_liquidity_updated_at: Option<u64>,
        current_pool: Option<Pool>,
        test_liquidity_amount: Option<Nat>,
        apy: f64,
    ) -> Self {
        let id = get_next_snapshot_id(strategy_id);
        let timestamp = current_timestamp_secs();

        Self::new(
            id,
            strategy_id,
            timestamp,
            total_balance,
            total_shares,
            current_liquidity,
            position_id,
            users_count,
            current_liquidity_updated_at,
            current_pool,
            test_liquidity_amount,
            apy,
        )
    }
}

impl Validation for StrategySnapshot {
    fn define_validations(&self) -> FieldsValidator {
        FieldsValidator::new()
            .positive("total_balance", self.total_balance.clone())
            .positive("total_shares", self.total_shares.clone())
            .build()
    }
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct Pool {
    pub id: String,
    pub token0: CanisterId,
    pub token1: CanisterId,
    pub provider: ExchangeId,
}

impl Pool {
    pub fn new(
        id: String,
        token0: CanisterId,
        token1: CanisterId,
        provider: ExchangeId,
    ) -> Self {
        Self {
            id,
            token0,
            token1,
            provider,
        }
    }
}

fn get_next_snapshot_id(strategy_id: StrategyId) -> String {
    let count = snapshots_repo::get_snapshots_count_by_strategy_id(strategy_id);

    format!("{}", count + 1)
}