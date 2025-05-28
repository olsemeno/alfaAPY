use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::pools::pool_data_service::{CurrentPosition, PoolCurrentData};

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolSnapshot {
    pub pool_id: String,
    pub timestamp: u64,
    pub position: CurrentPosition,
    pub pool_data: Option<PoolCurrentData>,
    pub apy: f64,
    // pub tvl: u128,
}

impl PoolSnapshot {
    pub fn new(
        pool_id: String, 
        timestamp: u64, 
        position: CurrentPosition,
        pool_data: Option<PoolCurrentData>,
        apy: f64,
    ) -> Self {
        Self {
            pool_id,
            timestamp,
            position,
            pool_data,
            apy
        }
    }
}
