use async_trait::async_trait;
use candid::{Nat, Int, Principal};
use std::ops::{Div, Mul};
use num_traits::ToPrimitive;
use std::collections::HashMap;
use std::sync::Arc;

use utils::util::{nat_to_u64, int_to_nat};
use types::CanisterId;
use providers::providers_factory::ProviderImpls;
use providers::icpswap::ICPSwapProvider;
use icpswap_swap_pool_canister::getTokenMeta::TokenMetadataValue;
use icpswap_swap_pool_canister::metadata::Metadata;
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;
use icpswap_swap_pool_canister::decreaseLiquidity::DecreaseLiquidityResponse;
use icpswap_swap_pool_canister::getUserPosition::UserPosition;
use icpswap_swap_pool_canister::claim::ClaimResponse;
use icpswap_swap_pool_canister::getUserPositionsByPrincipal::UserPositionWithId;
use icpswap_swap_factory_canister::ICPSwapPool;
use icpswap_swap_calculator_canister::getTokenAmountByLiquidity::GetTokenAmountByLiquidityResponse;
use icpswap_node_index_canister::getAllTokens::TokenData;
use icpswap_tvl_storage_canister::getPoolChartTvl::PoolChartTvl;
use swap::token_swaps::icpswap::SLIPPAGE_TOLERANCE;
use errors::internal_error::error::{InternalError, build_error_code};
use utils::constants::CKUSDT_TOKEN_CANISTER_ID;
use icrc_ledger_client;
use types::liquidity::{
    AddLiquidityResponse,
    WithdrawLiquidityResponse,
    TokensFee,
    GetPositionByIdResponse,
    GetPoolDataResponse,
};

use crate::liquidity_client::LiquidityClient;

// Use full range of prices for liquidity in the pool
const TICK_LOWER: i32 = -887220;
const TICK_UPPER: i32 = 887220;

pub struct ICPSwapLiquidityClient {
    provider_impls: ProviderImpls,
    canister_id: Option<CanisterId>,
    token0: CanisterId, // token0 may be token1 in the pool and vice versa
    token1: CanisterId, // token1 may be token0 in the pool and vice versa
    pool: Option<ICPSwapPool>,
}

impl ICPSwapLiquidityClient {
    pub fn new(
        provider_impls: ProviderImpls,
        token0: CanisterId,
        token1: CanisterId,
    ) -> ICPSwapLiquidityClient {
        ICPSwapLiquidityClient {
            provider_impls,
            canister_id: None,
            token0,
            token1,
            pool: None,
        }
    }

    pub async fn with_pool(mut self) -> Result<Self, InternalError> {
        let pool = self.get_pool(self.token0.clone(), self.token1.clone()).await?;

        self.pool = Some(pool.clone());
        self.canister_id = Some(pool.canisterId);

        Ok(self)
    }

    fn icpswap_provider(&self) -> &Arc<dyn ICPSwapProvider + Send + Sync> {
        &self.provider_impls.icpswap
    }

