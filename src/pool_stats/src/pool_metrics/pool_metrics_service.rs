use candid::Nat;

use crate::pools::pool::Pool;
use crate::pool_metrics::pool_metrics::PoolMetrics;
use crate::pool_metrics::pool_yield_service;
use crate::repository::pools_repo;
use utils::util::current_timestamp;

pub fn create_pool_metrics(pool: Pool) -> PoolMetrics {
    let snapshots = pools_repo::get_pool_snapshots(pool.id.clone()).unwrap_or_default();
    let apy = pool_yield_service::calculate_pool_yield(&snapshots, current_timestamp());
    let tvl = snapshots.iter()
        .max_by_key(|snapshot| snapshot.timestamp)
        .and_then(|snapshot| snapshot.pool_data.as_ref().map(|pool_data| pool_data.tvl.clone()))
        .unwrap_or(Nat::from(0 as u128));

    PoolMetrics {
        apy,
        tvl,
    }
}
