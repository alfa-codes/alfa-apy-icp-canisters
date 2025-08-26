use smart_rebalance::{self, types::*};
use utils::util::current_timestamp_secs;

use ::types::strategies::Pool;

use crate::pool_stats::pool_stats_service;

pub const BPS_SCALE_FACTOR: u32 = 10_000;

pub struct RebalanceInputs {
    pub current_pool: Pool,
    pub pools: Vec<Pool>,
    pub profile: StrategyProfile,
    pub last_rebalance_at: Option<u64>,
    pub position_value_usd: f64,
}

pub async fn decide_rebalance(inputs: RebalanceInputs) -> RebalanceDecision {
    let pool_ids: Vec<String> = inputs.pools.iter().map(|p| p.id.clone()).collect();
    let actor = pool_stats_service::get_pool_stats_actor().await.unwrap();
    let pool_metrics_map = actor.get_pool_metrics(pool_ids).await;

    let params = smart_rebalance::profiles::default_params_for_profile(inputs.profile);
    let fee_percent = (params.dex_fee_percent_bps as f64) / BPS_SCALE_FACTOR as f64;
    let gas_cost_usd = 0.0; // TODO: wire from config or estimation

    let mut scores: Vec<ScoreOutput> = Vec::new();

    for (pool_id, pool_metrics) in pool_metrics_map {
        let pool_score_input = PoolScoreInput {
            pool_id: pool_id.clone(),
            tvl: pool_metrics.tvl.clone(),
            volume_period: 0u64.into(), // TODO: use real volume after pool stats canister implements it
            usd_apy_series: vec![pool_metrics.apy.usd_apy], // TODO: use real series
            token_apy_series: vec![pool_metrics.apy.tokens_apy], // TODO: use real series
            usd_apy_long_term: pool_metrics.apy.usd_apy,
            avg_token_price_usd_series: vec![], // TODO: use real series
        };

        let components = smart_rebalance::metrics::compute_components(
            &pool_score_input,
            inputs.position_value_usd,
            fee_percent,
            gas_cost_usd
        );

        scores.push(
            smart_rebalance::scoring::compute_score(pool_id, &components, &params.weights)
        );
    }

    let current_score = scores
        .iter()
        .find(|s| s.pool_id == inputs.current_pool.id).cloned()
        .unwrap_or_else(|| scores[0].clone());

    smart_rebalance::engine::decide(
        current_timestamp_secs(),
        inputs.last_rebalance_at,
        &current_score,
        &scores,
        &params,
        inputs.position_value_usd,
        current_score.components.sma_apy_usd,
    )
}
