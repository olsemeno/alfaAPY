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
        amount0: Nat,
        amount1: Nat,
    },
    AddLiquidityToPoolCompleted {
        pool_id: String,
        amount0: Nat,
        amount1: Nat,
    },
    AddLiquidityToPoolFailed {
        pool_id: String,
        amount0: Nat,
        amount1: Nat,
    },
    // Remove liquidity from pool
    RemoveLiquidityFromPoolStarted {
        pool_id: String,
        amount0: Nat,
        amount1: Nat,
    },
    RemoveLiquidityFromPoolCompleted {
        pool_id: String,
        amount0: Nat,
        amount1: Nat,
    },
    RemoveLiquidityFromPoolFailed {
        pool_id: String,
        amount0: Nat,
        amount1: Nat,
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
}
