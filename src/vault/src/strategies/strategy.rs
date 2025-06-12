use async_trait::async_trait;
use candid::{Nat, Principal};
use ic_cdk::caller;
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use std::cell::RefMut;
use std::cmp::Ordering;
use std::collections::HashMap;
// use uuid::Uuid;

use swap::swap_service;
use swap::token_swaps::nat_to_u128;
use utils::util::nat_to_f64;
use liquidity::liquidity_calculator::LiquidityCalculator;
use types::exchange_id::ExchangeId;
use types::pool::PoolTrait;
use errors::internal_error::builder::InternalErrorBuilder;
use errors::internal_error::error::InternalError;

use crate::event_logs::event_log_params_builder::EventLogParamsBuilder;
use crate::event_logs::event_log_service;
use crate::repository::strategies_repo::save_strategy;
use crate::strategies::basic_strategy::BasicStrategy;
use crate::strategies::strategy_candid::StrategyCandid;
use crate::types::types::{StrategyDepositResponse, StrategyRebalanceResponse, StrategyResponse, StrategyWithdrawResponse};
use crate::liquidity::liquidity_service::{
    add_liquidity_to_pool,
    get_pools_data,
    withdraw_liquidity_from_pool,
};

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
        let mut user_shares_map = self.get_user_shares();
        user_shares_map.insert(user, shares);
        self.set_user_shares(user_shares_map);
    }

    // TODO: Test function. Remove after testing.
    async fn reset_strategy(&mut self) {
        self.set_user_shares(HashMap::new());
        self.set_total_shares(Nat::from(0u64));

        self.set_total_balance(Nat::from(0u64));
        self.set_initial_deposit(HashMap::new());

        save_strategy(self.clone_self());
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
    /// A `StrategyDepositResponse` struct containing the following fields:
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
    async fn deposit(&mut self, investor: Principal, amount: Nat) -> Result<StrategyDepositResponse, InternalError> {
        let correlation_id = "1".to_string(); // Uuid::new_v4().to_string();
        let pools_data = get_pools_data(self.get_pools()).await;

        // // TODO: remove this after testing
        // self.set_current_pool(None);

        // Set current pool to the best APY pool if not set
        if self.get_current_pool().is_none() {
            let best_apy_pool = pools_data
                .iter()
                .filter(|x| x.pool.provider == ExchangeId::KongSwap) // TODO: remove this after testing
                // .max_by_key(|x| x.apy)
                .map(|x| x.pool.clone())
                .next();

            if let Some(pool) = best_apy_pool {
                self.set_current_pool(Some(pool));
            } else {
                // ========== Event log begin ==========
                let event_log_params = EventLogParamsBuilder::strategy_deposit_failed()
                    .strategy_id(self.get_id().to_string())
                    .pool_id(None)
                    .amount0(amount)
                    .build();

                let internal_error = InternalErrorBuilder::business_logic()
                    .context("Strategy::deposit")
                    .message("No pool found to deposit".to_string())
                    .build();

                event_log_service::create_event_log(
                    event_log_params,
                    correlation_id.clone(),
                    Some(investor),
                    Some(internal_error.clone()),
                );
                // ========== Event log end ==========

                return Err(internal_error);
            }
        }

        // TODO: remove this after testing
        save_strategy(self.clone_self());

        if let Some(ref current_pool) = self.get_current_pool() {
             // Add liquidity to pool
            let add_liquidity_response = add_liquidity_to_pool(
                amount.clone(),
                current_pool.clone()
            ).await;

            // Calculate new shares for investor's deposit
            let new_shares = LiquidityCalculator::calculate_shares_for_deposit(
                amount.clone(),
                self.get_total_balance().clone(),
                self.get_total_shares().clone(),
            );

            // Update total shares
            self.set_total_shares(self.get_total_shares() + new_shares.clone());

            // Update user shares (sum of current user shares and new shares)
            let user_shares = self.get_user_shares().get(&investor).cloned().unwrap_or(Nat::from(0u64));
            let new_user_shares = user_shares + new_shares.clone();
            self.update_user_shares(investor, new_user_shares.clone());

            // Update initial deposit (sum of current user initial deposit and amount)
            let mut initial_deposit_map = self.get_initial_deposit();
            let initial_deposit = initial_deposit_map.get(&investor).cloned().unwrap_or(Nat::from(0u64));
            let new_initial_deposit = initial_deposit + amount.clone();
            initial_deposit_map.insert(investor.clone(), new_initial_deposit);
            self.set_initial_deposit(initial_deposit_map);

            // Update total balance
            self.set_total_balance(self.get_total_balance() + amount.clone());

            // Save strategy with new total balance, initial deposit, user shares and total shares
            save_strategy(self.clone_self());

            // ========== Event log begin ==========
            let event_log_params = EventLogParamsBuilder::strategy_deposit_completed()
                .strategy_id(self.get_id().to_string())
                .pool_id(Some(current_pool.get_id()))
                .amount0(amount.clone())
                .build();

            event_log_service::create_event_log(
                event_log_params,
                correlation_id.clone(),
                Some(investor),
                None,
            );
            // ========== Event log end ==========

            Ok(StrategyDepositResponse {
                amount: amount,
                shares: self.get_user_shares().get(&investor).unwrap().clone(),
                tx_id: 0,
                request_id: add_liquidity_response.request_id,
            })
        } else {
            // ========== Event log begin ==========
            let event_log_params = EventLogParamsBuilder::strategy_deposit_failed()
                .strategy_id(self.get_id().to_string())
                .pool_id(None)
                .amount0(amount)
                .build();

            let internal_error = InternalErrorBuilder::business_logic()
                .context("Strategy::deposit")
                .message("No current pool found to deposit".to_string())
                .build();

            event_log_service::create_event_log(
                event_log_params,
                correlation_id.clone(),
                Some(investor),
                Some(internal_error.clone()),
            );
            // ========== Event log end ==========

            return Err(internal_error);
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
    /// * `StrategyWithdrawResponse` - Contains the amount of tokens withdrawn and remaining shares
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
    /// TODO: Rename `shares` to `percentage`
    async fn withdraw(&mut self, mut shares: Nat) -> Result<StrategyWithdrawResponse, InternalError> {
        let correlation_id = "2".to_string(); //Uuid::new_v4().to_string();
        let investor = caller(); // <- "Ya ne halyavshchik, ya partner!"
        let user_shares = self.get_user_shares_by_principal(investor.clone());
        let current_pool = self.get_current_pool().clone();
        let current_pool_id = current_pool.clone().unwrap().get_id();

        let percentage = shares; // TODO: Fix naming
        shares = user_shares.clone() * percentage.clone() / Nat::from(100u64); // TODO: Check this operation


        if user_shares == Nat::from(0u8) {
            // ========== Event log begin ==========
            let event_log_params = EventLogParamsBuilder::strategy_withdraw_failed()
                .strategy_id(self.get_id().to_string())
                .pool_id(Some(current_pool_id))
                .shares(shares.clone())
                .build();

            let internal_error = InternalErrorBuilder::business_logic()
                .context("Strategy::withdraw")
                .message("No shares found for user".to_string())
                .extra(HashMap::from([
                    ("percentage".to_string(), percentage.to_string()),
                    ("user_shares".to_string(), user_shares.to_string()),
                    ("shares".to_string(), shares.to_string()),
                ]))
                .build();

            event_log_service::create_event_log(
                event_log_params,
                correlation_id.clone(),
                Some(investor),
                Some(internal_error.clone()),
            );
            // ========== Event log end ==========

            return Err(internal_error);
        }

        // Check if user has enough shares
        if shares > user_shares {
            // ========== Event log begin ==========
            let event_log_params = EventLogParamsBuilder::strategy_withdraw_failed()
                .strategy_id(self.get_id().to_string())
                .pool_id(Some(current_pool_id))
                .shares(shares.clone())
                .build();

            let internal_error = InternalErrorBuilder::business_logic()
                .context("Strategy::withdraw")
                .message("Not sufficient shares for user".to_string())
                .extra(HashMap::from([
                    ("percentage".to_string(), percentage.to_string()),
                    ("user_shares".to_string(), user_shares.to_string()),
                    ("shares".to_string(), shares.to_string()),
                ]))
                .build();

            event_log_service::create_event_log(
                event_log_params,
                correlation_id.clone(),
                Some(investor),
                Some(internal_error.clone()),
            );
            // ========== Event log end ==========

            return Err(internal_error);
        }

        if let Some(current_pool) = current_pool {
            let token0 = current_pool.token0.clone();
            let token1 = current_pool.token1.clone();

            // Withdraw liquidity from pool
            let withdraw_response = withdraw_liquidity_from_pool(
                self.get_total_shares(),
                shares.clone(),
                current_pool.clone(),
            )
            .await;

            // Swap withdrawn token_1 to token_0 (base token)
            let swap_response = swap_service::swap_icrc2_optimal(
                token1.clone(),
                token0.clone(),
                nat_to_f64(&withdraw_response.token_1_amount) as u128,
            ).await;

            // Sum of token_0 amount and token_1 amount after swap to token_0 (base token)
            let amount_0_to_withdraw = withdraw_response.token_0_amount + swap_response.amount_out;

            // Transfer amount of token_0 (base token) to user
            let transfer_result = icrc1_transfer(
                token0,
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
            ).await;

            match transfer_result {
                Ok(Ok(x)) => x,
                Err(message) => {
                    // ========== Event log begin ==========
                    let event_log_params = EventLogParamsBuilder::strategy_withdraw_failed()
                        .strategy_id(self.get_id().to_string())
                        .pool_id(Some(current_pool.clone().get_id()))
                        .shares(shares.clone())
                        .build();

                    let internal_error = InternalErrorBuilder::external_service("icrc1_transfer".to_string())
                        .context("Strategy::withdraw")
                        .message(format!("Transfer to user error 1: {:?}", message))
                        .extra(HashMap::from([
                            ("amount".to_string(), amount_0_to_withdraw.to_string()),
                        ]))
                        .build();

                    event_log_service::create_event_log(
                        event_log_params,
                        correlation_id.clone(),
                        Some(investor),
                        Some(internal_error.clone()),
                    );
                    // ========== Event log end ==========

                    return Err(internal_error);
                }
                Ok(Err(message)) => {
                    // ========== Event log begin ==========
                    let event_log_params = EventLogParamsBuilder::strategy_withdraw_failed()
                        .strategy_id(self.get_id().to_string())
                        .pool_id(Some(current_pool.clone().get_id()))
                        .shares(shares.clone())
                        .build();

                    let internal_error = InternalErrorBuilder::external_service("icrc1_transfer".to_string())
                        .context("Strategy::withdraw")
                        .message(format!("Transfer to user error 2: {:?}", message))
                        .extra(HashMap::from([
                            ("amount".to_string(), amount_0_to_withdraw.to_string()),
                        ]))
                        .build();

                    event_log_service::create_event_log(
                        event_log_params,
                        correlation_id.clone(),
                        Some(investor),
                        Some(internal_error.clone()),
                    );
                    // ========== Event log end ==========

                    return Err(internal_error);
                }
            };

            // Update total shares
            let new_total_shares = self.get_total_shares() - shares.clone();
            self.set_total_shares(new_total_shares.clone());

            // Update user shares
            let previous_user_shares = self.get_user_shares().get(&investor).cloned().unwrap();
            let new_user_shares = previous_user_shares.clone() - shares.clone();
            self.update_user_shares(investor.clone(), new_user_shares.clone());

            // Update initial deposit proportional to the new shares
            let mut initial_deposit = self.get_initial_deposit();
            let user_initial_deposit = initial_deposit
                .get(&investor)
                .cloned()
                .unwrap_or(Nat::from(0u64));

            let new_user_initial_deposit = if previous_user_shares == Nat::from(0u64) {
                Nat::from(0u64)
            } else {
                user_initial_deposit.clone() * new_user_shares.clone() / previous_user_shares.clone()
            };

            initial_deposit.insert(investor.clone(), new_user_initial_deposit.clone());
            self.set_initial_deposit(initial_deposit);

            // Update total balance
            let total_balance = self.get_total_balance().clone();
            let new_total_balance = total_balance - user_initial_deposit + new_user_initial_deposit.clone();
            self.set_total_balance(new_total_balance.clone());

            // Save strategy with new total balance, initial deposit, user shares and total shares
            save_strategy(self.clone_self());

            // ========== Event log begin ==========
            let event_log_params = EventLogParamsBuilder::strategy_withdraw_completed()
                .strategy_id(self.get_id().to_string())
                .pool_id(Some(current_pool.clone().get_id()))
                .shares(shares.clone())
                .amount0(amount_0_to_withdraw.clone())
                .build();

            event_log_service::create_event_log(
                event_log_params,
                correlation_id.clone(),
                Some(investor),
                None,
            );
            // ========== Event log end ==========

            Ok(StrategyWithdrawResponse {
                amount: amount_0_to_withdraw,
                current_shares: new_user_shares.clone(),
            })
        } else {
            // ========== Event log begin ==========
            let event_log_params = EventLogParamsBuilder::strategy_withdraw_failed()
                .strategy_id(self.get_id().to_string())
                .pool_id(None)
                .shares(shares.clone())
                .build();

            let internal_error = InternalErrorBuilder::business_logic()
                .context("Strategy::withdraw")
                .message("No current pool found in strategy".to_string())
                .build();

            event_log_service::create_event_log(
                event_log_params,
                correlation_id.clone(),
                Some(investor),
                Some(internal_error.clone()),
            );
            // ========== Event log end ==========

            return Err(internal_error);
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
    /// * `StrategyRebalanceResponse` - Contains:
    ///   * `pool` - The pool being used after rebalancing
    ///
    async fn rebalance(&mut self) -> Result<StrategyRebalanceResponse, InternalError> {
        let correlation_id = "3".to_string(); // Uuid::new_v4().to_string();
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
            return Ok(StrategyRebalanceResponse {
                previous_pool: current_pool.clone().unwrap(),
                current_pool: current_pool.clone().unwrap(),
                is_rebalanced: false,
            });
        }

        let max_apy_pool = max_apy_pool.unwrap();

        if let Some(current_pool) = &current_pool {
             // If current pool is the same as max APY pool, return
            if current_pool.is_same_pool(&max_apy_pool) {
                return Ok(StrategyRebalanceResponse {
                    previous_pool: current_pool.clone(),
                    current_pool: current_pool.clone(),
                    is_rebalanced: false,
                });
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

            // ========== Event log begin ==========
            let event_log_params = EventLogParamsBuilder::strategy_rebalance_completed()
                .strategy_id(self.get_id().to_string())
                .previous_pool_id(Some(current_pool.get_id()))
                .new_pool_id(Some(max_apy_pool.get_id()))
                .build();

            event_log_service::create_event_log(
                event_log_params,
                correlation_id.clone(),
                None,
                None,
            );
            // ========== Event log end ==========

            // Update current pool
            self.set_current_pool(Some(max_apy_pool));

            Ok(StrategyRebalanceResponse {
                previous_pool: current_pool.clone(),
                current_pool: self.get_current_pool().unwrap(),
                is_rebalanced: true,
            })
        } else {
            // ========== Event log begin ==========
            let event_log_params = EventLogParamsBuilder::strategy_rebalance_failed()
                .strategy_id(self.get_id().to_string())
                .previous_pool_id(None)
                .new_pool_id(None)
                .build();

            let internal_error = InternalErrorBuilder::business_logic()
                .context("Strategy::rebalance")
                .message("No current pool found in strategy".to_string())
                .build();

            event_log_service::create_event_log(
                event_log_params,
                correlation_id.clone(),
                None,
                Some(internal_error.clone()),
            );
            // ========== Event log end ==========

            return Err(internal_error);
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
            total_balance: self.get_total_balance(),
            total_shares: self.get_total_shares(),
            user_shares: self.get_user_shares(),
            initial_deposit: self.get_initial_deposit(),
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
