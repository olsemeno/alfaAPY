use std::collections::HashMap;
use candid::{Nat, Principal};
use crate::types::types::StrategyId;
use crate::pools::pool::Pool;

pub trait BasicStrategy {
    fn get_name(&self) -> String;
    fn get_id(&self) -> StrategyId;
    fn get_description(&self) -> String;
    fn get_pools(&self) -> Vec<Pool>;
    fn get_total_shares(&self) -> Nat;
    fn set_total_shares(&mut self, total_shares: Nat);
    fn get_total_balance(&self) -> Nat;
    fn set_total_balance(&mut self, total_balance: Nat);
    fn get_current_pool(&self) -> Option<Pool>;
    fn set_current_pool(&mut self, pool: Option<Pool>);
    fn get_user_shares(&self) -> HashMap<Principal, Nat>;
    fn set_user_shares(&mut self, user_shares: HashMap<Principal, Nat>);
    fn get_initial_deposit(&self) -> HashMap<Principal, Nat>;
    fn set_initial_deposit(&mut self, map: HashMap<Principal, Nat>);
    fn get_users_count(&self) -> u32;
    fn get_user_shares_by_principal(&self, principal: Principal) -> Nat;
    fn get_position_id(&self) -> Option<u64>;
    fn set_position_id(&mut self, position_id: Option<u64>);
    fn get_current_liquidity(&self) -> Option<Nat>;
    fn set_current_liquidity(&mut self, current_liquidity: Option<Nat>);
    fn get_current_liquidity_updated_at(&self) -> Option<u64>;
    fn set_current_liquidity_updated_at(&mut self, current_liquidity_updated_at: Option<u64>);
}

#[macro_export]
macro_rules! impl_strategy_methods {
    ($type:ty) => {
        #[async_trait]
        impl BasicStrategy for $type {
            fn get_name(&self) -> String {
                STRATEGY_MAP.get(&self.id).unwrap().name.clone()
            }

            fn get_id(&self) -> StrategyId {
                self.id
            }

            fn get_description(&self) -> String {
                STRATEGY_MAP.get(&self.id).unwrap().description.clone()
            }

            fn get_pools(&self) -> Vec<Pool> {
                STRATEGY_MAP.get(&self.id).unwrap().pools.clone()
            }

            fn get_total_shares(&self) -> Nat {
                self.total_shares.clone()
            }

            fn set_total_shares(&mut self, total_shares: Nat) {
                self.total_shares = total_shares;
            }

            fn get_total_balance(&self) -> Nat {
                self.total_balance.clone()
            }

            fn set_total_balance(&mut self, total_balance: Nat) {
                self.total_balance = total_balance;
            }

            fn get_current_pool(&self) -> Option<Pool> {
                self.current_pool.clone()
            }

            fn set_current_pool(&mut self, pool: Option<Pool>) {
                self.current_pool = pool;
            }

            fn get_user_shares(&self) -> HashMap<Principal, Nat> {
                self.user_shares.clone()
            }

            fn set_user_shares(&mut self, user_shares: HashMap<Principal, Nat>) {
                self.user_shares = user_shares;
            }

            fn get_initial_deposit(&self) -> HashMap<Principal, Nat> {
                self.initial_deposit.clone()
            }

            fn set_initial_deposit(&mut self, map: HashMap<Principal, Nat>) {
                self.initial_deposit = map;
            }

            fn get_users_count(&self) -> u32 {
                self.user_shares.len() as u32
            }

            fn get_user_shares_by_principal(&self, principal: Principal) -> Nat {
                self.user_shares.get(&principal).unwrap().clone()
            }

            fn get_position_id(&self) -> Option<u64> {
                self.position_id.clone()
            }

            fn set_position_id(&mut self, position_id: Option<u64>) {
                self.position_id = position_id;
            }

            fn get_current_liquidity(&self) -> Option<Nat> {
                self.current_liquidity.clone()
            }

            fn get_current_liquidity_updated_at(&self) -> Option<u64> {
                self.current_liquidity_updated_at.clone()
            }

            fn set_current_liquidity(&mut self, current_liquidity: Option<Nat>) {
                self.current_liquidity = current_liquidity;
            }

            fn set_current_liquidity_updated_at(&mut self, current_liquidity_updated_at: Option<u64>) {
                self.current_liquidity_updated_at = current_liquidity_updated_at;
            }
        }
    };
}