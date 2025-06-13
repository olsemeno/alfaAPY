use candid::Nat;
use types::CanisterId;

use crate::event_logs::event_log::EventLogParams;

pub struct EventLogParamsBuilder;
impl EventLogParamsBuilder {
    pub fn strategy_deposit_started() -> StrategyDepositStartedParamsBuilder { StrategyDepositStartedParamsBuilder::default() }
    pub fn strategy_deposit_completed() -> StrategyDepositCompletedParamsBuilder { StrategyDepositCompletedParamsBuilder::default() }
    pub fn strategy_deposit_failed() -> StrategyDepositFailedParamsBuilder { StrategyDepositFailedParamsBuilder::default() }
    pub fn strategy_withdraw_started() -> StrategyWithdrawStartedParamsBuilder { StrategyWithdrawStartedParamsBuilder::default() }
    pub fn strategy_withdraw_completed() -> StrategyWithdrawCompletedParamsBuilder { StrategyWithdrawCompletedParamsBuilder::default() }
    pub fn strategy_withdraw_failed() -> StrategyWithdrawFailedParamsBuilder { StrategyWithdrawFailedParamsBuilder::default() }
    pub fn strategy_rebalance_started() -> StrategyRebalanceStartedParamsBuilder { StrategyRebalanceStartedParamsBuilder::default() }
    pub fn strategy_rebalance_completed() -> StrategyRebalanceCompletedParamsBuilder { StrategyRebalanceCompletedParamsBuilder::default() }
    pub fn strategy_rebalance_failed() -> StrategyRebalanceFailedParamsBuilder { StrategyRebalanceFailedParamsBuilder::default() }
    pub fn add_liquidity_to_pool_started() -> AddLiquidityToPoolStartedParamsBuilder { AddLiquidityToPoolStartedParamsBuilder::default() }
    pub fn add_liquidity_to_pool_completed() -> AddLiquidityToPoolCompletedParamsBuilder { AddLiquidityToPoolCompletedParamsBuilder::default() }
    pub fn add_liquidity_to_pool_failed() -> AddLiquidityToPoolFailedParamsBuilder { AddLiquidityToPoolFailedParamsBuilder::default() }
    pub fn remove_liquidity_from_pool_started() -> RemoveLiquidityFromPoolStartedParamsBuilder { RemoveLiquidityFromPoolStartedParamsBuilder::default() }
    pub fn remove_liquidity_from_pool_completed() -> RemoveLiquidityFromPoolCompletedParamsBuilder { RemoveLiquidityFromPoolCompletedParamsBuilder::default() }
    pub fn remove_liquidity_from_pool_failed() -> RemoveLiquidityFromPoolFailedParamsBuilder { RemoveLiquidityFromPoolFailedParamsBuilder::default() }
    pub fn swap_token_started() -> SwapTokenStartedParamsBuilder { SwapTokenStartedParamsBuilder::default() }
    pub fn swap_token_completed() -> SwapTokenCompletedParamsBuilder { SwapTokenCompletedParamsBuilder::default() }
    pub fn swap_token_failed() -> SwapTokenFailedParamsBuilder { SwapTokenFailedParamsBuilder::default() }
}

// === StrategyDepositStarted ===
#[derive(Default, Debug, Clone)]
pub struct StrategyDepositStartedParamsBuilder {
    strategy_id: Option<String>,
    pool_id: Option<String>,
    amount0: Option<Nat>,
}
impl StrategyDepositStartedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn strategy_id(mut self, id: impl Into<String>) -> Self { self.strategy_id = Some(id.into()); self }
    pub fn pool_id(mut self, id: Option<String>) -> Self { self.pool_id = id; self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::StrategyDepositStarted {
            strategy_id: self.strategy_id.expect("strategy_id required"),
            pool_id: self.pool_id,
            amount0: self.amount0,
        }
    }
}

