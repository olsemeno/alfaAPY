use async_trait::async_trait;
use candid::Nat;
use std::ops::{Div, Mul};   
use std::collections::HashMap;

use types::CanisterId;
use providers::kongswap as kongswap_provider;
use kongswap_canister::user_balances::UserBalancesReply;
use utils::util::nat_to_f64;
use swap::swap_service;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse, GetPositionByIdResponse, GetPoolDataResponse};
use errors::internal_error::error::InternalError;
use errors::internal_error::error::build_error_code;
use icrc_ledger_client;
use utils::constants::CKUSDT_TOKEN_CANISTER_ID;

use crate::liquidity_client::LiquidityClient;
use crate::liquidity_calculator::LiquidityCalculator;


pub struct KongSwapLiquidityClient {
    canister_id: CanisterId,
    // TODO: change to Pool
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

    async fn add_liquidity_to_pool(&self, amount: Nat) -> Result<AddLiquidityResponse, InternalError> {
        // Get liquidity amounts for pool
        let add_liq_amounts_reply = kongswap_provider::add_liquidity_amounts(
            self.token_kongswap_format(self.token0.clone()),
            amount.clone(),
            self.token_kongswap_format(self.token1.clone()),
        ).await?;

        let amount_0_for_pool = add_liq_amounts_reply.amount_0;
        let amount_1_for_pool = add_liq_amounts_reply.amount_1;

        // Get quote for token swap
        let quote_result = swap_service::quote_swap_icrc2_optimal(
            self.token0.clone(),
            self.token1.clone(),
            amount.clone(),
        ).await?;

        let amount_out = quote_result.amount_out;
        let swap_provider = quote_result.provider;

        // Calculate pool ratio and swap price for better swap proposition 
        // to make equal amount of token0 and token1 in pool
        let pool_ratio = nat_to_f64(&amount_1_for_pool) / nat_to_f64(&amount_0_for_pool); // TODO: Change f64 -> Nat
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
        let swap_response = swap_service::swap_icrc2(
            self.token0.clone(),
            self.token1.clone(),
            Nat::from(token_0_for_swap_amount as u128),
            swap_provider,
        ).await?;

        let token_1_for_pool_amount = swap_response.amount_out;

        // Add token0 and token1 liquidity to pool
        let response = kongswap_provider::add_liquidity(
            self.token_kongswap_format(self.token0.clone()),
            Nat::from(token_0_for_pool_amount as u128),
            self.token_kongswap_format(self.token1.clone()),
            Nat::from(token_1_for_pool_amount as u128),
            self.token0,
            self.token1,
        ).await?;

        // panic!("response: {:?}", response);

        Ok(AddLiquidityResponse {
            token_0_amount: Nat::from(token_0_for_pool_amount as u128),
            token_1_amount: Nat::from(token_1_for_pool_amount as u128),
            request_id: response.request_id,
        })
    }

    async fn withdraw_liquidity_from_pool(&self, total_shares: Nat, shares: Nat) -> Result<WithdrawFromPoolResponse, InternalError> {
        let canister_id = ic_cdk::id();

        // Fetch LP positions in pool
        let user_balances_response = kongswap_provider::user_balances(
            canister_id.to_string()
        ).await?;

        // Get user balance in pool
        let balance = user_balances_response
            .into_iter()
            .filter_map(|reply| match reply {
                UserBalancesReply::LP(lp) => Some(lp),
                _ => None,
            })
            .find(|balance|
                (balance.address_0 == self.token0.to_text() && balance.address_1 == self.token1.to_text()) ||
                (balance.address_0 == self.token1.to_text() && balance.address_1 == self.token0.to_text())
            )
            .map(|balance_reply| balance_reply.balance)
            .ok_or_else(|| {
                InternalError::business_logic(
                    build_error_code(2101, 3, 1), // 2101 03 01
                    "KongSwapLiquidityClient::withdraw_liquidity_from_pool".to_string(),
                    "No user LP balance".to_string(),
                    Some(HashMap::from([
                        ("token0".to_string(), self.token0.to_text()),
                        ("token1".to_string(), self.token1.to_text()),
                        ("total_shares".to_string(), total_shares.to_string()),
                        ("shares".to_string(), shares.to_string()),
                    ]))
                )
            })?;

        // Calculate how much LP tokens to withdraw
        let lp_tokens_to_withdraw: f64 = balance.mul(nat_to_f64(&shares)).div(nat_to_f64(&total_shares)).mul(100000000.0);

        // Remove liquidity from pool
        let remove_liquidity_response = kongswap_provider::remove_liquidity(
            self.token_kongswap_format(self.token0.clone()),
            self.token_kongswap_format(self.token1.clone()),
            Nat::from(lp_tokens_to_withdraw.round() as u128),
        ).await?;

        Ok(WithdrawFromPoolResponse {
            token_0_amount: remove_liquidity_response.amount_0,
            token_1_amount: remove_liquidity_response.amount_1,
        })
    }

