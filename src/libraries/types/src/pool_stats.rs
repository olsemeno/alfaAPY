use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use crate::exchanges::TokenInfo;
use crate::exchange_id::ExchangeId;

// Pool

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Pool {
    pub id: String,
    pub token0: TokenInfo,
    pub token1: TokenInfo,
    pub provider: ExchangeId,
    pub position: Option<Position>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Position {
    pub id: Nat,
    pub initial_amount0: Nat,
    pub initial_amount1: Nat,
}

// Pool Snapshots

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolSnapshot {
    pub pool_id: String,
    pub timestamp: u64,
    pub position_data: Option<PositionData>,
    pub pool_data: Option<PoolData>,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PositionData {
    pub id: Nat,
    pub amount0: Nat,
    pub amount1: Nat,
    pub usd_amount0: Nat,
    pub usd_amount1: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolData {
    pub tvl: Nat,
    // pub balance0: Nat,
    // pub balance1: Nat,
    // pub lp_fee_0: Nat,
    // pub lp_fee_1: Nat,
}

// Pool Metrics

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolMetrics {
    pub pool: Pool,
    pub apy: PoolApy,
    pub snapshots: Vec<PoolSnapshot>,
    // pub tvl: u128,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct ApyValue {
    pub tokens_apy: f64,
    pub usd_apy: f64,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolApy {
    pub week: ApyValue,
    pub month: ApyValue,
    pub year: ApyValue,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct GetPoolMetricsArgs {
    pub token0: TokenInfo,
    pub token1: TokenInfo,
    pub provider: ExchangeId,
}
