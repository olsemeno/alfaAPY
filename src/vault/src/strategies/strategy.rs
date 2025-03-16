use crate::liquidity::liquidity_service::{add_liquidity_to_pool, get_pools_data, to_tokens_info, withdraw_from_pool};
use crate::repo::repo::save_strategy;
use crate::strategies::basic_strategy::BasicStrategy;
use crate::strategies::calculator::Calculator;
use crate::strategies::strategy_candid::StrategyCandid;
use crate::swap::swap_service::swap_icrc2_kong;
use crate::swap::token_swaps::nat_to_u128;
use crate::types::types::{DepositResponse, RebalanceResponse, StrategyResponse, WithdrawResponse};
use crate::util::util::nat_to_f64;
use async_trait::async_trait;
use candid::{Nat, Principal};
use ic_cdk::{caller, trap};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use std::cell::RefMut;
use std::cmp::Ordering;

#[async_trait]
pub trait IStrategy: Send + Sync+  BasicStrategy  {
    fn update_user_shares(&mut self, user: Principal, shares: Nat) {
        let mut user_shares = self.get_user_shares();
        user_shares.insert(user, shares);
        self.set_user_shares(user_shares);
    }

    fn update_initial_deposit(&mut self, investor: Principal, new_shares: Nat) {
        let mut initial_deposit = self.get_initial_deposit();
        let user_deposit = initial_deposit.get(&investor).cloned().unwrap_or(Nat::from(0u64));
        // Remaining initial deposit proportional to the new shares
        let new_initial_deposit = user_deposit * new_shares.clone() / self.get_user_shares().get(&investor).unwrap().clone();
        initial_deposit.insert(investor.clone(), new_initial_deposit.clone());
        self.set_initial_deposit(initial_deposit);
    }

    async fn deposit(&mut self, investor: Principal, amount: Nat) -> DepositResponse {
        // TODO: remove this (added to setting current pool)
        let pools_data = get_pools_data(self.get_pools()).await;

        //TODO fixme temp approach to run the pool
        if self.get_current_pool().is_none() {
            self.set_current_pool(pools_data.iter()
                .find(|&x| x.symbol == "PANDA_ICP")
                .cloned());
        }

        if let Some(ref pool_reply) = self.get_current_pool() {

            // Calculate new shares for investor's deposit
            let new_shares = Calculator::calculate_shares(nat_to_f64(&amount), nat_to_f64(&self.get_total_balance()), nat_to_f64(&self.get_total_shares()));

            // Update total balance and total shares
            self.set_total_balance(self.get_total_balance() + amount.clone());
            self.set_total_shares(self.get_total_shares() + Nat::from(new_shares as u128));
            self.update_user_shares(investor, Nat::from(new_shares as u128));

            // Update initial deposit
            self.update_initial_deposit(investor, amount.clone());

            let resp = add_liquidity_to_pool(amount.clone(), pool_reply.clone()).await;

            save_strategy(self.clone_self());

            DepositResponse {
                amount: amount,
                shares: Nat::from(new_shares as u128),
                tx_id: 0,
                request_id: resp.request_id,
            }
        } else {
            trap("Rebalance");
        }
    }

