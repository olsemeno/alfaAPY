use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use types::exchanges::TokenInfo;
use types::exchange_id::ExchangeId;
use utils::pool_id_util::generate_pool_id;

use crate::repository::pools_repo;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct Pool {
    pub id: String,
    pub token0: TokenInfo,
    pub token1: TokenInfo,
    pub provider: ExchangeId,
    pub position: Option<Position>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub id: Nat,
    pub initial_amount0: Nat,
    pub initial_amount1: Nat,
}

impl Pool {
    pub fn new(id: String, token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) -> Self {
        Self { id, token0, token1, provider, position: None }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn create(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) -> Self {
        let id = generate_pool_id(&token0, &token1, &provider);
        let pool = Self::new(id, token0, token1, provider);
        pool.save();

        pool
    }

    pub fn save(&self) {
        pools_repo::save_pool(self.clone());
    }

    pub fn delete(&self) {
        pools_repo::delete_pool(self.id.clone());
    }
}
