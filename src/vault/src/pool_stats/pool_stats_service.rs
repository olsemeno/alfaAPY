use std::collections::HashMap;
use ic_cdk::call;

use types::pool_stats::PoolMetrics;
use utils::constants::POOL_STATS_CANISTER_ID;

pub async fn get_pool_metrics(pool_ids: Vec<String>) -> HashMap<String, PoolMetrics> {
    let (pool_metrics,): (HashMap<String, PoolMetrics>, ) = call(
        *POOL_STATS_CANISTER_ID,
        "get_pool_metrics",
        (pool_ids,)
    ).await.expect("Pool stats canister call failed");

    pool_metrics
}
