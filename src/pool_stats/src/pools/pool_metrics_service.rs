use candid::Nat;

use crate::pools::pool::Pool;
use crate::pools::pool_metrics::PoolMetrics;
use crate::snapshots::apy_service;
use crate::repository::pools_repo;

pub fn create_pool_metrics(pool: Pool) -> PoolMetrics {
    let snapshots = pools_repo::get_pool_snapshots(pool.id.clone()).unwrap_or_default();
    let now = ic_cdk::api::time();
    let apy = apy_service::calculate_pool_apy(&snapshots, now);
    let tvl = snapshots.iter()
        .max_by_key(|snapshot| snapshot.timestamp)
        .and_then(|snapshot| snapshot.pool_data.as_ref().map(|pool_data| pool_data.tvl.clone()))
        .unwrap_or(Nat::from(0 as u128));

    PoolMetrics { pool, apy, tvl }
}
