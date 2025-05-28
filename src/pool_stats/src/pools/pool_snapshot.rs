use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::pools::pool_data_service::{PositionData, PoolData};

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolSnapshot {
    pub pool_id: String,
    pub timestamp: u64,
    pub position_data: PositionData,
    pub pool_data: Option<PoolData>,
    // pub tvl: u128,
}

impl PoolSnapshot {
    pub fn new(
        pool_id: String, 
        timestamp: u64, 
        position_data: PositionData,
        pool_data: Option<PoolData>,
    ) -> Self {
        Self {
            pool_id,
            timestamp,
            position_data,
            pool_data,
        }
    }
}
