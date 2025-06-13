use candid::{CandidType, Deserialize, Nat};
use std::collections::HashMap;
use serde::Serialize;
use types::CanisterId;

use event_logs::generic_event_log::GenericEventLog;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EventLog(pub GenericEventLog<EventLogType, EventLogParams>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventLogType {
    // Strategy Deposit
    StrategyDepositStarted,
    StrategyDepositCompleted,
    StrategyDepositFailed,
    // Strategy Withdraw
    StrategyWithdrawStarted,
    StrategyWithdrawCompleted,
    StrategyWithdrawFailed,
    // Strategy Rebalance
    StrategyRebalanceStarted,
    StrategyRebalanceCompleted,
    StrategyRebalanceFailed,
    // Add liquidity to pool
    AddLiquidityToPoolStarted,
    AddLiquidityToPoolCompleted,
    AddLiquidityToPoolFailed,
    // Remove liquidity from pool
    RemoveLiquidityFromPoolStarted,
    RemoveLiquidityFromPoolCompleted,
    RemoveLiquidityFromPoolFailed,
    // Swap token
    SwapTokenStarted,
    SwapTokenCompleted,
    SwapTokenFailed,
    // External Call
    ExternalCallStarted,
    ExternalCallCompleted,
    ExternalCallFailed,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EventLogParams {
    // Strategy Deposit
    StrategyDepositStarted {
        strategy_id: String,
        pool_id: Option<String>,
        amount0: Option<Nat>,
    },
    StrategyDepositCompleted {
        strategy_id: String,
        pool_id: Option<String>,
        amount0: Option<Nat>,
    },
    StrategyDepositFailed {
        strategy_id: String,
        pool_id: Option<String>,
        amount0: Option<Nat>,
    },
    // Strategy Withdraw
    StrategyWithdrawStarted {
        strategy_id: String,
        pool_id: Option<String>,
        shares: Option<Nat>,
    },
    StrategyWithdrawCompleted {
        strategy_id: String,
        pool_id: Option<String>,
        shares: Option<Nat>,
        amount0: Option<Nat>,
    },
    StrategyWithdrawFailed {
        strategy_id: String,
        pool_id: Option<String>,
        shares: Option<Nat>,
    },
    // Strategy Rebalance
    StrategyRebalanceStarted {
        strategy_id: String,
        previous_pool_id: Option<String>,
    },
    StrategyRebalanceCompleted {
        strategy_id: String,
        previous_pool_id: Option<String>,
        new_pool_id: Option<String>,
    },
    StrategyRebalanceFailed {
        strategy_id: String,
        previous_pool_id: Option<String>,
        new_pool_id: Option<String>,
    },
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
    // Swap token
    SwapTokenStarted {
        pool_id: String,
        token_in: CanisterId,
        token_out: CanisterId,
        amount_in: Option<Nat>,
    },
    SwapTokenCompleted {
        token_in: CanisterId,
        token_out: CanisterId,
        amount_in: Option<Nat>,
        amount_out: Option<Nat>,
    },
    SwapTokenFailed {
        pool_id: String,
        token_in: CanisterId,
        token_out: CanisterId,
        amount_in: Option<Nat>,
    },
    // External Call
    ExternalCallStarted {
        service: String,
        method: String,
        params: HashMap<String, String>,
    },
    ExternalCallCompleted {
        service: String,
        method: String,
        params: HashMap<String, String>,
        result: HashMap<String, String>,
    },
    ExternalCallFailed {
        service: String,
        method: String,
        params: HashMap<String, String>,
        error: String,
    },
}

impl EventLogParams {
    pub fn event_type(&self) -> EventLogType {
        match self {
            // Strategy Deposit
            EventLogParams::StrategyDepositStarted { .. } => EventLogType::StrategyDepositStarted,
            EventLogParams::StrategyDepositCompleted { .. } => EventLogType::StrategyDepositCompleted,
            EventLogParams::StrategyDepositFailed { .. } => EventLogType::StrategyDepositFailed,
            // Strategy Withdraw
            EventLogParams::StrategyWithdrawStarted { .. } => EventLogType::StrategyWithdrawStarted,
            EventLogParams::StrategyWithdrawCompleted { .. } => EventLogType::StrategyWithdrawCompleted,
            EventLogParams::StrategyWithdrawFailed { .. } => EventLogType::StrategyWithdrawFailed,
            // Strategy Rebalance
            EventLogParams::StrategyRebalanceStarted { .. } => EventLogType::StrategyRebalanceStarted,
            EventLogParams::StrategyRebalanceCompleted { .. } => EventLogType::StrategyRebalanceCompleted,
            EventLogParams::StrategyRebalanceFailed { .. } => EventLogType::StrategyRebalanceFailed,
            // Add liquidity to pool
            EventLogParams::AddLiquidityToPoolStarted { .. } => EventLogType::AddLiquidityToPoolStarted,
            EventLogParams::AddLiquidityToPoolCompleted { .. } => EventLogType::AddLiquidityToPoolCompleted,
            EventLogParams::AddLiquidityToPoolFailed { .. } => EventLogType::AddLiquidityToPoolFailed,
            // Remove liquidity from pool
            EventLogParams::RemoveLiquidityFromPoolStarted { .. } => EventLogType::RemoveLiquidityFromPoolStarted,
            EventLogParams::RemoveLiquidityFromPoolCompleted { .. } => EventLogType::RemoveLiquidityFromPoolCompleted,
            EventLogParams::RemoveLiquidityFromPoolFailed { .. } => EventLogType::RemoveLiquidityFromPoolFailed,
            // Swap token
            EventLogParams::SwapTokenStarted { .. } => EventLogType::SwapTokenStarted,
            EventLogParams::SwapTokenCompleted { .. } => EventLogType::SwapTokenCompleted,
            EventLogParams::SwapTokenFailed { .. } => EventLogType::SwapTokenFailed,
            // External Call
            EventLogParams::ExternalCallStarted { .. } => EventLogType::ExternalCallStarted,
            EventLogParams::ExternalCallCompleted { .. } => EventLogType::ExternalCallCompleted,
            EventLogParams::ExternalCallFailed { .. } => EventLogType::ExternalCallFailed,
        }
    }
}
