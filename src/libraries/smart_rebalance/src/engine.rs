use crate::types::{Params, RebalanceDecision, ScoreOutput};

pub const SECONDS_IN_HOUR: u64 = 3600;
pub const SECONDS_IN_DAY: u64 = SECONDS_IN_HOUR * 24;
pub const SECONDS_IN_YEAR: u64 = SECONDS_IN_DAY * 365;

pub fn decide(
    now_secs: u64,
    last_rebalance_at: Option<u64>,
    current_score: &ScoreOutput,
    candidate_scores: &[ScoreOutput],
    params: &Params,
    position_value_usd: f64,
    current_apy_usd_sma: f64,
) -> RebalanceDecision {
    // 1) cooldown
    if let Some(ts) = last_rebalance_at {
        if now_secs.saturating_sub(ts) < params.cooldown_secs {
            return RebalanceDecision {
                should_move: false,
                target_pool_id: None,
                score_diff: 0.0,
                expected_gain: 0.0,
                rebalance_cost: current_score.components.rebalance_cost,
            };
        }
    }

    // 2) find best target by score (exclude current)
    let mut best: Option<&ScoreOutput> = None;
    for c in candidate_scores {
        if c.pool_id == current_score.pool_id { continue; }
        if best.is_none() || c.score > best.unwrap().score {
            best = Some(c);
        }
    }

    let Some(best) = best else {
        return RebalanceDecision {
            should_move: false,
            target_pool_id: None,
            score_diff: 0.0,
            expected_gain: 0.0,
            rebalance_cost: current_score.components.rebalance_cost,
        };
    };

    let score_diff = best.score - current_score.score;
    if score_diff < params.score_threshold {
        return RebalanceDecision {
            should_move: false,
            target_pool_id: Some(best.pool_id.clone()),
            score_diff,
            expected_gain: 0.0,
            rebalance_cost: best.components.rebalance_cost,
        };
    }

    // 3) expected gain vs cost
    // APY_target - APY_current (using sma_apy_usd as proxy)
    let apy_delta = best.components.sma_apy_usd - current_apy_usd_sma;
    let expected_gain_usd = (apy_delta / 100.0) * position_value_usd 
        * (params.cooldown_secs as f64 / (SECONDS_IN_YEAR as f64));

    let cost_usd = best.components.rebalance_cost;

    if expected_gain_usd >= cost_usd * params.gain_cost_multiplier {
        RebalanceDecision {
            should_move: true,
            target_pool_id: Some(best.pool_id.clone()),
            score_diff,
            expected_gain: expected_gain_usd,
            rebalance_cost: cost_usd,
        }
    } else {
        RebalanceDecision {
            should_move: false,
            target_pool_id: Some(best.pool_id.clone()),
            score_diff,
            expected_gain: expected_gain_usd,
            rebalance_cost: cost_usd,
        }
    }
}