// === StrategyDepositCompleted ===
#[derive(Default, Debug, Clone)]
pub struct StrategyDepositCompletedParamsBuilder {
    strategy_id: Option<String>,
    pool_id: Option<String>,
    amount0: Option<Nat>,
}
impl StrategyDepositCompletedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn strategy_id(mut self, id: impl Into<String>) -> Self { self.strategy_id = Some(id.into()); self }
    pub fn pool_id(mut self, id: Option<String>) -> Self { self.pool_id = id; self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::StrategyDepositCompleted {
            strategy_id: self.strategy_id.expect("strategy_id required"),
            pool_id: self.pool_id,
            amount0: self.amount0,
        }
    }
}

// === StrategyDepositFailed ===
#[derive(Default, Debug, Clone)]
pub struct StrategyDepositFailedParamsBuilder {
    strategy_id: Option<String>,
    pool_id: Option<String>,
    amount0: Option<Nat>,
}
impl StrategyDepositFailedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn strategy_id(mut self, id: impl Into<String>) -> Self { self.strategy_id = Some(id.into()); self }
    pub fn pool_id(mut self, id: Option<String>) -> Self { self.pool_id = id; self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::StrategyDepositFailed {
            strategy_id: self.strategy_id.expect("strategy_id required"),
            pool_id: self.pool_id,
            amount0: self.amount0,
        }
    }
}

// === StrategyWithdrawStarted ===
#[derive(Default, Debug, Clone)]
pub struct StrategyWithdrawStartedParamsBuilder {
    strategy_id: Option<String>,
    pool_id: Option<String>,
    shares: Option<Nat>,
}
impl StrategyWithdrawStartedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn strategy_id(mut self, id: impl Into<String>) -> Self { self.strategy_id = Some(id.into()); self }
    pub fn pool_id(mut self, id: Option<String>) -> Self { self.pool_id = id; self }
    pub fn shares(mut self, shares: Nat) -> Self { self.shares = Some(shares); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::StrategyWithdrawStarted {
            strategy_id: self.strategy_id.expect("strategy_id required"),
            pool_id: self.pool_id,
            shares: self.shares,
        }
    }
}

// === StrategyWithdrawCompleted ===
#[derive(Default, Debug, Clone)]
pub struct StrategyWithdrawCompletedParamsBuilder {
    strategy_id: Option<String>,
    pool_id: Option<String>,
    shares: Option<Nat>,
    amount0: Option<Nat>,
}
impl StrategyWithdrawCompletedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn strategy_id(mut self, id: impl Into<String>) -> Self { self.strategy_id = Some(id.into()); self }
    pub fn pool_id(mut self, id: Option<String>) -> Self { self.pool_id = id; self }
    pub fn shares(mut self, shares: Nat) -> Self { self.shares = Some(shares); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::StrategyWithdrawCompleted {
            strategy_id: self.strategy_id.expect("strategy_id required"),
            pool_id: self.pool_id,
            shares: self.shares,
            amount0: self.amount0,
        }
    }
}

// === StrategyWithdrawFailed ===
#[derive(Default, Debug, Clone)]
pub struct StrategyWithdrawFailedParamsBuilder {
    strategy_id: Option<String>,
    pool_id: Option<String>,
    shares: Option<Nat>,
}
impl StrategyWithdrawFailedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn strategy_id(mut self, id: impl Into<String>) -> Self { self.strategy_id = Some(id.into()); self }
    pub fn pool_id(mut self, id: Option<String>) -> Self { self.pool_id = id; self }
    pub fn shares(mut self, shares: Nat) -> Self { self.shares = Some(shares); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::StrategyWithdrawFailed {
            strategy_id: self.strategy_id.expect("strategy_id required"),
            pool_id: self.pool_id,
            shares: self.shares,
        }
    }
}

// === StrategyRebalanceStarted ===
#[derive(Default, Debug, Clone)]
pub struct StrategyRebalanceStartedParamsBuilder {
    strategy_id: Option<String>,
    previous_pool_id: Option<String>,
}
impl StrategyRebalanceStartedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn strategy_id(mut self, id: impl Into<String>) -> Self { self.strategy_id = Some(id.into()); self }
    pub fn previous_pool_id(mut self, id: Option<String>) -> Self { self.previous_pool_id = id; self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::StrategyRebalanceStarted {
            strategy_id: self.strategy_id.expect("strategy_id required"),
            previous_pool_id: self.previous_pool_id,
        }
    }
}

