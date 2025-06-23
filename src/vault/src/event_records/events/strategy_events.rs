use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

// Strategy Deposit
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyDepositStarted {
    pub strategy_id: String,
    pub pool_id: Option<String>,
    pub amount0: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyDepositCompleted {
    pub strategy_id: String,
    pub pool_id: Option<String>,
    pub amount0: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyDepositFailed {
    pub strategy_id: String,
    pub pool_id: Option<String>,
    pub amount0: Option<Nat>,
}

// Strategy Withdraw
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyWithdrawStarted {
    pub strategy_id: String,
    pub pool_id: Option<String>,
    pub shares: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyWithdrawCompleted {
    pub strategy_id: String,
    pub pool_id: Option<String>,
    pub shares: Option<Nat>,
    pub amount0: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyWithdrawFailed {
    pub strategy_id: String,
    pub pool_id: Option<String>,
    pub shares: Option<Nat>,
}

// Strategy Rebalance
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyRebalanceStarted {
    pub strategy_id: String,
    pub previous_pool_id: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyRebalanceCompleted {
    pub strategy_id: String,
    pub previous_pool_id: Option<String>,
    pub new_pool_id: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyRebalanceFailed {
    pub strategy_id: String,
    pub previous_pool_id: Option<String>,
    pub new_pool_id: Option<String>,
}
