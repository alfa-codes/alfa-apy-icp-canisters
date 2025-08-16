use candid::Nat;

use crate::types::{Params, StrategyProfile, Weights};

pub const SECONDS_IN_HOUR: u64 = 3600;

pub fn default_params_for_profile(profile: StrategyProfile) -> Params {
    let weights = match profile {
        StrategyProfile::Conservative => Weights {
            w1_usd_apy_sma: 1.0,
            w2_token_apy_sma: 0.2,
            w3_log_tvl: 0.01,
            w4_capital_efficiency: 0.3,
            w5_apy_volatility: 2.0,
            w6_rebalance_cost: 1.5,
            w7_token_price_volatility: 2.0
        },
        StrategyProfile::Balanced => Weights {
            w1_usd_apy_sma: 1.0,
            w2_token_apy_sma: 0.4,
            w3_log_tvl: 0.02,
            w4_capital_efficiency: 0.5,
            w5_apy_volatility: 1.0,
            w6_rebalance_cost: 1.0,
            w7_token_price_volatility: 0.5
        },
        StrategyProfile::Aggressive => Weights {
            w1_usd_apy_sma: 1.0,
            w2_token_apy_sma: 0.6,
            w3_log_tvl: 0.0, 
            w4_capital_efficiency: 1.0,
            w5_apy_volatility: 0.2,
            w6_rebalance_cost: 0.3,
            w7_token_price_volatility: 0.1
        },
        StrategyProfile::TokenAccumulator => Weights {
            w1_usd_apy_sma: 0.3,
            w2_token_apy_sma: 1.0,
            w3_log_tvl: 0.01,
            w4_capital_efficiency: 0.4,
            w5_apy_volatility: 0.5,
            w6_rebalance_cost: 0.8,
            w7_token_price_volatility: 0.3
        },
        StrategyProfile::IncentiveFarmer => Weights {
            w1_usd_apy_sma: 0.8,
            w2_token_apy_sma: 0.7,
            w3_log_tvl: 0.01,
            w4_capital_efficiency: 0.7,
            w5_apy_volatility: 0.6,
            w6_rebalance_cost: 0.7,
            w7_token_price_volatility: 0.4
        },
        StrategyProfile::StableOnly => Weights {
            w1_usd_apy_sma: 1.0,
            w2_token_apy_sma: 0.3,
            w3_log_tvl: 0.05,
            w4_capital_efficiency: 0.2,
            w5_apy_volatility: 2.5,
            w6_rebalance_cost: 1.2,
            w7_token_price_volatility: 2.0
        },
    };

    let (cooldown_secs, score_threshold, gain_cost_multiplier) = match profile {
        StrategyProfile::Conservative => (72 * SECONDS_IN_HOUR, 8.0, 3.0),
        StrategyProfile::Balanced => (36 * SECONDS_IN_HOUR, 5.0, 2.0), // 24–48h -> choose middle
        StrategyProfile::Aggressive => (8 * SECONDS_IN_HOUR, 2.0, 1.2), // 6–12h -> choose middle
        StrategyProfile::TokenAccumulator => (36 * SECONDS_IN_HOUR, 3.0, 1.5),
        StrategyProfile::IncentiveFarmer => (18 * SECONDS_IN_HOUR, 4.0, 1.8),
        StrategyProfile::StableOnly => (60 * SECONDS_IN_HOUR, 6.0, 2.5),
    };

    Params {
        cooldown_secs,
        score_threshold,
        gain_cost_multiplier,
        weights,
        dex_fee_percent_bps: 60, // 0.6%
        gas_cost: Nat::from(0_u64),
        long_term_apy_usd_min: 0.0, // filter out pools with negative long-term APY
        sma_window_hours: 72,
    }
}
