use std::collections::HashMap;
use candid::Principal;

use types::pool_stats::PoolMetrics;
use errors::internal_error::error::InternalError;
use crate::utils::service_resolver::get_service_resolver;

pub struct PoolStatsActor {
    principal: Principal,
}

impl PoolStatsActor {
    pub async fn get_principal(&self) -> Principal {
        self.principal
    }

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
    let service_resolver = get_service_resolver();
    let pool_stats_principal = service_resolver.pool_stats_canister_id().unwrap();

    Ok(PoolStatsActor {
        principal: pool_stats_principal,
    })
}
