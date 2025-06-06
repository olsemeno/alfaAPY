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

impl Pool {
    /// Check if two pools are the same
    pub fn is_same_pool(&self, compared_pool: &Pool) -> bool {
        let direct_match = self.token0.ledger == compared_pool.token0.ledger
            && self.token1.ledger == compared_pool.token1.ledger;
        let reverse_match = self.token0.ledger == compared_pool.token1.ledger
            && self.token1.ledger == compared_pool.token0.ledger;

        (direct_match || reverse_match) && self.provider == compared_pool.provider
    }
}
