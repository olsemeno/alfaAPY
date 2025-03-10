use std::fmt::format;
use crate::providers::kong::kong::{add_liquidity_amounts, swap_amounts};
use crate::strategies::strategy::Pool;
use candid::Nat;
use ic_cdk::trap;
use std::ops::{Add, Div, Mul};

pub struct Calculator;


pub struct CalculatePoolLiquidityAmountsResponse {
    pub token_0_for_swap: Nat,
    pub token_0_for_pool: Nat,
    pub token_1_for_pool: Nat,
}

impl Calculator {
    pub fn calculate_shares(amount: Nat, total_balance: Nat, total_shares: Nat) -> Nat {
        let zero = Nat::from(0u64);;
        let one = Nat::from(1u64);

        let share_price = if total_shares == zero {
            one.clone()
        } else {
            total_balance.clone() / total_shares.clone()
        };

        if total_balance == zero || total_shares == zero {
            amount
        } else {
            amount / share_price
        }
    }


    pub async fn calculate_pool_liquidity_amounts(amount: Nat, pool: Pool) -> CalculatePoolLiquidityAmountsResponse {
        let token_0 = pool.token0;
        let token_1 = pool.token1;

        // KongSwap
        // response ?
        let add_liq_amounts_resp = match add_liquidity_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
            Ok(x) => {
                x
            }
            Err(e) => {
                trap( format!("Error for {} and {} and {}", token_1, token_1, amount).as_str())
            }
        };
        let swap_amounts_resp = match swap_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
            Ok(x) => {
                x
            }
            Err(e) => {
                trap(e.as_str())
            }
        };

        let pool_ratio = (add_liq_amounts_resp.amount_1).div(add_liq_amounts_resp.amount_0);

        // Фактическая цена свапа (price)
        let price = swap_amounts_resp.price;



        let token_0_for_swap: Nat = amount.clone().mul(pool_ratio.clone()).div(Nat::from(price as u128 ).add(pool_ratio.clone()));
        let token_0_for_pool: Nat = amount.clone().min(token_0_for_swap.clone());

        // Token1, полученных при свапе
        let token_1_from_swap:Nat = token_0_for_swap.clone().mul(Nat::from(price as u128) );
        // Требуемое количество token1 для депозита оставшихся token0 через функцию add_liquidity_amounts
        let required_token_1 = token_0_for_pool.clone().mul(pool_ratio.clone());

        // Если token1, полученных при свапе, меньше требуемого количества token1 для депозита,
        // то корректируем количество token0 для депозита так, чтобы стоимость совпадала.
        let (final_token_0_for_pool, final_token_1_for_pool) = if token_1_from_swap.clone() < required_token_1 {
            let adjusted_token_0_for_pool = token_1_from_swap.clone() / pool_ratio;
            (adjusted_token_0_for_pool, token_1_from_swap.clone())
        } else {
            (token_0_for_pool, required_token_1)
        };

        CalculatePoolLiquidityAmountsResponse {
            token_0_for_swap: token_0_for_swap,
            token_0_for_pool: final_token_0_for_pool,
            token_1_for_pool: final_token_1_for_pool,
        }
    }
}