    fn extract_token_decimals(&self, meta: &Vec<(String, TokenMetadataValue)>) -> Option<u128> {
        meta.iter()
            .find_map(|(key, value)| {
                if key == "decimals" {
                    if let TokenMetadataValue::Nat(n) = value {
                        n.0.to_u128()
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }

    fn get_tokens_fee(&self, token_meta: &TokenMeta) -> Result<TokensFee, InternalError> {
        let token_in_str = self.token0.to_text();
        let token_out_str = self.token1.to_text();

        let pool = self.pool.as_ref().unwrap();

        match (pool.token0.address.as_str(), pool.token1.address.as_str()) {
            (t0, t1) if t0 == token_in_str && t1 == token_out_str => Ok(TokensFee {
                token0_fee: token_meta.token0Fee.clone(),
                token1_fee: token_meta.token1Fee.clone(),
            }),
            (t0, t1) if t0 == token_out_str && t1 == token_in_str => Ok(TokensFee {
                token0_fee: token_meta.token1Fee.clone(),
                token1_fee: token_meta.token0Fee.clone(),
            }),
            (t0, t1) => Err(InternalError::business_logic(
                build_error_code(2102, 3, 1), // 2102 03 01
                "ICPSwapLiquidityClient::get_tokens_fee".to_string(),
                "Invalid token configuration for ICPSwap pool".to_string(),
                Some(HashMap::from([
                    ("token0".to_string(), self.token0.to_text()),
                    ("token1".to_string(), self.token1.to_text()),
                    ("t0".to_string(), t0.to_string()),
                    ("t1".to_string(), t1.to_string()),
                ])),
            )),
        }
    }
    
    fn is_zero_for_one_swap_direction(&self) -> Result<bool, InternalError> {
        let token_in_str = self.token0.to_text();
        let token_out_str = self.token1.to_text();

        let pool = self.pool.as_ref().unwrap();

        match (pool.token0.address.as_str(), pool.token1.address.as_str()) {
            (t0, t1) if t0 == token_in_str && t1 == token_out_str => Ok(true),
            (t0, t1) if t0 == token_out_str && t1 == token_in_str => Ok(false),
            (t0, t1) => Err(InternalError::business_logic(
                build_error_code(2102, 3, 2), // 2102 03 02
                "ICPSwapLiquidityClient::is_zero_for_one_swap_direction".to_string(),
                "Invalid token configuration for ICPSwap pool".to_string(),
                Some(HashMap::from([
                    ("token0".to_string(), self.token0.to_text()),
                    ("token1".to_string(), self.token1.to_text()),
                    ("t0".to_string(), t0.to_string()),
                    ("t1".to_string(), t1.to_string()),
                ])),
            )),
        }
    }

    async fn get_pool(&self, token0: CanisterId, token1: CanisterId) -> Result<ICPSwapPool, InternalError> {
        let pool =  self.icpswap_provider().get_pool(token0, token1).await?;

        Ok(pool)
    }

    async fn get_token_meta(&self) -> Result<TokenMeta, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let token_meta = self.icpswap_provider().get_token_meta(canister_id.clone()).await?;

        Ok(token_meta)
    }

    async fn deposit_from(&self, token: CanisterId, amount: Nat, token_fee: Nat) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let deposited_amount = self.icpswap_provider().deposit_from(
            canister_id.clone(),
            token.clone(),
            amount.clone(),
            token_fee.clone()
        ).await?;

        Ok(Nat::from(deposited_amount))
    }

    async fn metadata(&self) -> Result<Metadata, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let metadata = self.icpswap_provider().metadata(canister_id.clone()).await?;

        Ok(metadata)
    }
    
    async fn get_price(
        &self,
        sqrt_price_x96: Nat,
        token_0_decimals: Nat,
        token_1_decimals: Nat
    ) -> Result<f64, InternalError> {
        let price = self.icpswap_provider().get_price(
            sqrt_price_x96.clone(),
            token_0_decimals.clone(),
            token_1_decimals.clone()
        ).await?;

        Ok(price)
    }

    async fn quote(
        &self,
        amount_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat
    ) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let amount_out = self.icpswap_provider().quote(
            canister_id.clone(),
            amount_in,
            zero_for_one,
            amount_out_minimum
        ).await?;

        Ok(amount_out)
    }

    async fn mint(
        &self,
        token0: String,
        token1: String,
        amount0_desired: String,
        amount1_desired: String,
        fee: Nat,
        tick_lower: i32,
        tick_upper: i32
    ) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let minted_amount = self.icpswap_provider().mint(
            canister_id.clone(),
            token0.clone(),
            token1.clone(),
            amount0_desired.clone(),
            amount1_desired.clone(),
            fee.clone(),
            Int::from(tick_lower),
            Int::from(tick_upper)
        ).await?;

