use std::collections::HashMap;
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;

use types::CanisterId;
use crate::pools::pool::Pool;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct StrategyResponse {
    pub name: String,
    pub id: StrategyId,
    pub description: String,
    pub pools: Vec<Pool>,
    pub current_pool: Option<Pool>,
    pub total_balance: Nat,
    pub total_shares: Nat,
    pub user_shares: HashMap<Principal, Nat>,
    pub initial_deposit: HashMap<Principal, Nat>,
}


// TODO: rename to UserPositionResponse
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct UserStrategyResponse {
    pub strategy_id: StrategyId,
    pub strategy_name: String,
    pub strategy_current_pool: Pool,
    pub total_shares: Nat,
    pub user_shares: Nat,
    pub initial_deposit: Nat,
    pub users_count: u32,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct AcceptInvestmentArgs {
    pub ledger: CanisterId,
    pub amount: Nat,
    pub strategy_id: StrategyId,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Icrc28TrustedOriginsResponse {
    pub trusted_origins: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct WithdrawArgs {
    pub ledger: CanisterId,
    pub amount: Nat, // Rename to percentage
    pub strategy_id: StrategyId,
}

#[derive(CandidType, Deserialize, Eq, PartialEq, Debug)]
pub struct SupportedStandard {
    pub url: String,
    pub name: String,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct DepositResponse {
    pub amount: Nat,
    pub shares: Nat,
    pub tx_id: u64,
    pub request_id: u64,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct WithdrawResponse {
    pub amount: Nat,
    pub current_shares: Nat,
}

pub struct RebalanceResponse {
    pub previous_pool: Pool,
    pub current_pool: Pool,
    pub is_rebalanced: bool,
}

pub type StrategyId = u16;
