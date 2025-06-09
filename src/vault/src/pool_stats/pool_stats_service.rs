use std::collections::HashMap;
use once_cell::sync::Lazy;
use candid::Principal;
use ic_cdk::call;

use types::pool_stats::PoolMetrics;
use utils::util::principal_to_canister_id;

pub static POOL_STATS_CANISTER_ID: Lazy<Principal> = Lazy::new(|| principal_to_canister_id("oxawg-7aaaa-aaaag-aub6q-cai"));

pub async fn get_pool_metrics(pool_ids: Vec<String>) -> HashMap<String, PoolMetrics> {
    let (pool_metrics,): (HashMap<String, PoolMetrics>, ) = call(
        *POOL_STATS_CANISTER_ID,
        "get_pool_metrics",
        (pool_ids,)
    ).await.expect("Pool stats canister call failed");


    pool_metrics
}