// === StrategyRebalanceCompleted ===
#[derive(Default, Debug, Clone)]
pub struct StrategyRebalanceCompletedParamsBuilder {
    strategy_id: Option<String>,
    previous_pool_id: Option<String>,
    new_pool_id: Option<String>,
}
impl StrategyRebalanceCompletedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn strategy_id(mut self, id: impl Into<String>) -> Self { self.strategy_id = Some(id.into()); self }
    pub fn previous_pool_id(mut self, id: Option<String>) -> Self { self.previous_pool_id = id; self }
    pub fn new_pool_id(mut self, id: Option<String>) -> Self { self.new_pool_id = id; self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::StrategyRebalanceCompleted {
            strategy_id: self.strategy_id.expect("strategy_id required"),
            previous_pool_id: self.previous_pool_id,
            new_pool_id: self.new_pool_id,
        }
    }
}

// === StrategyRebalanceFailed ===
#[derive(Default, Debug, Clone)]
pub struct StrategyRebalanceFailedParamsBuilder {
    strategy_id: Option<String>,
    previous_pool_id: Option<String>,
    new_pool_id: Option<String>,
}
impl StrategyRebalanceFailedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn strategy_id(mut self, id: impl Into<String>) -> Self { self.strategy_id = Some(id.into()); self }
    pub fn previous_pool_id(mut self, id: Option<String>) -> Self { self.previous_pool_id = id; self }
    pub fn new_pool_id(mut self, id: Option<String>) -> Self { self.new_pool_id = id; self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::StrategyRebalanceFailed {
            strategy_id: self.strategy_id.expect("strategy_id required"),
            previous_pool_id: self.previous_pool_id,
            new_pool_id: self.new_pool_id,
        }
    }
}

// === AddLiquidityToPoolStarted ===
#[derive(Default, Debug, Clone)]
pub struct AddLiquidityToPoolStartedParamsBuilder {
    pool_id: Option<String>,
    amount0: Option<Nat>,
    amount1: Option<Nat>,
}
impl AddLiquidityToPoolStartedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn amount1(mut self, amount: Nat) -> Self { self.amount1 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::AddLiquidityToPoolStarted {
            pool_id: self.pool_id.expect("pool_id required"),
            amount0: self.amount0,
            amount1: self.amount1,
        }
    }
}

// === AddLiquidityToPoolCompleted ===
#[derive(Default, Debug, Clone)]
pub struct AddLiquidityToPoolCompletedParamsBuilder {
    pool_id: Option<String>,
    amount0: Option<Nat>,
    amount1: Option<Nat>,
}
impl AddLiquidityToPoolCompletedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn amount1(mut self, amount: Nat) -> Self { self.amount1 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::AddLiquidityToPoolCompleted {
            pool_id: self.pool_id.expect("pool_id required"),
            amount0: self.amount0,
            amount1: self.amount1,
        }
    }
}

// === AddLiquidityToPoolFailed ===
#[derive(Default, Debug, Clone)]
pub struct AddLiquidityToPoolFailedParamsBuilder {
    pool_id: Option<String>,
    amount0: Option<Nat>,
    amount1: Option<Nat>,
}
impl AddLiquidityToPoolFailedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn amount1(mut self, amount: Nat) -> Self { self.amount1 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::AddLiquidityToPoolFailed {
            pool_id: self.pool_id.expect("pool_id required"),
            amount0: self.amount0,
            amount1: self.amount1,
        }
    }
}

// === RemoveLiquidityFromPoolStarted ===
#[derive(Default, Debug, Clone)]
pub struct RemoveLiquidityFromPoolStartedParamsBuilder {
    pool_id: Option<String>,
    amount0: Option<Nat>,
    amount1: Option<Nat>,
}
impl RemoveLiquidityFromPoolStartedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn amount1(mut self, amount: Nat) -> Self { self.amount1 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::RemoveLiquidityFromPoolStarted {
            pool_id: self.pool_id.expect("pool_id required"),
            amount0: self.amount0,
            amount1: self.amount1,
        }
    }
}

