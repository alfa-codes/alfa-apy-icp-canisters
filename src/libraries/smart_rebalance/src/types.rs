use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

#[derive(Clone, Copy, CandidType, Deserialize, Serialize, Debug)]
pub enum StrategyProfile {
    Conservative,
    Balanced,
    Aggressive,
    TokenAccumulator,
    IncentiveFarmer,
    StableOnly,
}

#[derive(Clone, Copy, CandidType, Deserialize, Serialize, Debug)]
pub struct Weights {
    pub w1_usd_apy_sma: f64,
    pub w2_token_apy_sma: f64,
    pub w3_log_tvl: f64,
    pub w4_capital_efficiency: f64,
    pub w5_apy_volatility: f64,
    pub w6_rebalance_cost: f64,
    pub w7_token_price_volatility: f64,
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct Params {
    pub cooldown_secs: u64,
    pub score_threshold: f64,
    pub gain_cost_multiplier: f64,
    pub weights: Weights,
    pub dex_fee_percent_bps: u32,
    pub gas_cost: Nat,
    pub long_term_apy_usd_min: f64,
    pub sma_window_hours: u32,
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct PoolScoreInput {
    pub pool_id: String,
    pub tvl: Nat,
    pub volume_period: Nat,
    pub usd_apy_series: Vec<f64>,
    pub token_apy_series: Vec<f64>,
    pub usd_apy_long_term: f64,
    pub avg_token_price_usd_series: Vec<f64>,
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct ScoreComponents {
    pub sma_apy_usd: f64,
    pub sma_apy_tokens: f64,
    pub log_tvl: f64,
    pub capital_efficiency: f64,
    pub apy_volatility: f64,
    pub rebalance_cost: f64,
    pub token_price_volatility: f64,
    pub usd_apy_long_term: f64,
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct ScoreOutput {
    pub pool_id: String,
    pub score: f64,
    pub components: ScoreComponents,
}

#[derive(Clone, CandidType, Deserialize, Serialize, Debug)]
pub struct RebalanceDecision {
    pub should_move: bool,
    pub target_pool_id: Option<String>,
    pub score_diff: f64,
    pub expected_gain: f64,
    pub rebalance_cost: f64,
}
