use async_trait::async_trait;
use ic_cdk::trap;
use candid::{Nat, Principal};
use std::ops::{Div, Mul};

use crate::liquidity::liquidity_client::LiquidityClient;
use types::CanisterId;
use kongswap_canister::PoolReply;
use crate::providers::kong::kong::{add_liquidity, add_liquidity_amounts, pools, remove_liquidity, swap_amounts, user_balances};
use kongswap_canister::user_balances::UserBalancesReply;
use crate::util::util::nat_to_f64;
use crate::strategies::calculator::Calculator;
use crate::swap::swap_service::swap_icrc2_kong;
use crate::types::types::{AddLiquidityResponse, WithdrawFromPoolResponse};
use types::exchanges::TokenInfo;

pub struct KongSwapLiquidityClient {
    canister_id: CanisterId,
    token0: TokenInfo,
    token1: TokenInfo,
}

impl KongSwapLiquidityClient {
    pub fn new(canister_id: CanisterId, token0: TokenInfo, token1: TokenInfo) -> KongSwapLiquidityClient {
        KongSwapLiquidityClient {
            canister_id,
            token0,
            token1,
        }
    }
}

#[async_trait]
impl LiquidityClient for KongSwapLiquidityClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn add_liquidity_to_pool(&self, amount: Nat) -> Result<AddLiquidityResponse, String> {
        let token_0 = self.token0.symbol.clone();
        let token_1 = self.token1.symbol.clone();
        let address_0 = self.token0.ledger.to_string();
        let address_1 = self.token1.ledger.to_string();
    
        // Get amounts of token_0 and token1 to add to pool
        let add_liq_amounts_resp = match add_liquidity_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
            (Ok(x), ) => x,
            (Err(e), ) => trap(format!("Error for {} and {} and {}: {}", token_0, token_1, amount, e).as_str()),
        };
        // Get amounts of token_0 and token1 to swap
        let swap_amounts_resp = match swap_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
            (Ok(x), ) => x,
            (Err(e), ) => trap(format!("Error for {} and {} and {}: {}", token_0, token_1, amount, e).as_str()),
        };
    
        // Calculate pool ratio and swap price
        let pool_ratio = nat_to_f64(&add_liq_amounts_resp.amount_1) / nat_to_f64(&add_liq_amounts_resp.amount_0);
        let swap_price = nat_to_f64(&swap_amounts_resp.receive_amount) / nat_to_f64(&swap_amounts_resp.pay_amount);
        // Calculate how much token_0 and token_1 to swap and add to pool
        //TODO visibility
        let calculator_response = Calculator::calculate_pool_liquidity_amounts(
            nat_to_f64(&amount),
            pool_ratio.clone(),
            swap_price.clone(),
        );
    
        let token_0_for_swap = calculator_response.token_0_for_swap;
        let token_0_for_pool = calculator_response.token_0_for_pool;
        let token_1_for_pool = calculator_response.token_1_for_pool;
        let _ = swap_icrc2_kong(
            self.token0.clone(),
            self.token1.clone(),
            token_0_for_swap as u128,
        ).await;
    
        // Add liquidity to pool with token0 and token1
        let response = add_liquidity(
            token_0,
            Nat::from(token_0_for_pool as u128),
            token_1,
            Nat::from(token_1_for_pool as u128),
            Principal::from_text(address_0).unwrap(),
            Principal::from_text(address_1).unwrap(),
        ).await;
    
        match response {
            Ok(r) => {
                Ok(AddLiquidityResponse {
                    token_0_amount: Nat::from(token_0_for_pool as u128),
                    token_1_amount: Nat::from(token_1_for_pool as u128),
                    request_id: r.request_id,
                })
            },
            Err(e) => {
                Err(format!("Error: {}", e))
            }
        }
    }

    async fn withdraw_from_pool(&self, total_shares: Nat, shares: Nat) -> Result<WithdrawFromPoolResponse, String> {
        // trap("Not implemented yet");
        let canister_id = ic_cdk::id();
    
        // Fetch LP tokens amount in pool
        let user_balances_response = match user_balances(canister_id.to_string()).await.0 {
            Ok(reply) => reply,
            Err(err) => {
                trap(format!("Error user_balances_response: {}", err).as_str());
            }
        };
    
        // Get user balance in pool
        let user_balance_reply = user_balances_response.into_iter()
            .filter_map(|reply| match reply {
                UserBalancesReply::LP(lp) => Some(lp),
            })
            .find(|balance|
                (balance.address_0 == self.token0.ledger.to_string() && balance.address_1 == self.token1.ledger.to_string()) ||
                (balance.address_0 == self.token1.ledger.to_string() && balance.address_1 == self.token0.ledger.to_string())
            )
            .unwrap_or_else(|| trap("Expected LP balance"));
    
        let balance = user_balance_reply.balance;
    
        // Calculate how much LP tokens to withdraw
        let lp_tokens_to_withdraw: f64 = balance.mul(nat_to_f64(&shares)).div(nat_to_f64(&total_shares)).mul(100000000.0);
    
        // Remove liquidity from pool
        let remove_liquidity_response = match remove_liquidity(
            self.token0.symbol.clone(),
            self.token1.symbol.clone(),
            Nat::from(lp_tokens_to_withdraw.round() as u128),
        ).await {
            Ok(r) => { r }
            Err(e) => {
                trap(format!("Error: {} with balance {} and lp_tokens_to_withdraw {}", e, balance, Nat::from(lp_tokens_to_withdraw.round() as u128)).as_str());
            },
        };
    
        Ok(WithdrawFromPoolResponse {
            token_0_amount: remove_liquidity_response.amount_0,
            token_1_amount: remove_liquidity_response.amount_1,
        })
    }
}
