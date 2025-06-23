use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

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

    pub fn add_liquidity_to_pool_failed(pool_id: Option<String>, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        Self::AddLiquidityToPoolFailed(AddLiquidityToPoolFailed { pool_id, amount0, amount1 })
    }

    pub fn withdraw_liquidity_from_pool_started(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        Self::WithdrawLiquidityFromPoolStarted(WithdrawLiquidityFromPoolStarted { pool_id, amount0, amount1 })
    }

    pub fn withdraw_liquidity_from_pool_completed(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        Self::WithdrawLiquidityFromPoolCompleted(WithdrawLiquidityFromPoolCompleted { pool_id, amount0, amount1 })
    }

    pub fn withdraw_liquidity_from_pool_failed(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        Self::WithdrawLiquidityFromPoolFailed(WithdrawLiquidityFromPoolFailed { pool_id, amount0, amount1 })
    }
}
