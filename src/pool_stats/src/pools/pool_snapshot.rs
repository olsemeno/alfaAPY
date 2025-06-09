use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::pools::pool_data_service::{PositionData, PoolData};
use crate::repository::pools_repo;
use utils::util::current_timestamp;

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

    pub fn build(pool_id: String, position_data: Option<PositionData>, pool_data: Option<PoolData>) -> Self {
        let id = (pools_repo::get_pool_snapshots_count(pool_id.clone()) + 1).to_string();
        let timestamp = current_timestamp();

        Self::new(
            id,
            pool_id,
            timestamp,
            position_data,
            pool_data,
        )
    }

    pub fn save(&self) {
        pools_repo::save_pool_snapshot(self.clone());
    }
}
