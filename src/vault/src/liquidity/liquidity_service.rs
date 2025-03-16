use crate::providers::kong::kong::{add_liquidity, add_liquidity_amounts, pools, remove_liquidity, swap_amounts, user_balances};
use crate::strategies::calculator::Calculator;
use crate::swap::swap_service::swap_icrc2_kong;
use crate::types::types::{AddLiquidityResponse, Pool, TokensInfo, WithdrawFromPoolResponse};
use crate::util::util::nat_to_f64;
use candid::{Nat, Principal};
use ic_cdk::trap;
use kongswap_canister::user_balances::UserBalancesReply;
use kongswap_canister::PoolReply;
use std::ops::{Div, Mul};
use types::exchanges::TokenInfo;

pub async fn get_pools_data(required_pools: Vec<Pool>) -> Vec<PoolReply> {
    match pools().await {
        Ok(response) => {
            let pools = response.pools;
            let mut pool_data = Vec::new();
            for pool in required_pools {
                match pools.iter().find(|&x| x.symbol == pool.pool_symbol)
                {
                    None => {}
                    Some(x) => {
                        pool_data.push(x.to_owned());
                    }
                }
            }
            pool_data
        }
        Err(error) => {
            trap(error.as_str());
        }
    }
}


pub fn to_tokens_info(pool: PoolReply) -> TokensInfo {
    let token_info_0 = TokenInfo {
        ledger: Principal::from_text(&pool.address_0).unwrap(),
        symbol: pool.symbol_0.clone(),
    };

    let token_info_1 = TokenInfo {
        ledger: Principal::from_text(&pool.address_1).unwrap(),
        symbol: pool.symbol_1.clone(),
    };

    TokensInfo {
        token_0: token_info_0,
        token_1: token_info_1,
    }
}

pub async fn add_liquidity_to_pool(amount: Nat, pool: PoolReply) -> AddLiquidityResponse {
    let token_0 = pool.symbol_0.clone();
    let token_1 = pool.symbol_1.clone();
    let address_0 = pool.address_0.clone();
    let address_1 = pool.address_1.clone();
    let tokens_info = to_tokens_info(pool);

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
        tokens_info.token_0,
        tokens_info.token_1,
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
            AddLiquidityResponse {
                token_0_amount: Nat::from(token_0_for_pool as u128),
                token_1_amount: Nat::from(token_1_for_pool as u128),
                request_id: r.request_id,

            }
        }
        Err(e) => {
            trap(format!("Error: {}", e).as_str());
        }
    }
}


pub async fn withdraw_from_pool(total_shares: Nat, shares: Nat, pool: PoolReply) -> WithdrawFromPoolResponse {
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
        .find(|balance| balance.symbol == pool.symbol)
        .unwrap_or_else(|| trap("Expected LP balance"));

    let balance = user_balance_reply.balance;

    // Calculate how much LP tokens to withdraw
    let lp_tokens_to_withdraw: f64 = balance.mul(nat_to_f64(&shares)).div(nat_to_f64(&total_shares)).mul(100000000.0);

    // Remove liquidity from pool
    let remove_liquidity_response = match remove_liquidity(
        pool.symbol_0.clone(),
        pool.symbol_1.clone(),
        Nat::from(lp_tokens_to_withdraw.round() as u128),
    ).await {
        Ok(r) => { r }
        Err(e) => {
            trap(format!("Error: {} with balance {} and lp_tokens_to_withdraw {}", e, balance, Nat::from(lp_tokens_to_withdraw.round() as u128)).as_str());
        }
    };

    WithdrawFromPoolResponse {
        token_0_amount: remove_liquidity_response.amount_0,
        token_1_amount: remove_liquidity_response.amount_1,
    }
}