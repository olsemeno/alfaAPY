use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::pools::pool::Pool;
use crate::repository::pools_repo;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolMetrics {
    pub pool: Pool,
    pub avg_apy: f64,
    // pub tvl: u128,
}

impl PoolMetrics {
    pub fn build(pool: Pool) -> Self {
        let snapshots = pools_repo::get_pool_snapshots(&pool.id).unwrap_or_default();
        let avg_apy = snapshots.iter().map(|snapshot| snapshot.apy).sum::<f64>() / snapshots.len() as f64;
        Self { pool, avg_apy }
    }
}
