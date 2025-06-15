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
    RemoveLiquidityFromPoolStarted,
    RemoveLiquidityFromPoolCompleted,
    RemoveLiquidityFromPoolFailed,
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
    RemoveLiquidityFromPoolStarted {
        pool_id: String,
        amount0: Option<Nat>,
        amount1: Option<Nat>,
    },
    RemoveLiquidityFromPoolCompleted {
        pool_id: String,
        amount0: Option<Nat>,
        amount1: Option<Nat>,
    },
    RemoveLiquidityFromPoolFailed {
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
            EventLogParams::RemoveLiquidityFromPoolStarted { .. } => EventLogType::RemoveLiquidityFromPoolStarted,
            EventLogParams::RemoveLiquidityFromPoolCompleted { .. } => EventLogType::RemoveLiquidityFromPoolCompleted,
            EventLogParams::RemoveLiquidityFromPoolFailed { .. } => EventLogType::RemoveLiquidityFromPoolFailed,
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

    pub fn remove_liquidity_from_pool_started(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        EventLogParams::RemoveLiquidityFromPoolStarted { pool_id, amount0, amount1 }
    }

    pub fn remove_liquidity_from_pool_completed(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        EventLogParams::RemoveLiquidityFromPoolCompleted { pool_id, amount0, amount1 }
    }

    pub fn remove_liquidity_from_pool_failed(pool_id: String, amount0: Option<Nat>, amount1: Option<Nat>) -> Self {
        EventLogParams::RemoveLiquidityFromPoolFailed { pool_id, amount0, amount1 }
    }
}
