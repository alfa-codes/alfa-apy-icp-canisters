use utils::util::nat_to_f64;

use crate::types::{PoolScoreInput, ScoreComponents};

pub fn compute_components(
    input: &PoolScoreInput,
    position_value_usd: f64,
    fee_percent: f64,
    gas_cost_usd: f64,
) -> ScoreComponents {
    let sma_apy_usd = average(&input.usd_apy_series);
    let sma_apy_tokens = average(&input.token_apy_series);

    let apy_volatility = stddev(&input.usd_apy_series);
    let token_price_volatility = stddev(&input.avg_token_price_usd_series);

    let tvl_usd = nat_to_f64(&input.tvl);
    let log_tvl = if tvl_usd > 0.0 { tvl_usd.log10() } else { 0.0 };

    let volume_usd = nat_to_f64(&input.volume_period);
    let capital_efficiency = if tvl_usd > 0.0 { volume_usd / tvl_usd } else { 0.0 };

    let rebalance_cost = (fee_percent * position_value_usd) + gas_cost_usd;

    let components = ScoreComponents {
        sma_apy_usd,
        sma_apy_tokens,
        log_tvl,
        capital_efficiency,
        apy_volatility,
        rebalance_cost,
        token_price_volatility,
        usd_apy_long_term: input.usd_apy_long_term,
    };

    // Apply long-term APY filter: if long-term < 0, we may set score-affecting fields 
    // to conservative values upstream.
    // Here we just carry the value for scoring layer to use if needed.

    components
}

pub fn average(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }

    xs.iter().sum::<f64>() / xs.len() as f64
}

pub fn stddev(xs: &[f64]) -> f64 {
    let n = xs.len();

    if n <= 1 {
        return 0.0;
    }

    let mean = average(xs);
    let var = xs.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / (n as f64);
    var.sqrt()
}

pub fn rebalance_cost_usd_from_bps(position_value_usd: f64, fee_bps: u32, gas_cost_usd: f64) -> f64 {
    let fee_percent = fee_bps as f64 / 10_000.0;
    (fee_percent * position_value_usd) + gas_cost_usd
}
