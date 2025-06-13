use async_trait::async_trait;
use ic_cdk::trap;
use candid::{Nat, Principal};
use std::ops::{Div, Mul};

use types::CanisterId;
use providers::kongswap::{add_liquidity, add_liquidity_amounts, remove_liquidity, swap_amounts, user_balances, pools};
use kongswap_canister::user_balances::UserBalancesReply;
use utils::util::{nat_to_f64, nat_to_u64, nat_to_u128};
use swap::swap_service::{swap_icrc2_kongswap, quote_swap_icrc2_optimal, swap_icrc2};
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse, GetPositionByIdResponse, GetPoolData};
use types::context::Context;

use crate::liquidity_client::LiquidityClient;
use crate::liquidity_calculator::LiquidityCalculator;

const CKUSDT_CANISTER_ID: &str = "cngnf-vqaaa-aaaar-qag4q-cai";

pub struct KongSwapLiquidityClient {
    canister_id: CanisterId,
    token0: CanisterId,
    token1: CanisterId,
}

impl KongSwapLiquidityClient {
    pub fn new(canister_id: CanisterId, token0: CanisterId, token1: CanisterId) -> KongSwapLiquidityClient {
        KongSwapLiquidityClient {
            canister_id,
            token0,
            token1,
        }
    }

    fn token_kongswap_format(&self, token: CanisterId) -> String {
        format!("IC.{}", token.to_text())
    }
}

#[async_trait]
impl LiquidityClient for KongSwapLiquidityClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn add_liquidity_to_pool(&self, context: Context, amount: Nat) -> Result<AddLiquidityResponse, String> {
        // let token_0_str = self.token0.to_text();
        // let token_1_str = self.token1.to_text();
    
        // Get amounts of token_0 and token1 to add to pool
        let add_liq_amounts_response = match add_liquidity_amounts(
            self.token_kongswap_format(self.token0.clone()),
            amount.clone(),
            self.token_kongswap_format(self.token1.clone()),
        ).await {
            (Ok(add_liq_amounts_response), ) => add_liq_amounts_response,
            (Err(e), ) => trap(
                format!(
                    "KongSwapLiquidityClient.add_liquidity_to_pool: \
                    add_liquidity_amounts error for {} and {} and {}: {}",
                    self.token0.to_text(),
                    self.token1.to_text(),
                    amount,
                    e
                ).as_str()
            ),
        };

        // Get quote for token swap
        let quote_result = quote_swap_icrc2_optimal(
            self.token0.clone(),
            self.token1.clone(),
            nat_to_u128(&amount),
        ).await;

        let amount_out = quote_result.amount_out;
        let swap_provider = quote_result.provider;
    
        // Calculate pool ratio and swap price for better swap proposition 
        // to make equal amount of token0 and token1 in pool
        let pool_ratio = nat_to_f64(&add_liq_amounts_response.amount_1) / nat_to_f64(&add_liq_amounts_response.amount_0);
        let swap_price = (amount_out as f64) / (nat_to_f64(&amount) as f64);

        // Calculate how much token_0 and token_1 to swap and add to pool
        let calculator_response = LiquidityCalculator::calculate_token_amounts_for_deposit(
            nat_to_f64(&amount),
            pool_ratio.clone(),
            swap_price.clone(),
        );
    
        let token_0_for_swap_amount = calculator_response.token_0_for_swap;
        let token_0_for_pool_amount = calculator_response.token_0_for_pool;

        // Swap token0 for token1 with the best exchange provider
        let swap_response = swap_icrc2(
            self.token0.clone(),
            self.token1.clone(),
            token_0_for_swap_amount as u128,
            swap_provider,
        ).await;

        let token_1_for_pool_amount = swap_response.amount_out;

        // panic!("token_0_for_swap_amount {:?}, token_0_for_pool_amount {:?}, token_1_for_pool_amount {:?}", token_0_for_swap_amount, token_0_for_pool_amount, token_1_for_pool_amount);
    
