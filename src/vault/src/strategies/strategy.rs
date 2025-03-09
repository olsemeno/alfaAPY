use crate::liquidity::liquidity_service::get_pools_data;
use crate::strategies::r#impl::ck_btc_strategy::ckBTCStrategy;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use async_trait::async_trait;
use candid::{CandidType, Deserialize};
use ic_ledger_types::Subaccount;
use kongswap_canister::PoolReply;
use serde::Serialize;
use std::cell::RefMut;
use std::cmp::Ordering;
use crate::strategies::strategy_candid::StrategyCandid;

pub type PoolSymbol = String;
pub type StrategyId = u16;

#[async_trait]
pub trait IStrategy {
    fn get_name(&self) -> String;
    fn get_id(&self) -> StrategyId;
    fn get_description(&self) -> String;
    fn get_pools(&self) -> Vec<Pool>;
    fn get_subaccount(&self) -> Subaccount;
    fn get_current_pool(&self) -> PoolReply;
    fn clone_self(&self) -> Box<dyn IStrategy>;
    //TODO make generic kongswap/icpswap
    async fn get_pools_data(&self) -> Vec<PoolReply> {
        get_pools_data(self.get_pools()).await
    }
    async fn rebalance(&self) -> PoolReply;
    fn to_candid(&self) -> StrategyCandid;
}

pub struct Strategy {
    pub name: String,
    pub id: StrategyId,
    pub description: String,
    pub pools: Vec<PoolSymbol>,
    pub subaccount: Subaccount,
}

pub struct Pool {
    pub pool_symbol: PoolSymbol,
    pub token0: String,
    pub token1: String,
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

