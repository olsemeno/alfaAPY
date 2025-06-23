use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

// Add liquidity to pool
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AddLiquidityToPoolStarted {
    pub pool_id: String,
    pub amount0: Option<Nat>,
    pub amount1: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AddLiquidityToPoolCompleted {
    pub pool_id: String,
    pub amount0: Option<Nat>,
    pub amount1: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AddLiquidityToPoolFailed {
    pub pool_id: Option<String>,
    pub amount0: Option<Nat>,
    pub amount1: Option<Nat>,
}

// Remove liquidity from pool
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WithdrawLiquidityFromPoolStarted {
    pub pool_id: String,
    pub amount0: Option<Nat>,
    pub amount1: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WithdrawLiquidityFromPoolCompleted {
    pub pool_id: String,
    pub amount0: Option<Nat>,
    pub amount1: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WithdrawLiquidityFromPoolFailed {
    pub pool_id: String,
    pub amount0: Option<Nat>,
    pub amount1: Option<Nat>,
} 