        // Add token0 and token1 liquidity to pool
        match add_liquidity(
            self.token_kongswap_format(self.token0.clone()),
            Nat::from(token_0_for_pool_amount as u128),
            self.token_kongswap_format(self.token1.clone()),
            Nat::from(token_1_for_pool_amount as u128),
            self.token0,
            self.token1,
        ).await {
            Ok(response) => {
                Ok(AddLiquidityResponse {
                    token_0_amount: Nat::from(token_0_for_pool_amount as u128),
                    token_1_amount: Nat::from(token_1_for_pool_amount as u128),
                    request_id: response.request_id,
                })
            },
            Err(e) => {
                Err(format!("KongSwapLiquidityClient.add_liquidity_to_pool: add_liquidity error: {}", e))
            }
        }
    }

    async fn withdraw_liquidity_from_pool(&self, context: Context, total_shares: Nat, shares: Nat) -> Result<WithdrawFromPoolResponse, String> {
        // trap("Not implemented yet");
        let canister_id = ic_cdk::id();
    
        // Fetch LP positions in pool
        let user_balances_response = match user_balances(canister_id.to_string()).await.0 {
            Ok(reply) => reply,
            Err(err) => {
                trap(format!("KongSwapLiquidityClient.withdraw_liquidity_from_pool: user_balances error: {}", err).as_str());
            }
        };
    
        // Get user balance in pool
        let user_balance = user_balances_response
            .into_iter()
            .filter_map(|reply| match reply {
                UserBalancesReply::LP(lp) => Some(lp),
            })
            .find(|balance|
                (balance.address_0 == self.token0.to_text() && balance.address_1 == self.token1.to_text()) ||
                (balance.address_0 == self.token1.to_text() && balance.address_1 == self.token0.to_text())
            )
            .unwrap_or_else(|| trap("KongSwapLiquidityClient.withdraw_liquidity_from_pool: no user LP balance"));
    
        let balance = user_balance.balance;
    
        // Calculate how much LP tokens to withdraw
        let lp_tokens_to_withdraw: f64 = balance.mul(nat_to_f64(&shares)).div(nat_to_f64(&total_shares)).mul(100000000.0);
    
        // Remove liquidity from pool
        let remove_liquidity_response = match remove_liquidity(
            self.token_kongswap_format(self.token0.clone()),
            self.token_kongswap_format(self.token1.clone()),
            Nat::from(lp_tokens_to_withdraw.round() as u128),
        ).await {
            Ok(r) => { r }
            Err(e) => {
                trap(format!(
                    "KongSwapLiquidityClient.withdraw_liquidity_from_pool: \
                    remove_liquidity error: {} with balance {} and lp_tokens_to_withdraw {}",
                    e,
                    balance,
                    Nat::from(lp_tokens_to_withdraw.round() as u128)
                ).as_str());
            },
        };
    
        Ok(WithdrawFromPoolResponse {
            token_0_amount: remove_liquidity_response.amount_0,
            token_1_amount: remove_liquidity_response.amount_1,
        })
    }

    async fn get_position_by_id(&self, context: Context, position_id: Nat) -> Result<GetPositionByIdResponse, String> {
        let canister_id = ic_cdk::id();

        // Fetch LP positions in pool
        let user_balances_response = match user_balances(canister_id.to_string()).await.0 {
            Ok(reply) => reply,
            Err(err) => {
                trap(format!("KongSwapLiquidityClient.get_position_by_id: user_balances error: {}", err).as_str());
            }
        };

        let user_balance = user_balances_response
            .into_iter()
            .filter_map(|reply| match reply {
                UserBalancesReply::LP(lp) => Some(lp),
            })
            .find(|balance|
                balance.lp_token_id == nat_to_u64(&position_id) &&
                (
                    (
                        balance.address_0 == self.token0.to_text() &&
                        balance.address_1 == self.token1.to_text()
                    ) ||
                    (
                        balance.address_0 == self.token1.to_text() &&
                        balance.address_1 == self.token0.to_text()
                    )
                )
            )
            .unwrap_or_else(|| trap("KongSwapLiquidityClient.get_position_by_id: no user LP balance"));

        Ok(GetPositionByIdResponse {
            position_id: position_id,
            token_0_amount: Nat::from(user_balance.amount_0 as u128),
            token_1_amount: Nat::from(user_balance.amount_1 as u128),
            usd_amount_0: Nat::from(user_balance.usd_amount_0 as u128),
            usd_amount_1: Nat::from(user_balance.usd_amount_1 as u128),
        })
    }

    async fn get_pool_data(&self, context: Context) -> Result<GetPoolData, String> {
        let pools_response = match pools().await {
            Ok(reply) => reply,
            Err(err) => {
                trap(format!("KongSwapLiquidityClient.get_pool_data: pools error: {}", err).as_str());
            }
        };

        let pool_data = pools_response.pools
            .iter()
            .find(|pool|
                (
                    pool.address_0 == self.token0.to_text() && pool.address_1 == self.token1.to_text()
                ) ||
                (
                    pool.address_0 == self.token1.to_text() && pool.address_1 == self.token0.to_text()
                )
            )
            .unwrap_or_else(|| trap("KongSwapLiquidityClient.get_pool_data: no pool"));

        let balance0 = pool_data.balance_0.clone() + pool_data.lp_fee_0.clone();
        let balance1 = pool_data.balance_1.clone() + pool_data.lp_fee_1.clone();

        // Get USD amount of token0 pool
        let usd_token0_amount = match swap_amounts(
            self.token_kongswap_format(self.token0.clone()),
            balance0.clone(),
            CKUSDT_CANISTER_ID.to_string()
        ).await {
            (Ok(swap_amounts_reply), ) => swap_amounts_reply.receive_amount,
            (Err(e), ) => trap(format!(
                "KongSwapLiquidityClient.get_pool_data: \
                swap_amounts error for {} and {} and {}: {}",
                self.token0.to_text(),
                CKUSDT_CANISTER_ID.to_string(),
                balance0,
                e
            ).as_str()),
        };

        // Get USD amount of token1 pool
        let usd_token1_amount = match swap_amounts(
            self.token_kongswap_format(self.token1.clone()),
            balance1.clone(),
            CKUSDT_CANISTER_ID.to_string()
        ).await {
            (Ok(swap_amounts_reply), ) => swap_amounts_reply.receive_amount,
            (Err(e), ) => trap(format!(
                "KongSwapLiquidityClient.get_pool_data: \
                swap_amounts error for {} and {} and {}: {}",
                self.token1.to_text(),
                CKUSDT_CANISTER_ID.to_string(),
                balance1,
                e
            ).as_str()),
        };

        let tvl = usd_token0_amount + usd_token1_amount;

        Ok(GetPoolData {
            tvl: tvl,
        })
    }
}
