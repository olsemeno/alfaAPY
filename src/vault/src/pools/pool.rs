use candid::{CandidType, Deserialize};
use serde::Serialize;

use types::exchange_id::ExchangeId;
use types::CanisterId;
use utils::pool_id_util::generate_pool_id;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Pool {
    pub id: String,
    pub token0: CanisterId,
    pub token1: CanisterId,
    pub provider: ExchangeId,
}

impl Pool {
    pub fn new(id: String, token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self {
        Self { id, token0, token1, provider }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn build(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self {
        let id = generate_pool_id(&token0, &token1, &provider);
        Self::new(id, token0, token1, provider)
    }

    pub fn is_same_pool(&self, compared_pool: &Pool) -> bool {
        self.id == compared_pool.id
    }
}
