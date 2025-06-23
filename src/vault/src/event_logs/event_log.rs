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
    WithdrawLiquidityFromPoolStarted,
    WithdrawLiquidityFromPoolCompleted,
    WithdrawLiquidityFromPoolFailed,
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
            EventLogParams::WithdrawLiquidityFromPoolStarted { .. } => EventLogType::WithdrawLiquidityFromPoolStarted,
            EventLogParams::WithdrawLiquidityFromPoolCompleted { .. } => EventLogType::WithdrawLiquidityFromPoolCompleted,
            EventLogParams::WithdrawLiquidityFromPoolFailed { .. } => EventLogType::WithdrawLiquidityFromPoolFailed,
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

    pub fn strategy_deposit_started(strategy_id: String, pool_id: Option<String>, amount0: Option<Nat>) -> Self {
        EventLogParams::StrategyDepositStarted { strategy_id, pool_id, amount0 }
    }

    pub fn strategy_deposit_completed(strategy_id: String, pool_id: Option<String>, amount0: Option<Nat>) -> Self {
        EventLogParams::StrategyDepositCompleted { strategy_id, pool_id, amount0 }
    }

    pub fn strategy_deposit_failed(strategy_id: String, pool_id: Option<String>, amount0: Option<Nat>) -> Self {
        EventLogParams::StrategyDepositFailed { strategy_id, pool_id, amount0 }
    }
    
    pub fn strategy_withdraw_started(strategy_id: String, pool_id: Option<String>, shares: Option<Nat>) -> Self {
        EventLogParams::StrategyWithdrawStarted { strategy_id, pool_id, shares }
    }

    pub fn strategy_withdraw_completed(strategy_id: String, pool_id: Option<String>, shares: Option<Nat>, amount0: Option<Nat>) -> Self {
        EventLogParams::StrategyWithdrawCompleted { strategy_id, pool_id, shares, amount0 }
    }

    pub fn strategy_withdraw_failed(strategy_id: String, pool_id: Option<String>, shares: Option<Nat>) -> Self {
        EventLogParams::StrategyWithdrawFailed { strategy_id, pool_id, shares }
    }
    
    pub fn strategy_rebalance_started(strategy_id: String, previous_pool_id: Option<String>) -> Self {
        EventLogParams::StrategyRebalanceStarted { strategy_id, previous_pool_id }
    }

    pub fn strategy_rebalance_completed(strategy_id: String, previous_pool_id: Option<String>, new_pool_id: Option<String>) -> Self {
        EventLogParams::StrategyRebalanceCompleted { strategy_id, previous_pool_id, new_pool_id }
    }

    pub fn strategy_rebalance_failed(strategy_id: String, previous_pool_id: Option<String>, new_pool_id: Option<String>) -> Self {
        EventLogParams::StrategyRebalanceFailed { strategy_id, previous_pool_id, new_pool_id }
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

    pub fn swap_token_started(pool_id: String, token_in: CanisterId, token_out: CanisterId, amount_in: Option<Nat>) -> Self {
        EventLogParams::SwapTokenStarted { pool_id, token_in, token_out, amount_in }
    }

    pub fn swap_token_completed(token_in: CanisterId, token_out: CanisterId, amount_in: Option<Nat>, amount_out: Option<Nat>) -> Self {
        EventLogParams::SwapTokenCompleted { token_in, token_out, amount_in, amount_out }
    }

    pub fn swap_token_failed(pool_id: String, token_in: CanisterId, token_out: CanisterId, amount_in: Option<Nat>) -> Self {
        EventLogParams::SwapTokenFailed { pool_id, token_in, token_out, amount_in }
    }

    pub fn external_call_started(service: String, method: String, params: HashMap<String, String>) -> Self {
        EventLogParams::ExternalCallStarted { service, method, params }
    }

    pub fn external_call_completed(service: String, method: String, params: HashMap<String, String>, result: HashMap<String, String>) -> Self {
        EventLogParams::ExternalCallCompleted { service, method, params, result }
    }

    pub fn external_call_failed(service: String, method: String, params: HashMap<String, String>, error: String) -> Self {
        EventLogParams::ExternalCallFailed { service, method, params, error }
    }
}
