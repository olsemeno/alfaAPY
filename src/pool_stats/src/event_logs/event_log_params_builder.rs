use candid::Nat;

use crate::event_logs::event_log::EventLogParams;

pub struct EventLogParamsBuilder;
impl EventLogParamsBuilder {
    pub fn add_liquidity_to_pool_started() -> AddLiquidityToPoolStartedParamsBuilder { AddLiquidityToPoolStartedParamsBuilder::default() }
    pub fn add_liquidity_to_pool_completed() -> AddLiquidityToPoolCompletedParamsBuilder { AddLiquidityToPoolCompletedParamsBuilder::default() }
    pub fn add_liquidity_to_pool_failed() -> AddLiquidityToPoolFailedParamsBuilder { AddLiquidityToPoolFailedParamsBuilder::default() }
    pub fn remove_liquidity_from_pool_started() -> RemoveLiquidityFromPoolStartedParamsBuilder { RemoveLiquidityFromPoolStartedParamsBuilder::default() }
    pub fn remove_liquidity_from_pool_completed() -> RemoveLiquidityFromPoolCompletedParamsBuilder { RemoveLiquidityFromPoolCompletedParamsBuilder::default() }
    pub fn remove_liquidity_from_pool_failed() -> RemoveLiquidityFromPoolFailedParamsBuilder { RemoveLiquidityFromPoolFailedParamsBuilder::default() }
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
            amount0: self.amount0.expect("amount0 required"),
            amount1: self.amount1.expect("amount1 required"),
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
            amount0: self.amount0.expect("amount0 required"),
            amount1: self.amount1.expect("amount1 required"),
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
            amount0: self.amount0.expect("amount0 required"),
            amount1: self.amount1.expect("amount1 required"),
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
            amount0: self.amount0.expect("amount0 required"),
            amount1: self.amount1.expect("amount1 required"),
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
            amount0: self.amount0.expect("amount0 required"),
            amount1: self.amount1.expect("amount1 required"),
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
            amount0: self.amount0.expect("amount0 required"),
            amount1: self.amount1.expect("amount1 required"),
        }
    }
}
