use candid::{CandidType, Deserialize};
use serde::Serialize;

use types::exchanges::TokenInfo;
use types::exchange_id::ExchangeId;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Pool {
    pub token0: TokenInfo,
    pub token1: TokenInfo,
    pub provider: ExchangeId,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct PoolResponse {
    pub token0: String,
    pub token1: String,
    pub provider: ExchangeId,
}

impl Pool {
    /// Check if two pools are the same
    pub fn is_same_pool(&self, compared_pool: &Pool) -> bool {
        let direct = self.token0.symbol == compared_pool.token0.symbol
            && self.token1.symbol == compared_pool.token1.symbol;
        let reverse = self.token0.symbol == compared_pool.token1.symbol
            && self.token1.symbol == compared_pool.token0.symbol;

        (direct || reverse) && self.provider == compared_pool.provider
    }

    pub fn to_response(&self) -> PoolResponse {
        PoolResponse {
            token0: self.token0.symbol.clone(),
            token1: self.token1.symbol.clone(),
            provider: self.provider.clone(),
        }
    }
}

