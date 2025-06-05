use async_trait::async_trait;
use candid::{Nat, Principal};
use ic_cdk::{caller, trap};
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use std::cell::RefMut;
use std::cmp::Ordering;

use swap::swap_service;
use swap::token_swaps::nat_to_u128;
use utils::util::nat_to_f64;
use liquidity::liquidity_calculator::LiquidityCalculator;
use types::exchange_id::ExchangeId;

use crate::enums::{SystemEventParams, UserEventParams};
use crate::events::event_service;
use crate::liquidity::liquidity_service::{
    add_liquidity_to_pool,
    get_pools_data,
    withdraw_liquidity_from_pool,
};
use crate::repository::strategies_repo::save_strategy;
use crate::strategies::basic_strategy::BasicStrategy;
use crate::strategies::strategy_candid::StrategyCandid;
use crate::types::types::{DepositResponse, RebalanceResponse, StrategyResponse, WithdrawResponse};

#[async_trait]
pub trait IStrategy: Send + Sync + BasicStrategy {
    /// Updates the shares owned by a specific user in the strategy
    ///
    /// # Arguments
    ///
    /// * `user` - The Principal ID of the user whose shares are being updated
    /// * `shares` - The new total number of shares for this user
    ///
    /// # Details
    ///
    /// This function:
    /// 1. Gets the current user shares mapping
    /// 2. Updates or inserts the new share amount for the specified user
    /// 3. Saves the updated mapping back to the strategy state
    fn update_user_shares(&mut self, user: Principal, shares: Nat) {
        let mut user_shares = self.get_user_shares();
        user_shares.insert(user, shares);
        self.set_user_shares(user_shares);
    }

    /// Updates the initial deposit amount for an investor based on their new share allocation
    ///
    /// # Arguments
    ///
    /// * `investor` - The Principal ID of the investor whose initial deposit is being updated
    /// * `new_shares` - The new number of shares owned by the investor
    ///
    /// # Details
    ///
    /// This function:
    /// 1. Gets the current initial deposit mapping
    /// 2. Retrieves the investor's current deposit amount (defaults to 0 if none exists)
    /// 3. Calculates the new initial deposit proportional to the new shares
    /// 4. Updates the initial deposit mapping with the new amount
    fn update_initial_deposit(&mut self, investor: Principal, new_shares: Nat) {
        let mut initial_deposit = self.get_initial_deposit();
        let user_deposit = initial_deposit
            .get(&investor)
            .cloned()
            .unwrap_or(Nat::from(0u64));
        // Remaining initial deposit proportional to the new shares
        let new_initial_deposit = user_deposit * new_shares.clone()
            / self.get_user_shares().get(&investor).unwrap().clone();
        initial_deposit.insert(investor.clone(), new_initial_deposit.clone());
        self.set_initial_deposit(initial_deposit);
    }

    /// Deposits an amount of tokens into the strategy
    ///
    /// # Arguments
    ///
    /// * `investor` - The Principal ID of the investor who is depositing tokens
    /// * `amount` - The amount of tokens to deposit
    ///
    /// # Returns
    ///
    /// A `DepositResponse` struct containing the following fields:
    /// - `amount`: The amount of tokens deposited
    /// - `shares`: The number of shares received
    /// - `tx_id`: The transaction ID (always 0 for this implementation)
    /// - `request_id`: The request ID from the deposit call
    ///
    /// # Details
    ///
    /// This function:
    /// 1. Retrieves the current pool from the strategy
    /// 2. Calculates the new shares for the investor's deposit
    /// 3. Updates the total balance and total shares
    /// 4. Updates the user shares mapping
    /// 5. Updates the initial deposit mapping
    /// 6. Adds liquidity to the pool
    /// 7. Saves the updated strategy state
    ///
    async fn deposit(&mut self, investor: Principal, amount: Nat) -> DepositResponse {
        let pools_data = get_pools_data(self.get_pools()).await;

        // TODO: remove this
        self.set_current_pool(None);

        // Set current pool to the best APY pool if not set
        if self.get_current_pool().is_none() {
            let best_apy_pool = pools_data
                .iter()
                .filter(|x| x.pool.provider == ExchangeId::KongSwap)
                // .max_by_key(|x| x.apy)
                .map(|x| x.pool.clone())
                .next();

            if let Some(pool) = best_apy_pool {
                self.set_current_pool(Some(pool));
            } else {
                trap("No pool found to deposit");
            }
        }

        if let Some(ref current_pool) = self.get_current_pool() {
            // Calculate new shares for investor's deposit
            let new_shares = LiquidityCalculator::calculate_shares(
                nat_to_f64(&amount),
                nat_to_f64(&self.get_total_balance()),
                nat_to_f64(&self.get_total_shares()),
            );

            // Update total balance and total shares
            self.set_total_balance(self.get_total_balance() + amount.clone());
            self.set_total_shares(self.get_total_shares() + Nat::from(new_shares as u128));
            self.update_user_shares(investor, Nat::from(new_shares as u128));

            // Update initial deposit
            self.update_initial_deposit(investor, amount.clone());

            // Add liquidity to pool
            let add_liquidity_response = add_liquidity_to_pool(
                amount.clone(),
                current_pool.clone()
            ).await;

            save_strategy(self.clone_self());

            // Create event for deposit
            event_service::create_user_event(
                UserEventParams::AddLiquidity {
                    amount: amount.clone(),
                    token: current_pool.token0.ledger.to_text(),
                    symbol: current_pool.token0.symbol.clone(),
                },
                investor,
            );

            DepositResponse {
                amount: amount,
                shares: self.get_user_shares().get(&investor).unwrap().clone(),
                tx_id: 0,
                request_id: add_liquidity_response.request_id,
            }
        } else {
            trap("No pool found to deposit");
        }
    }

