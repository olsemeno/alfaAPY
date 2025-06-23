use candid::Nat;

use crate::event_logs::event_log::EventLogParams;

pub struct EventLogParamsBuilder;
impl EventLogParamsBuilder {
    pub fn add_liquidity_to_pool_started() -> AddLiquidityToPoolStartedParamsBuilder { AddLiquidityToPoolStartedParamsBuilder::default() }
    pub fn add_liquidity_to_pool_completed() -> AddLiquidityToPoolCompletedParamsBuilder { AddLiquidityToPoolCompletedParamsBuilder::default() }
    pub fn add_liquidity_to_pool_failed() -> AddLiquidityToPoolFailedParamsBuilder { AddLiquidityToPoolFailedParamsBuilder::default() }
    pub fn withdraw_liquidity_from_pool_started() -> WithdrawLiquidityFromPoolStartedParamsBuilder { WithdrawLiquidityFromPoolStartedParamsBuilder::default() }
    pub fn withdraw_liquidity_from_pool_completed() -> WithdrawLiquidityFromPoolCompletedParamsBuilder { WithdrawLiquidityFromPoolCompletedParamsBuilder::default() }
    pub fn withdraw_liquidity_from_pool_failed() -> WithdrawLiquidityFromPoolFailedParamsBuilder { WithdrawLiquidityFromPoolFailedParamsBuilder::default() }
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
        EventLogParams::add_liquidity_to_pool_started(
            self.pool_id.expect("pool_id required"),
            self.amount0,
            self.amount1,
        )
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
        EventLogParams::add_liquidity_to_pool_completed(
            self.pool_id.expect("pool_id required"),
            self.amount0,
            self.amount1,
        )
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
        EventLogParams::add_liquidity_to_pool_failed(
            self.pool_id.expect("pool_id required"),
            self.amount0,
            self.amount1,
        )
    }
}

// === WithdrawLiquidityFromPoolStarted ===
#[derive(Default, Debug, Clone)]
pub struct WithdrawLiquidityFromPoolStartedParamsBuilder {
    pool_id: Option<String>,
    amount0: Option<Nat>,
    amount1: Option<Nat>,
}
impl WithdrawLiquidityFromPoolStartedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn amount1(mut self, amount: Nat) -> Self { self.amount1 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::withdraw_liquidity_from_pool_started(
            self.pool_id.expect("pool_id required"),
            self.amount0,
            self.amount1,
        )
    }
}

// === WithdrawLiquidityFromPoolCompleted ===
#[derive(Default, Debug, Clone)]
pub struct WithdrawLiquidityFromPoolCompletedParamsBuilder {
    pool_id: Option<String>,
    amount0: Option<Nat>,
    amount1: Option<Nat>,
}
impl WithdrawLiquidityFromPoolCompletedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn amount1(mut self, amount: Nat) -> Self { self.amount1 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::withdraw_liquidity_from_pool_completed(
            self.pool_id.expect("pool_id required"),
            self.amount0,
            self.amount1,
        )
    }
}

// === WithdrawLiquidityFromPoolFailed ===
#[derive(Default, Debug, Clone)]
pub struct WithdrawLiquidityFromPoolFailedParamsBuilder {
    pool_id: Option<String>,
    amount0: Option<Nat>,
    amount1: Option<Nat>,
}
impl WithdrawLiquidityFromPoolFailedParamsBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn amount0(mut self, amount: Nat) -> Self { self.amount0 = Some(amount); self }
    pub fn amount1(mut self, amount: Nat) -> Self { self.amount1 = Some(amount); self }
    pub fn build(self) -> EventLogParams {
        EventLogParams::withdraw_liquidity_from_pool_failed(
            self.pool_id.expect("pool_id required"),
            self.amount0,
            self.amount1,
        )
    }
}
