use candid::{CandidType, Deserialize};
use serde::Serialize;

use types::exchange_id::ExchangeId;
use types::CanisterId;
use types::pool::PoolTrait;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Pool {
    pub id: String,
    pub token0: CanisterId,
    pub token1: CanisterId,
    pub provider: ExchangeId,
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
        }
    }

    fn build(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Self {
        let id = Self::generate_pool_id(&token0, &token1, &provider);
        Self::new(id, token0, token1, provider)
    }
}
