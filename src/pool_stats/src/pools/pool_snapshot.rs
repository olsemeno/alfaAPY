use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::pools::pool_data_service::{CurrentLpPosition, PoolCurrentData};

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolSnapshot {
    pub pool_id: String,
    pub timestamp: u64,
    pub lp_position: CurrentLpPosition, 
    pub pool_data: PoolCurrentData,
    pub apy: f64,
    // pub tvl: u128,
}

impl PoolSnapshot {
    pub fn new(
        pool_id: String, 
        timestamp: u64, 
        lp_position: CurrentLpPosition,
        pool_data: PoolCurrentData, 
        apy: f64,
    ) -> Self {
        Self { 
            pool_id,
            timestamp,
            lp_position,
            pool_data,
            apy
        }
    }
}
