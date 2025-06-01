use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use crate::pools::pool::Pool;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct ApyValue {
    pub tokens_apy: u128,
    pub usd_apy: u128,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolApy {
    pub week: ApyValue,
    pub month: ApyValue,
    pub year: ApyValue,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolMetrics {
    pub pool: Pool,
    pub apy: PoolApy,
    pub tvl: Nat,
}
