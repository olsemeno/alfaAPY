use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::pools::pool_data_service::{PositionData, PoolData};

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolSnapshot {
    pub id: String,
    pub pool_id: String,
    pub timestamp: u64,
    pub position_data: Option<PositionData>,
    pub pool_data: Option<PoolData>,
}

impl PoolSnapshot {
    pub fn new(
        id: String,
        pool_id: String, 
        timestamp: u64, 
        position_data: Option<PositionData>,
        pool_data: Option<PoolData>,
    ) -> Self {
        Self {
            id,
            pool_id,
            timestamp,
            position_data,
            pool_data,
        }
    }
}
