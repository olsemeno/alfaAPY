use std::collections::HashMap;
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use kongswap_canister::PoolReply;
use types::exchanges::TokenInfo;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct StrategyResponse {
    pub name: String,
    pub id: StrategyId,
    pub description: String,
    pub pools: Vec<PoolSymbol>,
    pub current_pool: Option<PoolReply>,
    pub total_shares: Nat,
    pub user_shares: HashMap<Principal, Nat>,
    pub initial_deposit: HashMap<Principal, Nat>,
}


#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Pool {
    pub pool_symbol: PoolSymbol,
    pub token0: String,
    pub token1: String,
    pub rolling_24h_apy: f64,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
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

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct WithdrawFromPoolResponse {
    pub token_0_amount: Nat,
    pub token_1_amount: Nat,
}

pub struct AddLiquidityResponse {
    pub token_0_amount: Nat,
    pub token_1_amount: Nat,
    pub request_id: u64,
}

pub struct RebalanceResponse {
    pub pool: PoolReply,
}

pub struct TokensInfo {
    pub token_0: TokenInfo,
    pub token_1: TokenInfo,
}

pub type PoolSymbol = String;
pub type StrategyId = u16;
