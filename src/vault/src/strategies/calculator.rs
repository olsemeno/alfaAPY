use std::fmt::format;
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


    pub fn calculate_pool_liquidity_amounts(
        amount: Nat,
        pool_ratio: Nat,
        swap_price: f64,
    ) -> CalculatePoolLiquidityAmountsResponse {
        // Calculate token_0 amount for swap
        let token_0_for_swap: Nat = amount.clone().mul(pool_ratio.clone()).div(Nat::from(swap_price as u128 ).add(pool_ratio.clone()));
        let token_0_for_pool = amount.clone() - token_0_for_swap.clone();

        // token1 amount from swap
        let token_1_from_swap:Nat = token_0_for_swap.clone().mul(Nat::from(swap_price as u128) );

        // Calculate required token1 amount for deposit remaining token0
        let required_token_1 = token_0_for_pool.clone().mul(pool_ratio.clone());

        // If token1 amount from swap is less than required token1 amount for deposit,
        // adjust token0 amount for deposit so that the cost matches.
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

#[cfg(test)]
mod tests {
    mod calculate_shares {
        use super::super::*;
        use candid::Nat;

        #[test]
        fn test_with_zero_total() {
            let amount = Nat::from(100u64);
            let total_balance = Nat::from(0u64);
            let total_shares = Nat::from(0u64);

            let shares = Calculator::calculate_shares(amount.clone(), total_balance, total_shares);
            assert_eq!(shares, amount);
        }

        #[test]
        fn test_with_existing_total() {
            let amount = Nat::from(100u64);
            let total_balance = Nat::from(1000u64);
            let total_shares = Nat::from(500u64);

            let shares = Calculator::calculate_shares(amount, total_balance, total_shares);
            assert_eq!(shares, Nat::from(50u64));
        }
    }

    mod calculate_pool_liquidity_amounts {
        use super::super::*;
        use candid::Nat;

        #[test]
        fn test_with_equal_pool_ratio_and_swap_price_1_to_1() {
            let amount = Nat::from(1000u64);
            let pool_ratio = Nat::from(1u64); // 1:1 ratio
            let swap_price = 1.0f64; // 1:1 ratio

            let result = Calculator::calculate_pool_liquidity_amounts(
                amount.clone(),
                pool_ratio.clone(),
                swap_price
            );

            assert_eq!(result.token_0_for_swap, Nat::from(500u64));
            assert_eq!(result.token_0_for_pool, Nat::from(500u64));
            assert_eq!(result.token_1_for_pool, Nat::from(500u64));

            // Verify total token0 used equals original amount
            let total_token_0 = result.token_0_for_swap + result.token_0_for_pool;
            assert!(total_token_0 <= amount);
        }

        #[test]
        fn test_with_different_pool_ratio_and_swap_price_2_to_1() {
            let amount = Nat::from(1000u64);
            let pool_ratio = Nat::from(2u64); // 2:1 ratio
            let swap_price = 2.0f64; // 2:1 ratio

            let result = Calculator::calculate_pool_liquidity_amounts(
                amount.clone(),
                pool_ratio.clone(),
                swap_price
            );

            assert_eq!(result.token_0_for_swap, Nat::from(500u64));
            assert_eq!(result.token_0_for_pool, Nat::from(500u64));
            assert_eq!(result.token_1_for_pool, Nat::from(1000u64));

            // Verify total token0 used equals original amount
            let total_token_0 = result.token_0_for_swap + result.token_0_for_pool;
            assert!(total_token_0 <= amount);
        }

        #[test]
        fn test_with_different_pool_ratio_and_swap_price() {
            let amount = Nat::from(1000u64);
            let pool_ratio = Nat::from(3u64); // 3:1 ratio
            let swap_price = 2.0f64; // 2:1 ratio

            let result = Calculator::calculate_pool_liquidity_amounts(
                amount.clone(),
                pool_ratio.clone(),
                swap_price
            );

            assert_eq!(result.token_0_for_swap, Nat::from(600u64));
            assert_eq!(result.token_0_for_pool, Nat::from(400u64));
            assert_eq!(result.token_1_for_pool, Nat::from(1200u64));

            // Verify total token0 used equals original amount
            let total_token_0 = result.token_0_for_swap + result.token_0_for_pool;
            assert!(total_token_0 <= amount);
        }
    }
}