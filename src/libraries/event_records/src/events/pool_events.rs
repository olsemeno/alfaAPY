use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;
use errors::internal_error::error::InternalError;

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
    pub pool_id: String,
    pub amount0: Option<Nat>,
    pub error: InternalError,
}

// Withdraw liquidity from pool
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WithdrawLiquidityFromPoolStarted {
    pub pool_id: String,
    pub total_shares: Nat,
    pub shares: Nat,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WithdrawLiquidityFromPoolCompleted {
    pub pool_id: String,
    pub total_shares: Nat,
    pub shares: Nat,
    pub amount_token0: Nat,
    pub amount_token1: Nat,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WithdrawLiquidityFromPoolFailed {
    pub pool_id: String,
    pub total_shares: Nat,
    pub shares: Nat,
    pub error: InternalError,
}
