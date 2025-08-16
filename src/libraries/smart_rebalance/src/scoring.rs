use crate::types::{Weights, ScoreComponents, ScoreOutput};

pub fn compute_score(pool_id: String, c: &ScoreComponents, w: &Weights) -> ScoreOutput {
    let score =
        w.w1_usd_apy_sma * c.sma_apy_usd +
        w.w2_token_apy_sma * c.sma_apy_tokens +
        w.w3_log_tvl * c.log_tvl +
        w.w4_capital_efficiency * c.capital_efficiency -
        w.w5_apy_volatility * c.apy_volatility -
        w.w6_rebalance_cost * c.rebalance_cost -
        w.w7_token_price_volatility * c.token_price_volatility;

    ScoreOutput {
        pool_id,
        score,
        components: c.clone(),
    }
}
