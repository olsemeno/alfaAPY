use once_cell::sync::Lazy;
use candid::Principal;
use ic_cdk::call;

use types::pool_stats::{PoolMetrics, PoolByTokens};
use utils::util::principal_to_canister_id;

use crate::pools::pool::Pool;

pub static POOL_STATS_CANISTER_ID: Lazy<Principal> = Lazy::new(|| principal_to_canister_id("oxawg-7aaaa-aaaag-aub6q-cai"));

pub async fn get_pool_metrics(pools: Vec<Pool>) -> Vec<Option<PoolMetrics>> {
    let args: Vec<PoolByTokens> = pools.iter().map(|pool|
        PoolByTokens {
            token0: pool.token0.clone(),
            token1: pool.token1.clone(),
            provider: pool.provider.clone()
        }
    ).collect();

    let (pool_metrics,): (Vec<Option<PoolMetrics>>, ) = call(
        *POOL_STATS_CANISTER_ID,
        "get_pool_metrics",
        (args,)
    ).await.expect("Pool stats canister call failed");

    pool_metrics
}