        Ok(minted_amount)
    }

    async fn swap(
        &self,
        token_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat
    ) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let amount_out_nat = self.icpswap_provider().swap(
            canister_id.clone(),
            token_in.clone(),
            zero_for_one,
            amount_out_minimum.clone()
        ).await?;

        Ok(amount_out_nat)
    }

    async fn increase_liquidity(
        &self,
        position_id: Nat,
        amount0_desired: String,
        amount1_desired: String
    ) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let amount_out_nat = self.icpswap_provider().increase_liquidity(
            canister_id.clone(),
            position_id.clone(),
            amount0_desired.clone(),
            amount1_desired.clone()
        ).await?;

        Ok(amount_out_nat)
    }

    async fn decrease_liquidity(
        &self,
        position_id: Nat,
        liquidity: String
    ) -> Result<DecreaseLiquidityResponse, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let amount_out_nat = self.icpswap_provider().decrease_liquidity(
            canister_id.clone(),
            position_id.clone(),
            liquidity.clone()
        ).await?;

        Ok(amount_out_nat)
    }

    async fn withdraw(
        &self,
        token_out: CanisterId,
        amount: Nat,
        token_fee: Nat
    ) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let amount_out_nat = self.icpswap_provider().withdraw(
            canister_id.clone(),
            token_out.clone(),
            amount.clone(),
            token_fee.clone()
        ).await?;

        Ok(amount_out_nat)
    }

    async fn claim(
        &self,
        position_id: Nat
    ) -> Result<ClaimResponse, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let claim_response = self.icpswap_provider().claim(
            canister_id.clone(),
            position_id.clone()
        ).await?;

        Ok(claim_response)
    }

    async fn get_user_position_ids_by_principal(&self) -> Result<Vec<Nat>, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();
        let principal = ic_cdk::api::id();

        let position_ids = self.icpswap_provider().get_user_position_ids_by_principal(
            canister_id.clone(),
            principal
        ).await?;

        Ok(position_ids)
    }

    async fn get_user_positions_by_principal(&self) -> Result<Vec<UserPositionWithId>, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();
        let principal = ic_cdk::api::id();

        let user_positions = self.icpswap_provider().get_user_positions_by_principal(
            canister_id.clone(),
            principal
        ).await?;

        Ok(user_positions)
    }

    async fn get_user_position(&self, position_id: Nat) -> Result<UserPosition, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        let user_position = self.icpswap_provider().get_user_position(
            canister_id.clone(),
            position_id.clone()
        ).await?;

        Ok(user_position)
    }

    async fn get_token_amount_by_liquidity(
        &self,
        sqrt_price_x96: Nat,
        tick_lower: Int,
        tick_upper: Int,
        liquidity: Nat
    ) -> Result<GetTokenAmountByLiquidityResponse, InternalError> {
        let token_amount = self.icpswap_provider().get_token_amount_by_liquidity(
            sqrt_price_x96.clone(),
            tick_lower.clone(),
            tick_upper.clone(),
            liquidity.clone()
        ).await?;

        Ok(token_amount)
    }

    async fn get_all_tokens(&self) -> Result<Vec<TokenData>, InternalError> {
        let tokens = self.icpswap_provider().get_all_tokens()
            .await?;

        Ok(tokens)
    }

    async fn get_tvl_storage_canister(&self) -> Result<String, InternalError> {
        let tvl_storage_canister_response = self.icpswap_provider().get_tvl_storage_canister().await?;

        Ok(tvl_storage_canister_response[0].clone())
    }

    async fn get_pool_chart_tvl(&self, tvl_storage_canister_id: Principal) -> Result<Vec<PoolChartTvl>, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();
        let offset = Nat::from(0u128);
        let limit = Nat::from(0u128);

        let pool_chart_tvl = self.icpswap_provider().get_pool_chart_tvl(
            tvl_storage_canister_id.clone(),
            canister_id.to_string(),
            offset.clone(),
            limit.clone()
        ).await?;

        Ok(pool_chart_tvl)
    }
}