    /// Withdraws shares from the strategy and returns the corresponding tokens to the investor
    ///
    /// # Arguments
    ///
    /// * `shares` - The number of shares to withdraw
    ///
    /// # Returns
    ///
    /// * `WithdrawResponse` - Contains the amount of tokens withdrawn and remaining shares
    ///
    /// # Details
    ///
    /// This function:
    /// 1. Verifies the caller has sufficient shares
    /// 2. Gets the current pool and token information
    /// 3. Removes liquidity from the pool proportional to shares
    /// 4. Swaps secondary token to base token
    /// 5. Transfers total tokens to caller
    /// 6. Updates total shares, user shares and initial deposit
    /// 7. Saves updated strategy state
    ///
    async fn withdraw(&mut self, shares: Nat) -> WithdrawResponse {
        let user = caller();

        // Check if user has enough shares
        if let Some(user_shares) = self.get_user_shares().get(&user) {
            if shares > *user_shares {
                trap("Not sufficient shares".into());
            }
        } else {
            trap("No shares found for this user".into());
        }

        if let Some(current_pool) = self.get_current_pool() {
            let token0 = current_pool.token0.clone();
            let token1 = current_pool.token1.clone();

            // Remove liquidity from pool
            let withdraw_response = withdraw_liquidity_from_pool(
                self.get_total_shares(),
                shares.clone(),
                current_pool.clone(),
            )
            .await;

            // Swap token_1 to token_0 (base token)
            let swap_response = swap_service::swap_icrc2_optimal(
                token1.clone(),
                token0.clone(),
                nat_to_f64(&withdraw_response.token_1_amount) as u128,
            ).await;

            // Amount to withdraw is the sum of token_0 amount and token_1 amount after swap
            let amount_0_to_withdraw = withdraw_response.token_0_amount + swap_response.amount_out;

            // Transfer token_0 (base token) to user
            let transfer_result = icrc1_transfer(
                token0.ledger,
                &TransferArg {
                    from_subaccount: None,
                    to: Account {
                        owner: caller(),
                        subaccount: None,
                    },
                    fee: None,
                    created_at_time: None,
                    memo: None,
                    amount: amount_0_to_withdraw.clone(),
                },
            )
            .await;

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
            let new_total_shares = self.get_total_shares() - shares.clone();
            self.set_total_shares(new_total_shares.clone());

            // Update user shares
            let shares_before_withdraw = self.get_user_shares().get(&user).cloned().unwrap();
            let new_user_shares = shares_before_withdraw.clone() - shares;
            self.update_user_shares(user.clone(), new_user_shares.clone());

            // Update initial deposit
            // TODO: WIP - need to fix
            // let initial_deposit = self.get_initial_deposit().get(&investor).cloned().unwrap();
            // // Remaining initial deposit proportional to the new shares
            //
            // let new_initial_deposit = initial_deposit / shares_before_withdraw.clone() * new_shares.clone();
            // self.update_initial_deposit(investor.clone(), new_initial_deposit.clone());

            save_strategy(self.clone_self());

            // Create event for withdraw
            event_service::create_user_event(
                UserEventParams::RemoveLiquidity {
                    amount: amount_0_to_withdraw.clone(),
                    token: token0.ledger.to_text(),
                    symbol: token0.symbol.clone(),
                },
                user,
            );

            WithdrawResponse {
                amount: amount_0_to_withdraw,
                current_shares: new_user_shares.clone(),
            }
        } else {
            trap("No current pool found in strategy");
        }
    }

