use std::collections::HashMap;
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;

use kongswap_canister::PoolReply;
use types::CanisterId;
use types::exchanges::TokenInfo;
use crate::events::event::{UserEvent, SystemEvent};

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
    pub amount: Nat, // TODO: rename to shares
    pub strategy_id: StrategyId,
}


#[derive(CandidType, Deserialize, Eq, PartialEq, Debug)]
pub struct SupportedStandard {
    pub url: String,
    pub name: String,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct UserStrategyResponse {
    pub strategy_id: StrategyId,
    pub strategy_name: String,
    pub strategy_current_pool: String,
    pub total_shares: Nat,
    pub user_shares: Nat,
    pub initial_deposit: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct Pool {
    pub pool_symbol: PoolSymbol,
    pub token0: String,
    pub token1: String,
    pub rolling_24h_apy: f64,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]

pub struct PoolNew {
    pub token0: TokenInfo,
    pub token1: TokenInfo,
}

// pub struct PoolReply {
//     pub pool_id: u32,
//     pub name: String,
//     pub symbol: String,
//     pub chain_0: String,
//     pub symbol_0: String,
//     pub address_0: String,
//     pub balance_0: Nat,
//     pub lp_fee_0: Nat,
//     pub chain_1: String,
//     pub symbol_1: String,
//     pub address_1: String,
//     pub balance_1: Nat,
//     pub lp_fee_1: Nat,
//     pub price: f64,
//     pub lp_fee_bps: u8,
//     pub tvl: Nat,
//     pub rolling_24h_volume: Nat,
//     pub rolling_24h_lp_fee: Nat,
//     pub rolling_24h_num_swaps: Nat,
//     pub rolling_24h_apy: f64,
//     pub lp_token_symbol: String,
//     pub is_removed: bool,
// }




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

pub enum EventResponse {
    User(UserEvent),
    System(SystemEvent),
}