// === RemoveLiquidityFromPoolCompleted ===
#[derive(Default, Debug, Clone)]
pub struct RemoveLiquidityFromPoolCompletedParamsBuilder {
    pool_id: Option<String>,
    amount0: Option<Nat>,
    amount1: Option<Nat>,
}
impl RemoveLiquidityFromPoolCompletedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn amount1(mut self, amount: Nat) -> Self { self.amount1 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::RemoveLiquidityFromPoolCompleted {
            pool_id: self.pool_id.expect("pool_id required"),
            amount0: self.amount0,
            amount1: self.amount1,
        }
    }
}

// === RemoveLiquidityFromPoolFailed ===
#[derive(Default, Debug, Clone)]
pub struct RemoveLiquidityFromPoolFailedParamsBuilder {
    pool_id: Option<String>,
    amount0: Option<Nat>,
    amount1: Option<Nat>,
}
impl RemoveLiquidityFromPoolFailedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn amount1(mut self, amount: Nat) -> Self { self.amount1 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::RemoveLiquidityFromPoolFailed {
            pool_id: self.pool_id.expect("pool_id required"),
            amount0: self.amount0,
            amount1: self.amount1,
        }
    }
}

// === SwapTokenStarted ===
#[derive(Default, Debug, Clone)]
pub struct SwapTokenStartedParamsBuilder {
    pool_id: Option<String>,
    token_in: Option<CanisterId>,
    token_out: Option<CanisterId>,
    amount_in: Option<Nat>,
}
impl SwapTokenStartedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn token_in(mut self, token: CanisterId) -> Self { self.token_in = Some(token); self }
    pub fn token_out(mut self, token: CanisterId) -> Self { self.token_out = Some(token); self }
    pub fn amount_in(mut self, amount: Nat) -> Self { self.amount_in = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::SwapTokenStarted {
            pool_id: self.pool_id.expect("pool_id required"),
            token_in: self.token_in.expect("token_in required"),
            token_out: self.token_out.expect("token_out required"),
            amount_in: self.amount_in,
        }
    }
}

// === SwapTokenCompleted ===
#[derive(Default, Debug, Clone)]
pub struct SwapTokenCompletedParamsBuilder {
    token_in: Option<CanisterId>,
    token_out: Option<CanisterId>,
    amount_in: Option<Nat>,
    amount_out: Option<Nat>,
}
impl SwapTokenCompletedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn token_in(mut self, token: CanisterId) -> Self { self.token_in = Some(token); self }
    pub fn token_out(mut self, token: CanisterId) -> Self { self.token_out = Some(token); self }
    pub fn amount_in(mut self, amount: Nat) -> Self { self.amount_in = Some(amount); self }
    pub fn amount_out(mut self, amount: Nat) -> Self { self.amount_out = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::SwapTokenCompleted {
            token_in: self.token_in.expect("token_in required"),
            token_out: self.token_out.expect("token_out required"),
            amount_in: self.amount_in,
            amount_out: self.amount_out,
        }
    }
}

// === SwapTokenFailed ===
#[derive(Default, Debug, Clone)]
pub struct SwapTokenFailedParamsBuilder {
    pool_id: Option<String>,
    token_in: Option<CanisterId>,
    token_out: Option<CanisterId>,
    amount_in: Option<Nat>,
}
impl SwapTokenFailedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn token_in(mut self, token: CanisterId) -> Self { self.token_in = Some(token); self }
    pub fn token_out(mut self, token: CanisterId) -> Self { self.token_out = Some(token); self }
    pub fn amount_in(mut self, amount: Nat) -> Self { self.amount_in = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::SwapTokenFailed {
            pool_id: self.pool_id.expect("pool_id required"),
            token_in: self.token_in.expect("token_in required"),
            token_out: self.token_out.expect("token_out required"),
            amount_in: self.amount_in,
        }
    }
}