    async fn get_position_by_id(&self, position_id: u64) -> Result<GetPositionByIdResponse, InternalError> {
        let canister_id = ic_cdk::id();

        // Fetch user positions in pool
        let user_balances_response = kongswap_provider::user_balances(
            canister_id.to_string()
        ).await?;

        let user_balance = user_balances_response
            .into_iter()
            .filter_map(|reply| match reply {
                UserBalancesReply::LP(lp) => Some(lp),
                _ => None,
            })
            .find(|balance|
                (balance.address_0 == self.token0.to_text() && balance.address_1 == self.token1.to_text()) ||
                (balance.address_0 == self.token1.to_text() && balance.address_1 == self.token0.to_text())
            )
            .ok_or_else(|| InternalError::business_logic(
                build_error_code(2101, 3, 2), // 2101 03 02
                "KongSwapLiquidityClient::get_position_by_id".to_string(),
                "No user LP balance".to_string(),
                Some(HashMap::from([
                    ("token0".to_string(), self.token0.to_text()),
                    ("token1".to_string(), self.token1.to_text()),
                    ("position_id".to_string(), position_id.to_string()),
                ]))
            ))?;

        let token0_decimals = icrc_ledger_client::icrc1_decimals(self.token0.clone()).await?;
        let token1_decimals = icrc_ledger_client::icrc1_decimals(self.token1.clone()).await?;
        let usdt_decimals = icrc_ledger_client::icrc1_decimals(*CKUSDT_TOKEN_CANISTER_ID).await?;

        let token0_position_balance = Nat::from(
            (user_balance.amount_0 * 10f64.powi(token0_decimals as i32)).round() as u128
        );
        let token1_position_balance = Nat::from(
            (user_balance.amount_1 * 10f64.powi(token1_decimals as i32)).round() as u128
        );

        let token0_usd_amount = Nat::from(
            (user_balance.usd_amount_0 * 10f64.powi(usdt_decimals as i32)).round() as u128
        );
        let token1_usd_amount = Nat::from(
            (user_balance.usd_amount_1 * 10f64.powi(usdt_decimals as i32)).round() as u128
        );

        Ok(GetPositionByIdResponse {
            position_id: position_id,
            token_0_amount: token0_position_balance,
            token_1_amount: token1_position_balance,
            usd_amount_0: token0_usd_amount,
            usd_amount_1: token1_usd_amount,
        })
    }

    async fn get_pool_data(&self) -> Result<GetPoolDataResponse, InternalError> {
        let pools = kongswap_provider::pools().await?;

        let pool_data = pools
            .iter()
            .find(|pool|
                (pool.address_0 == self.token0.to_text() && pool.address_1 == self.token1.to_text()) ||
                (pool.address_0 == self.token1.to_text() && pool.address_1 == self.token0.to_text())
            )
            .ok_or_else(|| InternalError::business_logic(
                build_error_code(2101, 3, 3), // 2101 03 03
                "KongSwapLiquidityClient::get_pool_data".to_string(),
                "No pool data".to_string(),
                Some(HashMap::from([
                    ("token0".to_string(), self.token0.to_text()),
                    ("token1".to_string(), self.token1.to_text()),
                ]))
            ))?;

        let token0_balance = pool_data.balance_0.clone() + pool_data.lp_fee_0.clone();
        let token1_balance = pool_data.balance_1.clone() + pool_data.lp_fee_1.clone();

        let decimals_token0 = icrc_ledger_client::icrc1_decimals(self.token0.clone()).await?;
        let decimals_token1 = icrc_ledger_client::icrc1_decimals(self.token1.clone()).await?;
        let decimals_usdt = icrc_ledger_client::icrc1_decimals(*CKUSDT_TOKEN_CANISTER_ID).await?;

        let token0_base_unit = Nat::from(10u32.pow(decimals_token0 as u32)); // 10^decimals_token0
        let token1_base_unit = Nat::from(10u32.pow(decimals_token1 as u32)); // 10^decimals_token1
        let usdt_base_unit = Nat::from(10u32.pow(decimals_usdt as u32)); // 10^decimals_usdt

        // Multiply by multiplier to get more accurate result in TVL calculation
        let multiplier = Nat::from(1000u128);
        let token0_base_unit_multiplied = token0_base_unit.clone().mul(multiplier.clone());
        let token1_base_unit_multiplied = token1_base_unit.clone().mul(multiplier.clone());

        // Get quote for token0 swap to USDT
        let swap_amount0_reply = kongswap_provider::swap_amounts(
            self.token0.clone(),
            token0_base_unit_multiplied.clone(),
            *CKUSDT_TOKEN_CANISTER_ID
        ).await?;

        // Get quote for token1 swap to USDT
        let swap_amount1_reply = kongswap_provider::swap_amounts(
            self.token1,
            token1_base_unit_multiplied.clone(),
            *CKUSDT_TOKEN_CANISTER_ID
        ).await?;

        let token0_usdt_price = swap_amount0_reply.receive_amount.div(multiplier.clone());
        let token1_usdt_price = swap_amount1_reply.receive_amount.div(multiplier);

        let token0_usdt_balance = token0_balance
            .mul(token0_usdt_price.clone())
            .div(token0_base_unit)
            .div(usdt_base_unit.clone());

        let token1_usdt_balance = token1_balance
            .mul(token1_usdt_price.clone())
            .div(token1_base_unit)
            .div(usdt_base_unit);

        let tvl = token0_usdt_balance.clone() + token1_usdt_balance.clone();

        Ok(GetPoolDataResponse {
            tvl: tvl,
        })
    }
}