#[async_trait]
impl LiquidityClient for ICPSwapLiquidityClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id.as_ref().unwrap().clone()
    }

    async fn add_liquidity_to_pool(&self, amount: Nat) -> Result<AddLiquidityResponse, InternalError> {
        // Flow:
        // 1. Get user position ids
        // 2. Get token fees
        // 3. Get metadata
        // 4. Approve before deposit
        // 5. Deposit
        // 6. Quote
        // 7. Swap half of the token0 amount for the pool
        // 8. Mint new position or increase liquidity

        let error_context = "ICPSwapLiquidityClient::add_liquidity_to_pool".to_string();

        // 1. Get user position ids
        let user_position_ids = self.get_user_position_ids_by_principal().await?;

        // 2. Get token fees
        let token0_fee = icrc_ledger_client::icrc1_fee(self.token0.clone()).await?;

        // 3. Get metadata
        let metadata = self.metadata().await?;
        
        // 4. Approve before deposit
        icrc_ledger_client::icrc2_approve(
            self.canister_id(),
            self.token0.clone(),
            amount.clone()
        ).await?;

        // 5. Deposit
        let amount0_deposited = self.deposit_from(
            self.token0.clone(),
            amount.clone(),
            token0_fee.clone()
        ).await?;

        // Divided by 2 to swap half of the token0 amount to token1 for the pool
        let amount0_for_swap = amount0_deposited.clone().div(2u32);
        let amount0_for_pool = amount0_deposited.clone() - amount0_for_swap.clone();
        let amount1_out_minimum = Nat::from(0u128);
        let is_zero_for_one_swap_direction = self.is_zero_for_one_swap_direction()?;

        // 6. Quote
        // ICPSWAP provider is more convenient for swap for adding liquidity to ICPSwap pool
        let quote_amount = self.quote(
            amount0_for_swap.clone(),
            is_zero_for_one_swap_direction,
            amount1_out_minimum.clone()
        ).await?;

        // Considering slippage tolerance
        let amount1_min_after_swap = quote_amount.clone().div(1000u128) * (1000u128 - SLIPPAGE_TOLERANCE);

        // 7. Swap half of the token0 amount for the pool
        // ICPSWAP provider is more convenient for swap for adding liquidity to ICPSwap pool
        let amount1_swapped_for_pool = self.swap(
            amount0_for_swap.clone(),
            is_zero_for_one_swap_direction,
            amount1_min_after_swap.clone()
        ).await?;

        // Token0 and token1 in the pool are determined by the token0 and token1 in the metadata
        // So we need to determine the tokens amount order in the pool for minting new position or increasing liquidity
        let (amount0_for_position, amount1_for_position) = match (
            self.token0.to_text() == metadata.token0.address,
            self.token1.to_text() == metadata.token1.address,
            self.token0.to_text() == metadata.token1.address,
            self.token1.to_text() == metadata.token0.address,
        ) {
            // Token0 is token0 in the pool and token1 is token1 in the pool
            (true, true, _, _) => (amount0_for_pool.to_string(), amount1_swapped_for_pool.to_string()),
            // Token1 is token0 in the pool and token0 is token1 in the pool
            (_, _, true, true) => (amount1_swapped_for_pool.to_string(), amount0_for_pool.to_string()),
            _ => {
                return Err(InternalError::business_logic(
                    build_error_code(2102, 3, 3), // 2102 03 03
                    error_context.clone(),
                    "Token order does not match pool metadata".to_string(),
                    Some(HashMap::from([
                        ("token0".to_string(), self.token0.to_text()),
                        ("token1".to_string(), self.token1.to_text()),
                        ("metadata_token0".to_string(), metadata.token0.address),
                        ("metadata_token1".to_string(), metadata.token1.address),
                    ])),
                ));
            }
        };

        // In case of no position exists, mint new position
        // In case of position exists, increase liquidity
        let position_id = match user_position_ids.as_slice() {
            [] => {
                // 8. Mint new position if no position exists
                self.mint(
                    metadata.token0.address.clone(),
                    metadata.token1.address.clone(),
                    amount0_for_position.to_string(),
                    amount1_for_position.to_string(),
                    Nat::from(metadata.fee.clone()),
                    TICK_LOWER,
                    TICK_UPPER,
                ).await?
            }
            [position_id, ..] => {
                // 8. Increase liquidity if position already exists
                self.increase_liquidity(
                    position_id.clone(),
                    amount0_for_position.to_string(),
                    amount1_for_position.to_string(),
                ).await?
            }
        };

        Ok(AddLiquidityResponse {
            token_0_amount: Nat::from(amount0_for_pool),
            token_1_amount: Nat::from(amount1_swapped_for_pool),
            position_id: nat_to_u64(&position_id),
        })
    }

    async fn withdraw_liquidity_from_pool(
        &self,
        total_shares: Nat,
        shares: Nat
    ) -> Result<WithdrawLiquidityResponse, InternalError> {
        // Flow:
        // 1. Get user position ids
        // 2. Get user position
        // 3. Calculate how much liquidity to withdraw
        // 4. Decrease liquidity
        // 5. Determine which token is token0 and which is token1 in the pool

        let error_context = "ICPSwapLiquidityClient::withdraw_liquidity_from_pool".to_string();

        // 1. Get user position ids
        let user_position_ids = self.get_user_position_ids_by_principal().await?;

        if user_position_ids.is_empty() {
            return Err(InternalError::business_logic(
                build_error_code(2102, 3, 4), // 2102 03 04
                error_context.clone(),
                "No position ids found for user".to_string(),
                None,
            ));
        }

        let position_id = user_position_ids[0].clone();

        let metadata = self.metadata().await?;

        // 2. Get user position
        let user_position = self.get_user_position(position_id.clone()).await?;

        let liquidity = user_position.liquidity;

        // 3. Calculate how much liquidity to withdraw
        let liquidity_to_withdraw = liquidity
            .clone()
            .mul(shares.clone())
            .div(total_shares.clone());

        // 4. Decrease liquidity
        let decrease_liquidity_response = self.decrease_liquidity(
            position_id.clone(),
            liquidity_to_withdraw.to_string()
        ).await?;

        // 5. Determine which token is token0 and which is token1 in the pool
        let (amount0_to_withdraw, amount1_to_withdraw) = match (
            self.token0.to_text() == metadata.token0.address,
            self.token1.to_text() == metadata.token1.address,
            self.token0.to_text() == metadata.token1.address,
            self.token1.to_text() == metadata.token0.address,
        ) {
            (true, true, _, _) => (
                Nat::from(decrease_liquidity_response.amount0),
                Nat::from(decrease_liquidity_response.amount1)
            ),
            (_, _, true, true) => (
                Nat::from(decrease_liquidity_response.amount1),
                Nat::from(decrease_liquidity_response.amount0)
            ),
            _ => {
                return Err(InternalError::business_logic(
                    build_error_code(2102, 3, 5), // 2102 03 05
                    error_context.clone(),
                    "Token order does not match pool metadata".to_string(),
                    Some(HashMap::from([
                        ("token0".to_string(), self.token0.to_text()),
                        ("token1".to_string(), self.token1.to_text()),
                        ("metadata_token0".to_string(), metadata.token0.address),
                        ("metadata_token1".to_string(), metadata.token1.address),
                    ])),
                ));
            }
        };

        Ok(WithdrawLiquidityResponse {
            token_0_amount: amount0_to_withdraw,
            token_1_amount: amount1_to_withdraw,
        })
    }

    async fn get_position_by_id(&self, position_id: u64) -> Result<GetPositionByIdResponse, InternalError> {
        // 1. Get metadata
        let metadata = self.metadata().await?;

        let sqrt_price_x96 = metadata.sqrtPriceX96;

        // 2. Get user position
        let user_position = self.get_user_position(Nat::from(position_id)).await?;

        let token0_owed = user_position.tokensOwed0; // Amount of token0 from fees
        let token1_owed = user_position.tokensOwed1; // Amount of token1 from fees
        let liquidity = user_position.liquidity;
        let tick_lower = user_position.tickLower;
        let tick_upper = user_position.tickUpper;

        // 3. Get token amounts by liquidity
        let token_amounts = self.get_token_amount_by_liquidity(
            sqrt_price_x96.clone(),
            tick_lower.clone(),
            tick_upper.clone(),
            liquidity.clone()
        ).await?;

        let token0_amount = int_to_nat(token_amounts.amount0).unwrap() + token0_owed;
        let token1_amount = int_to_nat(token_amounts.amount1).unwrap() + token1_owed;

        // 4. Get all tokens
        let all_tokens = self.get_all_tokens().await?;

        let pool_tokens = all_tokens
            .iter()
            .filter(|token_data| {
                token_data.address == self.token0.to_text() || token_data.address == self.token1.to_text()
            }).collect::<Vec<_>>();

        let mut token0_price_usd = 0.0;
        let mut token1_price_usd = 0.0;

        // Select token0 and token1 prices from all tokens
        for token_data in &pool_tokens {
            match token_data.address.as_str() {
                addr if addr == self.token0.to_text() => { token0_price_usd = token_data.priceUSD },
                addr if addr == self.token1.to_text() => { token1_price_usd = token_data.priceUSD },
                _ => {}
            }
            if token0_price_usd != 0.0 && token1_price_usd != 0.0 {
                break;
            }
        }

        let token0_decimals = icrc_ledger_client::icrc1_decimals(self.token0.clone()).await?;
        let token1_decimals = icrc_ledger_client::icrc1_decimals(self.token1.clone()).await?;
        let usdt_decimals = icrc_ledger_client::icrc1_decimals(*CKUSDT_TOKEN_CANISTER_ID).await?;

        let token0_usd_amount = Nat::from(
            (nat_to_u64(&token0_amount) as f64
                * token0_price_usd
                * 10f64.powi(usdt_decimals as i32)
                / 10f64.powi(token0_decimals as i32)
            ) as u128
        );

        let token1_usd_amount = Nat::from(
            (nat_to_u64(&token1_amount) as f64
                * token1_price_usd
                * 10f64.powi(usdt_decimals as i32)
                / 10f64.powi(token1_decimals as i32)
            ) as u128
        );

        Ok(GetPositionByIdResponse {
            position_id: position_id,
            token_0_amount: token0_amount,
            token_1_amount: token1_amount,
            usd_amount_0: token0_usd_amount,
            usd_amount_1: token1_usd_amount,
        })
    }

    async fn get_pool_data(&self) -> Result<GetPoolDataResponse, InternalError> {
        let tvl_storage_canister_id = self.get_tvl_storage_canister().await?;

        let pool_chart_tvl_response = self.get_pool_chart_tvl(
            Principal::from_text(tvl_storage_canister_id).unwrap()
        ).await?;

        let tvl = Nat::from(pool_chart_tvl_response.last().unwrap().tvlUSD as u128);

        Ok(GetPoolDataResponse { tvl })
    }
}