    fn to_candid(&self) -> StrategyCandid;

    /// Converts the strategy into a StrategyResponse struct that can be returned to clients
    ///
    /// # Returns
    ///
    /// * `StrategyResponse` - A struct containing:
    ///   * `name` - Name of the strategy
    ///   * `id` - Unique identifier for the strategy
    ///   * `description` - Description of what the strategy does
    ///   * `pools` - List of pool symbols this strategy can invest in
    ///   * `current_pool` - The pool currently being used, if any
    ///   * `total_shares` - Total number of shares issued by this strategy
    ///   * `user_shares` - Mapping of user principals to their share amounts
    ///   * `initial_deposit` - Mapping of user principals to their initial deposits
    fn to_response(&self) -> StrategyResponse {
        StrategyResponse {
            name: self.get_name(),
            id: self.get_id(),
            description: self.get_description(),
            pools: self.get_pools(),
            current_pool: self.get_current_pool(),
            total_shares: self.get_total_shares(),
            user_shares: self.get_user_shares(),
            initial_deposit: self.get_initial_deposit(),
        }
    }

    /// Rebalances the strategy by finding and moving to the pool with the highest APY
    ///
    /// # Details
    ///
    /// 1. Gets data for all available pools
    /// 2. Finds the pool with highest APY
    /// 3. If current pool is different from highest APY pool:
    ///    - Withdraws liquidity from current pool
    ///    - Swaps token_1 to token_0 (base token)
    ///    - Adds liquidity to new pool
    ///    - Updates current pool
    ///
    /// # Returns
    ///
    /// * `RebalanceResponse` - Contains:
    ///   * `pool` - The pool being used after rebalancing
    ///
    async fn rebalance(&mut self) -> RebalanceResponse {
        let pools_data = get_pools_data(self.get_pools()).await;
        let mut max_apy = 0;
        let mut max_apy_pool = None;

        // Find pool with highest APY
        for pool_data in pools_data {
            if pool_data.apy > max_apy {
                max_apy = pool_data.apy;
                max_apy_pool = Some(pool_data.pool);
            }
        }

        let current_pool = self.get_current_pool();

        if max_apy_pool.is_none() {
            return RebalanceResponse {
                previous_pool: current_pool.clone().unwrap(),
                current_pool: current_pool.clone().unwrap(),
                is_rebalanced: false,
            };
        }

        let max_apy_pool = max_apy_pool.unwrap();

        if let Some(current_pool) = &current_pool {
             // If current pool is the same as max APY pool, return
            if current_pool.is_same_pool(&max_apy_pool) {
                return RebalanceResponse {
                    previous_pool: current_pool.clone(),
                    current_pool: current_pool.clone(),
                    is_rebalanced: false,
                };
            }

            let token0 = current_pool.token0.clone();
            let token1 = current_pool.token1.clone();

            // Withdraw liquidity from current pool
            let withdraw_response = withdraw_liquidity_from_pool(
                self.get_total_shares(),
                self.get_total_shares(),
                current_pool.clone(),
            ).await;

            let token_0_withdrawn_amount = withdraw_response.token_0_amount;
            let token_1_withdrawn_amount = withdraw_response.token_1_amount;

            // Swap withdrawed token_1 to token_0 (base token)
            let swap_response = swap_service::swap_icrc2_optimal(
                token1.clone(),
                token0.clone(),
                nat_to_u128(token_1_withdrawn_amount),
            ).await;

            // Calculate total token_0 to send in new pool after swap
            let token_0_to_pool_amount = token_0_withdrawn_amount + swap_response.amount_out;

            // Add liquidity to new pool
            add_liquidity_to_pool(
                token_0_to_pool_amount,
                max_apy_pool.clone(),
            ).await;

            // Create event for rebalance
            event_service::create_system_event(
                SystemEventParams::Rebalance {
                    old_pool: current_pool.token0.symbol.clone(),
                    new_pool: max_apy_pool.token0.symbol.clone(),
                },
            );

            // Update current pool
            self.set_current_pool(Some(max_apy_pool));

            RebalanceResponse {
                previous_pool: current_pool.clone(),
                current_pool: self.get_current_pool().unwrap(),
                is_rebalanced: true,
            }
        } else {
            trap("No current pool");
        }
    }

    fn clone_self(&self) -> Box<dyn IStrategy>;
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
        StrategyIterator {
            inner: trs,
            index: 0,
        }
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