    async fn withdraw(&mut self, shares: Nat) -> WithdrawResponse {
        let investor = caller();
        // Check if user has enough shares
        if let Some(user_shares) = self.get_user_shares().get(&investor) {
            if shares > *user_shares {
                trap("Not sufficient shares".into());
            }
        } else {
            trap("No shares found for this investor".into());
        }

        let pool = self.get_current_pool();

        if let Some(pool) = pool {
            let tokens_info = to_tokens_info(pool.clone());


            // trap(format!("shares: {:?}, total: {:?}", shares, self.get_total_shares()).as_str());
            // Remove liquidity from pool
            let withdraw_response = withdraw_from_pool(self.get_total_shares() ,shares.clone(),pool).await;

            // Swap token_1 to token_0 (to base token)
            let swap_response = swap_icrc2_kong(
                tokens_info.token_1,
                tokens_info.token_0.clone(),
                nat_to_f64(&withdraw_response.token_1_amount) as u128,
            ).await;

            // Calculate total token_0 to send after swap
            let amount_to_withdraw = withdraw_response.token_0_amount + swap_response.amount_out;

            let transfer_result = icrc1_transfer(
                tokens_info.token_0.ledger,
                &TransferArg {
                    from_subaccount: None,
                    to: Account {
                        owner: caller(),
                        subaccount: None,
                    },
                    fee: None,
                    created_at_time: None,
                    memo: None,
                    amount: amount_to_withdraw.clone(),
                },
            ).await;

            match transfer_result {
                Ok(Ok(x)) => x,
                Err(x) => {
                    trap(format!("Transfer error 1: {:?}", x.1).as_str());
                }
                Ok(Err(x)) => {
                    trap(format!("Transfer error 2: {:?}", x).as_str());
                }
            };

            // Update total shares
            let total_shares = self.get_total_shares() - shares.clone();
            self.set_total_shares(total_shares.clone());

            // Update user shares
            let shares_before_withdraw = self.get_user_shares().get(&investor).cloned().unwrap();
            let new_shares = shares_before_withdraw.clone() - shares;
            self.update_user_shares(investor.clone(), new_shares.clone());


            // Update initial deposit //TODO WIP - need to fix
            // let initial_deposit = self.get_initial_deposit().get(&investor).cloned().unwrap();
            // // Remaining initial deposit proportional to the new shares
            //
            // let new_initial_deposit = initial_deposit / shares_before_withdraw.clone() * new_shares.clone();
            // self.update_initial_deposit(investor.clone(), new_initial_deposit.clone());


            save_strategy(self.clone_self());

            WithdrawResponse {
                amount: amount_to_withdraw,
                current_shares: new_shares.clone(),
            }
        } else {
            trap("No current pool");
        }
    }

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
            initial_deposit: self.get_initial_deposit(),
        }
    }

    async fn rebalance(&mut self) -> RebalanceResponse {
        let pools_data = get_pools_data(Vec::from(self.get_pools())).await;
        let mut max_apy = 0.0;
        let mut max_apy_pool = None;

        // Find pool with highest APY
        for pool in pools_data {
            if pool.rolling_24h_apy > max_apy {
                max_apy = pool.rolling_24h_apy;
                max_apy_pool = Some(pool);
            }
        }
        let current_pool = self.get_current_pool();

        if let Some(max_pool) = max_apy_pool.clone() {
            // If current pool is the same as max APY pool, return
            if let Some(current_pool) = &current_pool {
                if current_pool.symbol == max_pool.symbol {
                    return RebalanceResponse {
                        pool: current_pool.clone(),
                    };
                }

                // Remove liquidity from current pool
                let withdraw_response = withdraw_from_pool(self.get_total_shares(), self.get_total_shares(), current_pool.clone()).await;

                let token_0_amount = withdraw_response.token_0_amount;
                let token_1_amount = withdraw_response.token_1_amount;

                let tokens_info = to_tokens_info(current_pool.clone());

                // Swap withdrawed token_1 to token_0 (to base token)
                let swap_response = swap_icrc2_kong(
                    tokens_info.token_1,
                    tokens_info.token_0,
                    nat_to_u128(token_1_amount)
                ).await;

                // Calculate total token_0 to send in new pool after swap
                let token_0_to_pool_amount = token_0_amount + swap_response.amount_out;

                // Add liquidity to new pool
                let _ = add_liquidity_to_pool(
                    token_0_to_pool_amount,
                    max_apy_pool.clone().unwrap()
                ).await;

                // Update current pool
                self.set_current_pool(Some(max_apy_pool.clone().unwrap()));

                RebalanceResponse {
                    pool: self.get_current_pool().unwrap(),
                }
            } else {
                trap("No current pool");
            }
        } else {
            RebalanceResponse {
                pool: self.get_current_pool().unwrap(),
            }
        }
    }

    fn clone_self(&self) -> Box<dyn IStrategy> ;
}

impl Clone for Box<dyn IStrategy> {
    fn clone(&self) -> Box<dyn IStrategy> {
        self.as_ref().clone_self()
    }
}

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

