use std::ops::Mul;

// TODO: remove this struct
pub struct LiquidityCalculator;

pub struct CalculatePoolLiquidityAmountsResponse {
    pub token_0_for_swap: f64,
    pub token_0_for_pool: f64,
    pub token_1_for_pool: f64,
}

impl LiquidityCalculator {
    pub fn calculate_shares(amount: f64, total_balance: f64, total_shares: f64) -> f64 {
        let zero =0f64;
        let one = 1f64;

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
        amount: f64,
        pool_ratio: f64,
        swap_price: f64,
    ) -> CalculatePoolLiquidityAmountsResponse {
        // Calculate token_0 amount for swap
        let token_0_for_swap: f64 = amount * pool_ratio / (swap_price + pool_ratio);
        let token_0_for_pool = amount - token_0_for_swap;

        // token1 amount from swap
        let token_1_from_swap = token_0_for_swap.mul(swap_price);

        // Calculate required token1 amount for deposit remaining token0
        let required_token_1 = token_0_for_pool.mul(pool_ratio);

        // If token1 amount from swap is less than required token1 amount for deposit,
        // adjust token0 amount for deposit so that the cost matches.
        let (final_token_0_for_pool, final_token_1_for_pool) = if token_1_from_swap < required_token_1 {
            let adjusted_token_0_for_pool = token_1_from_swap / pool_ratio;
            (adjusted_token_0_for_pool, token_1_from_swap)
        } else {
            (token_0_for_pool, required_token_1)
        };

        CalculatePoolLiquidityAmountsResponse {
            token_0_for_swap: token_0_for_swap.round(),
            token_0_for_pool: final_token_0_for_pool.round(),
            token_1_for_pool: final_token_1_for_pool.round(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod calculate_shares {
        use super::super::*;

        #[test]
        fn test_with_zero_total() {
            let amount = 100f64;
            let total_balance = 0f64;
            let total_shares = 0f64;

            let shares = LiquidityCalculator::calculate_shares(amount.clone(), total_balance, total_shares);
            assert_eq!(shares, amount);
        }

        #[test]
        fn test_with_existing_total() {
            let amount = 100f64;
            let total_balance = 1000f64;
            let total_shares = 500f64;

            let shares = LiquidityCalculator::calculate_shares(amount, total_balance, total_shares);
            assert_eq!(shares, 50f64);
        }
    }

    mod calculate_pool_liquidity_amounts {
        use candid::Nat;
        use super::super::*;

        #[test]
        fn test_with_equal_pool_ratio_and_swap_price_1_to_1() {
            let amount = 1000f64;
            let pool_ratio = 1f64; // 1:1 ratio
            let swap_price = 1.0f64; // 1:1 ratio

            let result = LiquidityCalculator::calculate_pool_liquidity_amounts(
                amount.clone(),
                pool_ratio.clone(),
                swap_price
            );

            assert_eq!(result.token_0_for_swap, 500f64);
            assert_eq!(result.token_0_for_pool, 500f64);
            assert_eq!(result.token_1_for_pool, 500f64);

            // Verify total token0 used equals original amount
            let total_token_0 = result.token_0_for_swap + result.token_0_for_pool;
            assert!(total_token_0 <= amount);
        }

        #[test]
        fn test_with_different_pool_ratio_and_swap_price_2_to_1() {
            let amount = 1000f64;
            let pool_ratio = 2f64; // 2:1 ratio
            let swap_price = 2.0f64; // 2:1 ratio

            let result = LiquidityCalculator::calculate_pool_liquidity_amounts(
                amount.clone(),
                pool_ratio.clone(),
                swap_price
            );

            assert_eq!(result.token_0_for_swap, 500f64);
            assert_eq!(result.token_0_for_pool, 500f64);
            assert_eq!(result.token_1_for_pool, 1000f64);

            // Verify total token0 used equals original amount
            let total_token_0 = result.token_0_for_swap + result.token_0_for_pool;
            assert!(total_token_0 <= amount);
        }

        #[test]
        fn test_with_different_pool_ratio_and_swap_price() {
            let amount = 1000f64;
            let pool_ratio = 3f64; // 3:1 ratio
            let swap_price = 2.0f64; // 2:1 ratio

            let result = LiquidityCalculator::calculate_pool_liquidity_amounts(
                amount.clone(),
                pool_ratio.clone(),
                swap_price
            );

            assert_eq!(result.token_0_for_swap, 600f64);
            assert_eq!(result.token_0_for_pool, 400f64);
            assert_eq!(result.token_1_for_pool, 1200f64);

            // Verify total token0 used equals original amount
            let total_token_0 = result.token_0_for_swap + result.token_0_for_pool;
            assert!(total_token_0 <= amount);
        }
    }
}
