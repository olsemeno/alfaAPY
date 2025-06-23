use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use event_logs::generic_event_log::GenericEventLog;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EventLog(pub GenericEventLog<EventLogType, EventLogParams>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventLogType {
    // Add liquidity to pool
    AddLiquidityToPoolStarted,
    AddLiquidityToPoolCompleted,
    AddLiquidityToPoolFailed,
    // Remove liquidity from pool
    WithdrawLiquidityFromPoolStarted,
    WithdrawLiquidityFromPoolCompleted,
    WithdrawLiquidityFromPoolFailed,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventLogParams {
    // Add liquidity to pool
    AddLiquidityToPoolStarted {
        pool_id: String,
        amount0: Option<Nat>,
        amount1: Option<Nat>,
    },
    AddLiquidityToPoolCompleted {
        pool_id: String,
        amount0: Option<Nat>,
        amount1: Option<Nat>,
    },
    AddLiquidityToPoolFailed {
        pool_id: String,
        amount0: Option<Nat>,
        amount1: Option<Nat>,
    },
    // Remove liquidity from pool
    WithdrawLiquidityFromPoolStarted {
        pool_id: String,
        amount0: Option<Nat>,
        amount1: Option<Nat>,
    },
    WithdrawLiquidityFromPoolCompleted {
        pool_id: String,
        amount0: Option<Nat>,
        amount1: Option<Nat>,
    },
    WithdrawLiquidityFromPoolFailed {
        pool_id: String,
        amount0: Option<Nat>,
        amount1: Option<Nat>,
    },
}

impl EventLogParams {
    pub fn event_type(&self) -> EventLogType {
        match self {
            // Add liquidity to pool
            EventLogParams::AddLiquidityToPoolStarted { .. } => EventLogType::AddLiquidityToPoolStarted,
            EventLogParams::AddLiquidityToPoolCompleted { .. } => EventLogType::AddLiquidityToPoolCompleted,
            EventLogParams::AddLiquidityToPoolFailed { .. } => EventLogType::AddLiquidityToPoolFailed,
            // Remove liquidity from pool
            EventLogParams::WithdrawLiquidityFromPoolStarted { .. } => EventLogType::WithdrawLiquidityFromPoolStarted,
            EventLogParams::WithdrawLiquidityFromPoolCompleted { .. } => EventLogType::WithdrawLiquidityFromPoolCompleted,
            EventLogParams::WithdrawLiquidityFromPoolFailed { .. } => EventLogType::WithdrawLiquidityFromPoolFailed,
        }
    }

    pub fn add_liquidity_to_pool_started(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        EventLogParams::AddLiquidityToPoolStarted { pool_id, amount0, amount1 }
    }

    pub fn add_liquidity_to_pool_completed(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        EventLogParams::AddLiquidityToPoolCompleted { pool_id, amount0, amount1 }
    }

    pub fn add_liquidity_to_pool_failed(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        EventLogParams::AddLiquidityToPoolFailed { pool_id, amount0, amount1 }
    }

    pub fn withdraw_liquidity_from_pool_started(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        EventLogParams::WithdrawLiquidityFromPoolStarted { pool_id, amount0, amount1 }
    }

    pub fn withdraw_liquidity_from_pool_completed(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        EventLogParams::WithdrawLiquidityFromPoolCompleted { pool_id, amount0, amount1 }
    }

    pub fn withdraw_liquidity_from_pool_failed(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        EventLogParams::WithdrawLiquidityFromPoolFailed { pool_id, amount0, amount1 }
    }
}
