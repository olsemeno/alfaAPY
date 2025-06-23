use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;
use types::CanisterId;

use event_records::generic_event_record::GenericEventRecord;
use event_records::events::pool_events::*;

use crate::event_records::events::strategy_events::*;
use crate::event_records::events::swap_events::*;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EventRecord(pub GenericEventRecord<Event>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum Event {
    // Strategy Deposit
    StrategyDepositStarted(StrategyDepositStarted),
    StrategyDepositCompleted(StrategyDepositCompleted),
    StrategyDepositFailed(StrategyDepositFailed),
    // Strategy Withdraw
    StrategyWithdrawStarted(StrategyWithdrawStarted),
    StrategyWithdrawCompleted(StrategyWithdrawCompleted),
    StrategyWithdrawFailed(StrategyWithdrawFailed),
    // Strategy Rebalance
    StrategyRebalanceStarted(StrategyRebalanceStarted),
    StrategyRebalanceCompleted(StrategyRebalanceCompleted),
    StrategyRebalanceFailed(StrategyRebalanceFailed),
    // Add liquidity to pool
    AddLiquidityToPoolStarted(AddLiquidityToPoolStarted),
    AddLiquidityToPoolCompleted(AddLiquidityToPoolCompleted),
    AddLiquidityToPoolFailed(AddLiquidityToPoolFailed),
    // Withdraw liquidity from pool
    WithdrawLiquidityFromPoolStarted(WithdrawLiquidityFromPoolStarted),
    WithdrawLiquidityFromPoolCompleted(WithdrawLiquidityFromPoolCompleted),
    WithdrawLiquidityFromPoolFailed(WithdrawLiquidityFromPoolFailed),
    // Swap token
    SwapTokenStarted(SwapTokenStarted),
    SwapTokenCompleted(SwapTokenCompleted),
    SwapTokenFailed(SwapTokenFailed),
}

impl Event {
    pub fn type_str(&self) -> &'static str {
        match self {
            // Strategy Deposit
            Self::StrategyDepositStarted(_) => "StrategyDepositStarted",
            Self::StrategyDepositCompleted(_) => "StrategyDepositCompleted",
            Self::StrategyDepositFailed(_) => "StrategyDepositFailed",
            // Strategy Withdraw
            Self::StrategyWithdrawStarted(_) => "StrategyWithdrawStarted",
            Self::StrategyWithdrawCompleted(_) => "StrategyWithdrawCompleted",
            Self::StrategyWithdrawFailed(_) => "StrategyWithdrawFailed",
            // Strategy Rebalance
            Self::StrategyRebalanceStarted(_) => "StrategyRebalanceStarted",
            Self::StrategyRebalanceCompleted(_) => "StrategyRebalanceCompleted",
            Self::StrategyRebalanceFailed(_) => "StrategyRebalanceFailed",
            // Add liquidity to pool
            Self::AddLiquidityToPoolStarted(_) => "AddLiquidityToPoolStarted",
            Self::AddLiquidityToPoolCompleted(_) => "AddLiquidityToPoolCompleted",
            Self::AddLiquidityToPoolFailed(_) => "AddLiquidityToPoolFailed",
            // Remove liquidity from pool
            Self::WithdrawLiquidityFromPoolStarted(_) => "WithdrawLiquidityFromPoolStarted",
            Self::WithdrawLiquidityFromPoolCompleted(_) => "WithdrawLiquidityFromPoolCompleted",
            Self::WithdrawLiquidityFromPoolFailed(_) => "WithdrawLiquidityFromPoolFailed",
            // Swap token
            Self::SwapTokenStarted(_) => "SwapTokenStarted",
            Self::SwapTokenCompleted(_) => "SwapTokenCompleted",
            Self::SwapTokenFailed(_) => "SwapTokenFailed",
        }
    }

    pub fn strategy_deposit_started(strategy_id: String, pool_id: Option<String>, amount0: Option<Nat>) -> Self {
        Self::StrategyDepositStarted(StrategyDepositStarted { strategy_id, pool_id, amount0 })
    }

    pub fn strategy_deposit_completed(strategy_id: String, pool_id: Option<String>, amount0: Option<Nat>) -> Self {
        Self::StrategyDepositCompleted(StrategyDepositCompleted { strategy_id, pool_id, amount0 })
    }

    pub fn strategy_deposit_failed(strategy_id: String, pool_id: Option<String>, amount0: Option<Nat>) -> Self {
        Self::StrategyDepositFailed(StrategyDepositFailed { strategy_id, pool_id, amount0 })
    }
    
    pub fn strategy_withdraw_started(strategy_id: String, pool_id: Option<String>, shares: Option<Nat>) -> Self {
        Self::StrategyWithdrawStarted(StrategyWithdrawStarted { strategy_id, pool_id, shares })
    }

    pub fn strategy_withdraw_completed(strategy_id: String, pool_id: Option<String>, shares: Option<Nat>, amount0: Option<Nat>) -> Self {
        Self::StrategyWithdrawCompleted(StrategyWithdrawCompleted { strategy_id, pool_id, shares, amount0 })
    }

    pub fn strategy_withdraw_failed(strategy_id: String, pool_id: Option<String>, shares: Option<Nat>) -> Self {
        Self::StrategyWithdrawFailed(StrategyWithdrawFailed { strategy_id, pool_id, shares })
    }
    
    pub fn strategy_rebalance_started(strategy_id: String, previous_pool_id: Option<String>) -> Self {
        Self::StrategyRebalanceStarted(StrategyRebalanceStarted { strategy_id, previous_pool_id })
    }

    pub fn strategy_rebalance_completed(strategy_id: String, previous_pool_id: Option<String>, new_pool_id: Option<String>) -> Self {
        Self::StrategyRebalanceCompleted(StrategyRebalanceCompleted { strategy_id, previous_pool_id, new_pool_id })
    }

    pub fn strategy_rebalance_failed(strategy_id: String, previous_pool_id: Option<String>, new_pool_id: Option<String>) -> Self {
        Self::StrategyRebalanceFailed(StrategyRebalanceFailed { strategy_id, previous_pool_id, new_pool_id })
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

    pub fn swap_token_started(pool_id: String, token_in: CanisterId, token_out: CanisterId, amount_in: Option<Nat>) -> Self {
        Self::SwapTokenStarted(SwapTokenStarted { pool_id, token_in, token_out, amount_in })
    }

    pub fn swap_token_completed(token_in: CanisterId, token_out: CanisterId, amount_in: Option<Nat>, amount_out: Option<Nat>) -> Self {
        Self::SwapTokenCompleted(SwapTokenCompleted { token_in, token_out, amount_in, amount_out })
    }

    pub fn swap_token_failed(pool_id: String, token_in: CanisterId, token_out: CanisterId, amount_in: Option<Nat>) -> Self {
        Self::SwapTokenFailed(SwapTokenFailed { pool_id, token_in, token_out, amount_in })
    }
}
