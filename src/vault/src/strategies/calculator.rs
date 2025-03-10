use candid::Nat;
use crate::strategies::strategy::{Pool};
use crate::providers::kong::kong::swap_amounts;

pub struct Calculator;

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

    pub struct CalculatePoolLiquidityAmountsResponse {
        pub token_0_for_swap: Nat,
        pub token_0_for_pool: Nat,
        pub token_1_for_pool: Nat,
    }

    pub fn calculate_pool_liquidity_amounts(amount: Nat, pool: Pool) -> CalculatePoolLiquidityAmountsResponse {
        let token_0 = pool.token0;
        let token_1 = pool.token1;

        // KongSwap
        // response ?
        let add_liq_amounts_resp = add_liquidity_amounts(token0_to_pool, mid_price);
        let swap_amounts_resp = swap_amounts(token_0, amount, token_1);

        let pool_ratio = (add_liq_amounts_resp.amount_1 as f64) / (add_liq_amounts_resp.amount_0 as f64);

        // Фактическая цена свапа (price)
        let price = swap_amounts_resp.price;

        let token_0_for_swap = total_token_0 * pool_ratio / (price + pool_ratio);
        let token_0_for_pool = total_token_0 - token_0_for_swap;


        // Token1, полученных при свапе
        let token1_from_swap = token_0_for_swap * price;
        // Требуемое количество token1 для депозита оставшихся token0 через функцию add_liquidity_amounts
        let required_token_1 = token_0_for_pool * pool_ratio;


        // Если token1, полученных при свапе, меньше требуемого количества token1 для депозита,
        // то корректируем количество token0 для депозита так, чтобы стоимость совпадала.
        let (final_token_0_for_pool, final_token_1_for_pool) = if token_1_from_swap < required_token_1 {
            let adjusted_token_0_for_pool = token_1_from_swap / pool_ratio;
            (adjusted_token_0_for_pool, token_1_from_swap)
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


