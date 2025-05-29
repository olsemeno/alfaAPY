use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::pools::pool::Pool;
use crate::pools::pool_snapshot::PoolSnapshot;
use crate::repository::pools_repo;
use crate::snapshots::apy_service;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct ApyValue {
    pub tokens_apy: f64,
    pub usd_apy: f64,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolApy {
    pub week: ApyValue,
    pub month: ApyValue,
    pub year: ApyValue,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolMetrics {
    pub pool: Pool,
    pub apy: PoolApy,
    pub snapshots: Vec<PoolSnapshot>,
    // pub tvl: u128,
}

impl PoolMetrics {
    pub fn build(pool: Pool) -> Self {
        let snapshots = pools_repo::get_pool_snapshots(pool.id.clone()).unwrap_or_default();
        let now = ic_cdk::api::time();
        let apy = apy_service::calculate_pool_apy(&snapshots, now);
        Self { pool, apy, snapshots }
    }
}
