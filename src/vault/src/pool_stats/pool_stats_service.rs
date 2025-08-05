use std::collections::HashMap;
use candid::Principal;

use types::pool_stats::PoolMetrics;
use utils::constants::POOL_STATS_PRINCIPAL_DEV;
use errors::internal_error::error::InternalError;

pub struct PoolStatsActor {
    principal: Principal,
}

impl PoolStatsActor {
    pub async fn get_pool_metrics(&self, pool_ids: Vec<String>) -> HashMap<String, PoolMetrics> {
        let (pool_metrics,): (HashMap<String, PoolMetrics>,) = 
            ic_cdk::call(
                self.principal,
                "get_pool_metrics",
                (pool_ids,)
            ).await.expect("Pool stats canister call failed");

        pool_metrics
    }
}

pub async fn get_pool_stats_actor() -> Result<PoolStatsActor, InternalError> {
    Ok(PoolStatsActor {
        principal: Principal::from_text(POOL_STATS_PRINCIPAL_DEV).unwrap(),
    })
}
