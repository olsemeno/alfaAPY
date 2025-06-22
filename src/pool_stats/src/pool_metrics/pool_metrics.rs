use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq)]
pub struct ApyValue {
    pub tokens_apy: f64, // TODO: rename to tokens_yield_percent
    pub usd_apy: f64, // TODO: rename to usd_change_percent
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq)]
pub struct PoolMetrics {
    pub apy: ApyValue,
    pub tvl: Nat,
}
