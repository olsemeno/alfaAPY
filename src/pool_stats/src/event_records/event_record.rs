use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use errors::internal_error::error::InternalError;
use event_records::generic_event_record::GenericEventRecord;
use event_records::events::pool_events::*;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EventRecord(pub GenericEventRecord<Event>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum Event {
    AddLiquidityToPoolStarted(AddLiquidityToPoolStarted),
    AddLiquidityToPoolCompleted(AddLiquidityToPoolCompleted),
    AddLiquidityToPoolFailed(AddLiquidityToPoolFailed),
    WithdrawLiquidityFromPoolStarted(WithdrawLiquidityFromPoolStarted),
    WithdrawLiquidityFromPoolCompleted(WithdrawLiquidityFromPoolCompleted),
    WithdrawLiquidityFromPoolFailed(WithdrawLiquidityFromPoolFailed),
}

impl Event {
    pub fn type_str(&self) -> &'static str {
        match self {
            // Add liquidity to pool
            Self::AddLiquidityToPoolStarted(_) => "AddLiquidityToPoolStarted",
            Self::AddLiquidityToPoolCompleted(_) => "AddLiquidityToPoolCompleted",
            Self::AddLiquidityToPoolFailed(_) => "AddLiquidityToPoolFailed",
            // Withdraw liquidity from pool
            Self::WithdrawLiquidityFromPoolStarted(_) => "WithdrawLiquidityFromPoolStarted",
            Self::WithdrawLiquidityFromPoolCompleted(_) => "WithdrawLiquidityFromPoolCompleted",
            Self::WithdrawLiquidityFromPoolFailed(_) => "WithdrawLiquidityFromPoolFailed",
        }
    }

    pub fn add_liquidity_to_pool_started(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        Self::AddLiquidityToPoolStarted(AddLiquidityToPoolStarted { pool_id, amount0, amount1 })
    }

    pub fn add_liquidity_to_pool_completed(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        Self::AddLiquidityToPoolCompleted(AddLiquidityToPoolCompleted { pool_id, amount0, amount1 })
    }

    pub fn add_liquidity_to_pool_failed(pool_id: String, amount0: Option<Nat>, error: InternalError) -> Self {
        Self::AddLiquidityToPoolFailed(AddLiquidityToPoolFailed { pool_id, amount0, error })
    }

    pub fn withdraw_liquidity_from_pool_started(pool_id: String, total_shares: Nat, shares: Nat) -> Self {
        Self::WithdrawLiquidityFromPoolStarted(WithdrawLiquidityFromPoolStarted { pool_id, total_shares, shares })
    }

    pub fn withdraw_liquidity_from_pool_completed(pool_id: String, total_shares: Nat, shares: Nat, amount_token0: Nat, amount_token1: Nat) -> Self {
        Self::WithdrawLiquidityFromPoolCompleted(WithdrawLiquidityFromPoolCompleted { pool_id, total_shares, shares, amount_token0, amount_token1 })
    }

    pub fn withdraw_liquidity_from_pool_failed(pool_id: String, total_shares: Nat, shares: Nat, error: InternalError) -> Self {
        Self::WithdrawLiquidityFromPoolFailed(WithdrawLiquidityFromPoolFailed { pool_id, total_shares, shares, error })
    }
}
