use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use types::exchange_id::ExchangeId;
use types::CanisterId;
use types::pool::PoolTrait;

use crate::repository::pools_repo;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct Pool {
    pub id: String,
    pub token0: CanisterId,
    pub token1: CanisterId,
    pub provider: ExchangeId,
    pub position_id: Option<u64>,
}

impl PoolTrait for Pool {
    fn get_id(&self) -> String { self.id.clone() }
    fn get_token0(&self) -> CanisterId { self.token0 }
    fn get_token1(&self) -> CanisterId { self.token1 }
    fn get_provider(&self) -> ExchangeId { self.provider }
    fn is_same_pool(&self, compared_pool: &Self) -> bool {
        let (token0, token1, provider) = Self::decode_pool_id(&compared_pool.id).unwrap();
        self.provider == provider && (
            (self.token0 == token0 && self.token1 == token1) ||
            (self.token0 == token1 && self.token1 == token0)
        )
    }

    fn new(id: String, token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self {
        Self {
            id,
            token0,
            token1,
            provider,
            position_id: None,
        }
    }

    fn build(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self {
        let id = Self::generate_pool_id(&token0, &token1, &provider);
        Self::new(id, token0, token1, provider)
    }
}

impl Pool {
    pub fn save(&self) {
        pools_repo::save_pool(self.clone());
    }

    pub fn delete(&self) {
        pools_repo::delete_pool(self.id.clone());
    }
}
