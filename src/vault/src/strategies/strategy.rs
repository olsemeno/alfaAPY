use candid::Nat;
use crate::liquidity::liquidity_service::get_pools_data;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use async_trait::async_trait;
use candid::{CandidType, Deserialize, Principal};
use ic_ledger_types::Subaccount;
use kongswap_canister::PoolReply;
use serde::Serialize;
use std::cell::RefMut;
use std::cmp::Ordering;
use crate::providers::kong::kong::{remove_liquidity, user_balances};
use crate::strategies::strategy_candid::StrategyCandid;
use types::exchanges::TokenInfo;
use std::collections::HashMap;

pub type PoolSymbol = String;
pub type StrategyId = u16;

#[async_trait]
pub trait IStrategy {
    fn get_name(&self) -> String;
    fn get_id(&self) -> StrategyId;
    fn get_description(&self) -> String;
    fn get_pools(&self) -> Vec<Pool>;
    fn get_subaccount(&self) -> Subaccount;
    fn get_current_pool(&self) -> Option<PoolReply>;
    fn get_pool_tokens_info(&self, pool: PoolReply) -> TokensInfo;
    fn get_user_shares(&self) -> HashMap<Principal, Nat>;
    fn get_total_shares(&self) -> Nat;
    fn clone_self(&self) -> Box<dyn IStrategy>;

    //TODO make generic kongswap/icpswap
    async fn get_pools_data(&self) -> Vec<PoolReply> {
        get_pools_data(self.get_pools()).await
    }
    async fn deposit(&mut self, investor: Principal, amount: Nat) -> DepositResponse;
    async fn withdraw(&mut self, investor: Principal, shares: Nat) -> WithdrawResponse;
    async fn rebalance(&mut self) -> RebalanceResponse;
    fn to_candid(&self) -> StrategyCandid;
    fn to_response(&self) -> StrategyResponse {
        StrategyResponse {
            name: self.get_name(),
            id: self.get_id(),
            description: self.get_description(),
            pools: self.get_pools().iter().map(|x| x.pool_symbol.clone()).collect(),
            current_pool: self.get_current_pool(),
            total_shares: self.get_total_shares(),
            user_shares: self.get_user_shares(),
        }
    }
    async fn withdraw_from_pool(&mut self, shares: Nat, pool: PoolReply) -> WithdrawFromPoolResponse;
    async fn add_liquidity_to_pool(&mut self, amount: Nat, pool: PoolReply) -> AddLiquidityResponse;
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct StrategyResponse {
    pub name: String,
    pub id: StrategyId,
    pub description: String,
    pub pools: Vec<PoolSymbol>,
    pub current_pool: Option<PoolReply>,
    pub total_shares: Nat,
    pub user_shares: HashMap<Principal, Nat>
}

pub struct Strategy {
    pub name: String,
    pub id: StrategyId,
    pub description: String,
    pub pools: Vec<PoolSymbol>,
    pub total_shares: Nat,
    pub user_shares: Nat,
    pub subaccount: Subaccount,
}

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

#[derive(CandidType, Deserialize, Clone,Debug, Serialize)]
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

impl Clone for Box<dyn IStrategy> {
    fn clone(&self) -> Box<dyn IStrategy> {
        self.as_ref().clone_self()
    }
}

pub trait StrategyClone: IStrategy + Clone {}

impl<T> StrategyClone for T where T: IStrategy + Clone {}

pub struct StrategyIterator<'a> {
    inner: RefMut<'a, Vec<Box<dyn IStrategy>>>,
    index: usize,
}

impl<'a> StrategyIterator<'a> {
    pub fn new(trs: RefMut<'a, Vec<Box<dyn IStrategy>>>) -> Self {
        StrategyIterator { inner: trs, index: 0 }
    }
}

impl<'a> Iterator for StrategyIterator<'a> {
    type Item = Box<dyn IStrategy>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.inner.len() {
            // Clone the item at the current index
            let cloned_item = self.inner[self.index].clone();
            self.index += 1;
            Some(cloned_item)
        } else {
            None
        }
    }
}

impl Eq for dyn IStrategy {}

impl PartialEq for dyn IStrategy {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Ord for dyn IStrategy {
    fn cmp(&self, other: &Self) -> Ordering {
        other.get_id().cmp(&self.get_id())
    }
}

impl PartialOrd for dyn IStrategy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.get_id().cmp(&self.get_id()))
    }
}
