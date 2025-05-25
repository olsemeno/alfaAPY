use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use types::exchanges::TokenInfo;
use types::exchange_id::ExchangeId;
use crate::repository::pools_repo;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Pool {
    pub id: String,
    pub token0: TokenInfo,
    pub token1: TokenInfo,
    pub provider: ExchangeId,
    pub lp_position: Option<LpPosition>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct LpPosition {
    pub id: Nat,
    pub initial_amount0: Nat,
    pub initial_amount1: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct CurrentLpPosition {
    pub id: Nat,
    pub initial_amount0: Nat,
    pub initial_amount1: Nat,
    pub current_amount0: Nat,
    pub current_amount1: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolCurrentData {
    pub balance0: Nat,
    pub balance1: Nat,
    pub lp_fee_0: Nat,
    pub lp_fee_1: Nat,
}

impl Pool {
    pub fn new(id: String, token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) -> Self {
        Self { id, token0, token1, provider, lp_position: None }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn save(&self) {
        pools_repo::save_pool(self.clone());
    }

    pub fn delete(&self) {
        pools_repo::delete_pool(&self.id);
    }

    // TODO: implement get_current_lp_position
    pub fn get_current_lp_position(&self) -> Option<CurrentLpPosition> {
        // get data from provider
        
        Some(CurrentLpPosition {
            id: Nat::from(0 as u64),
            initial_amount0: Nat::from(0 as u64),
            initial_amount1: Nat::from(0 as u64),
            current_amount0: Nat::from(0 as u64), 
            current_amount1: Nat::from(0 as u64),
        })
    }

    // TODO: implement get__current_data
    pub fn get_current_data(&self) -> PoolCurrentData {
        // get data from provider

        PoolCurrentData {
            balance0: Nat::from(0 as u64),
            balance1: Nat::from(0 as u64),
            lp_fee_0: Nat::from(0 as u64),
            lp_fee_1: Nat::from(0 as u64),
        }
    }
}
