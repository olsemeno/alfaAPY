use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

// Pool Snapshots

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolSnapshot {
    pub id: String,
    pub pool_id: String,
    pub timestamp: u64,
    pub position_data: Option<PositionData>,
    pub pool_data: Option<PoolData>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PositionData {
    pub id: Nat,
    pub amount0: Nat,
    pub amount1: Nat,
    pub usd_amount0: Nat,
    pub usd_amount1: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolData {
    pub tvl: Nat,
    // pub balance0: Nat,
    // pub balance1: Nat,
    // pub lp_fee_0: Nat,
    // pub lp_fee_1: Nat,
}

// Pool Metrics

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq)]
pub struct PoolMetrics {
    pub apy: ApyValue,
    pub tvl: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq)]
pub struct ApyValue {
    pub tokens_apy: f64,
    pub usd_apy: f64
}
