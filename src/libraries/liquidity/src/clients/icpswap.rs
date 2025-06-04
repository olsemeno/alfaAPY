use async_trait::async_trait;
use ic_cdk::trap;
use candid::{Nat, Int, Principal};
use std::ops::{Div, Mul};
use num_traits::ToPrimitive;

use utils::util::{nat_to_u64};
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse, TokensFee, GetPositionByIdResponse, GetPoolData};
use types::CanisterId;
use providers::{icpswap as icpswap_provider};
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
use icrc_ledger_canister::icrc2_approve::ApproveArgs;
use icpswap_tvl_storage_canister::getPoolChartTvl::PoolChartTvl;
use types::exchanges::TokenInfo;
use swap::token_swaps::icpswap::SLIPPAGE_TOLERANCE;

use crate::liquidity_client::LiquidityClient;

// Use full range of prices for liquidity in the pool
const TICK_LOWER: i32 = -887220;
const TICK_UPPER: i32 = 887220;

pub struct ICPSwapLiquidityClient {
    canister_id: CanisterId,
    token0: TokenInfo, // token0 may be token1 in the pool and vice versa
    token1: TokenInfo, // token1 may be token0 in the pool and vice versa
    pool: ICPSwapPool,
}

impl ICPSwapLiquidityClient {
    pub async fn new(token0: TokenInfo, token1: TokenInfo) -> ICPSwapLiquidityClient {
        let pool = match Self::get_pool(token0.clone(), token1.clone()).await {
            Ok(pool) => pool,
            Err(e) => trap(format!("Failed to get pool (ICPSWAP): {}", e).as_str()),
        };

        let canister_id = pool.canisterId;

        ICPSwapLiquidityClient {
            canister_id,
            token0,
            token1,
            pool,
        }
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

    fn get_tokens_fee(&self, token_meta: &TokenMeta) -> TokensFee {
        let token_in_str = self.token0.ledger.to_string();
        let token_out_str = self.token1.ledger.to_string();

        match (self.pool.token0.address.as_str(), self.pool.token1.address.as_str()) {
            (t0, t1) if t0 == token_in_str && t1 == token_out_str => TokensFee {
                token0_fee: token_meta.token0Fee.clone(),
                token1_fee: token_meta.token1Fee.clone(),
            },
            (t0, t1) if t0 == token_out_str && t1 == token_in_str => TokensFee {
                token0_fee: token_meta.token1Fee.clone(),
                token1_fee: token_meta.token0Fee.clone(),
            },
            (t0, t1) => trap(
                format!(
                    "ICPSwapLiquidityClient.get_tokens_fee: invalid token configuration for ICPSwap pool: Expected tokens {:?} and {:?}, but got pool with token0={}, token1={}", 
                    self.token0,
                    self.token1,
                    t0,
                    t1
                ).as_str()
            ),
        }
    }
    
    fn is_zero_for_one_swap_direction(&self) -> bool {
        let token_in_str = self.token0.ledger.to_string();
        let token_out_str = self.token1.ledger.to_string();

        match (self.pool.token0.address.as_str(), self.pool.token1.address.as_str()) {
            (t0, t1) if t0 == token_in_str && t1 == token_out_str => true,
            (t0, t1) if t0 == token_out_str && t1 == token_in_str => false,
            (t0, t1) => trap(
                format!(
                    "ICPSwapLiquidityClient.is_zero_for_one_swap_direction: invalid token configuration for ICPSwap pool: Expected tokens {:?} and {:?}, but got pool with token0={}, token1={}",
                    self.token0,
                    self.token1,
                    t0,
                    t1
                ).as_str()
            ),
        }
    }

    async fn icrc2_approve(&self, token: TokenInfo, amount: Nat) -> Result<(), String> {
        let approve_result = match icrc_ledger_canister_c2c_client::icrc2_approve(
            token.ledger.clone(),
            &ApproveArgs {
                from_subaccount: None,
                spender: self.canister_id().into(),
                // amount: amount,
                amount: Nat::from(99999999999999 as u128), //TODO
                expected_allowance: None,
                expires_at: None,
                fee: None,
                memo: None,
                created_at_time: None,
            },
        )
        .await
        {
            Ok(Ok(index)) => Ok(index),
            Ok(Err(error)) => Err(format!("ICPSwapLiquidityClient.icrc2_approve: icrc2_approve error: {:?}", error)),
            Err(error) => Err(format!("ICPSwapLiquidityClient.icrc2_approve: icrc2_approve error: {:?}", error)),
        };
    
        match approve_result {
            Ok(_) => Ok(()),
            Err(a) => {
                let c = token.ledger.to_text();
                trap(format!("ICRC2 approve SWAP (ICPSWAP) {a:?} : {c:?}").as_str());
            }
        }
    }

    async fn get_pool(token0: TokenInfo, token1: TokenInfo) -> Result<ICPSwapPool, String> {
        match icpswap_provider::get_pool(token0, token1).await {
            Ok(pool) => Ok(pool),
            Err(e) => Err(format!("ICPSwapLiquidityClient.get_pool: get_pool error: {:?}", e)),
        }
    }

    async fn get_token_meta(&self) -> Result<TokenMeta, String> {
        match icpswap_provider::get_token_meta(self.canister_id).await {
            Ok(token_meta) => Ok(token_meta),
            Err(e) => Err(format!("ICPSwapLiquidityClient.get_token_meta: get_token_meta error: {:?}", e)),
        }
    }

    async fn deposit_from(&self, token: TokenInfo, amount: Nat, token_fee: Nat) -> Result<Nat, String> {
        match icpswap_provider::deposit_from(self.canister_id, token.clone(), amount, token_fee).await {
            Ok(deposited_amount) => Ok(Nat::from(deposited_amount)),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.deposit_from: deposit_from error: {:?}", error));
            }
        }
    }

    async fn metadata(&self) -> Result<Metadata, String> {
        match icpswap_provider::metadata(self.canister_id).await {
            Ok(metadata) => Ok(metadata),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.metadata: metadata error: {:?}", error));
            }
        }
    }
    
    async fn get_price(&self, sqrt_price_x96: Nat, token_0_decimals: Nat, token_1_decimals: Nat) -> Result<f64, String> {
        match icpswap_provider::get_price(sqrt_price_x96, token_0_decimals, token_1_decimals).await {
            Ok(price) => Ok(price),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_price: get_price error: {:?}", error));
            }
        }
    }

    async fn quote(&self, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, String> {
        match icpswap_provider::quote(self.canister_id, amount_in, zero_for_one, amount_out_minimum).await {
            Ok(amount_out) => Ok(amount_out),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.quote: quote error: {:?}", error));
            }
        }
    }

    async fn mint(&self,
        token0: String, 
        token1: String, 
        amount0_desired: String,
        amount1_desired: String,
        fee: Nat, 
        tick_lower: i32, 
        tick_upper: i32
    ) -> Result<Nat, String> {
        match icpswap_provider::mint(
            self.canister_id,
            token0,
            token1,
            amount0_desired,
            amount1_desired,
            fee,
            Int::from(tick_lower),
            Int::from(tick_upper)
        ).await {
            Ok(minted_amount) => Ok(minted_amount),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.mint: mint error: {:?}", error));
            }
        }
    }

    async fn swap(&self, token_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, String> {
        match icpswap_provider::swap(self.canister_id, token_in, zero_for_one, amount_out_minimum).await {
            Ok(amount_out_nat) => Ok(amount_out_nat),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.swap: swap error: {:?}", error));
            }
        }
    }

    async fn increase_liquidity(&self, position_id: Nat, amount0_desired: String, amount1_desired: String) -> Result<Nat, String> {
        match icpswap_provider::increase_liquidity(self.canister_id, position_id, amount0_desired, amount1_desired).await {
            Ok(amount_out_nat) => Ok(amount_out_nat),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.increase_liquidity: increase_liquidity error: {:?}", error));
            }
        }
    }

    async fn decrease_liquidity(&self, position_id: Nat, liquidity: String) -> Result<DecreaseLiquidityResponse, String> {
        match icpswap_provider::decrease_liquidity(self.canister_id, position_id, liquidity).await {
            Ok(amount_out_nat) => Ok(amount_out_nat),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.decrease_liquidity: decrease_liquidity error: {:?}", error));
            }
        }
    }

    async fn withdraw(&self, token_out: TokenInfo, amount: Nat, token_fee: Nat) -> Result<Nat, String> {
        match icpswap_provider::withdraw(self.canister_id, token_out, amount, token_fee).await {
            Ok(amount_out_nat) => Ok(amount_out_nat),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.withdraw: withdraw error: {:?}", error));
            }
        }
    }

    async fn claim(&self, position_id: Nat) -> Result<ClaimResponse, String> {
        match icpswap_provider::claim(self.canister_id, position_id).await {
            Ok(claim_response) => Ok(claim_response),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.claim: claim error: {:?}", error));
            }
        }
    }

    async fn get_user_position_ids_by_principal(&self) -> Result<Vec<Nat>, String> {
        let principal = ic_cdk::api::id();

        match icpswap_provider::get_user_position_ids_by_principal(self.canister_id, principal).await {
            Ok(position_ids) => Ok(position_ids),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_user_position_ids_by_principal: get_user_position_ids_by_principal error: {:?}", error));
            }
        }
    }

    async fn get_user_positions_by_principal(&self) -> Result<Vec<UserPositionWithId>, String> {
        let principal = ic_cdk::api::id();

        match icpswap_provider::get_user_positions_by_principal(self.canister_id, principal).await {
            Ok(user_positions) => Ok(user_positions),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_user_positions_by_principal: get_user_positions_by_principal error: {:?}", error));
            }
        }
    }

    async fn get_user_position(&self, position_id: Nat) -> Result<UserPosition, String> {
        match icpswap_provider::get_user_position(self.canister_id, position_id).await {
            Ok(user_position) => Ok(user_position),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_user_position: get_user_position error: {:?}", error));
            }
        }
    }

    async fn get_token_amount_by_liquidity(
        &self,
        sqrt_price_x96: Nat,
        tick_lower: Int,
        tick_upper: Int,
        liquidity: Nat
    ) -> Result<GetTokenAmountByLiquidityResponse, String> {
        match icpswap_provider::get_token_amount_by_liquidity(sqrt_price_x96, tick_lower, tick_upper, liquidity).await {
            Ok(token_amount) => Ok(token_amount),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_token_amount_by_liquidity: get_token_amount_by_liquidity error: {:?}", error));
            }
        }
    }

    async fn get_all_tokens(&self) -> Result<Vec<TokenData>, String> {
        match icpswap_provider::get_all_tokens().await {
            Ok(tokens) => Ok(tokens),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_all_tokens: get_all_tokens error: {:?}", error));
            }
        }
    }

    async fn get_tvl_storage_canister(&self) -> Result<String, String> {
        match icpswap_provider::get_tvl_storage_canister().await {
            Ok(tvl_storage_canister_response) => Ok(tvl_storage_canister_response[0].clone()),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_tvl_storage_canister: get_tvl_storage_canister error: {:?}", error));
            }
        }
    }

    async fn get_pool_chart_tvl(&self, tvl_storage_canister_id: Principal) -> Result<Vec<PoolChartTvl>, String> {
        let offset = Nat::from(0u128);
        let limit = Nat::from(0u128);

        match icpswap_provider::get_pool_chart_tvl(
            tvl_storage_canister_id,
            self.canister_id.to_string(),
            offset,
            limit
        ).await {
            Ok(pool_chart_tvl) => Ok(pool_chart_tvl),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_pool_data: get_pool_chart_tvl error: {:?}", error));
            }
        }
    }
}

#[async_trait]
impl LiquidityClient for ICPSwapLiquidityClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn add_liquidity_to_pool(&self, amount: Nat) -> Result<AddLiquidityResponse, String> {
        // Flow:
        // 1. Get user position ids
        // 2. Get token meta
        // 3. Get metadata
        // 4. Approve before deposit
        // 5. Deposit
        // 6. Quote
        // 7. Swap half of the token0 amount for the pool
        // 8. Mint new position or increase liquidity


        // 1. Get user position ids
        let user_position_ids = match self.get_user_position_ids_by_principal().await {
            Ok(position_ids) => position_ids,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.add_liquidity_to_pool: get_user_position_ids_by_principal error: {:?}", error));
            }
        };

        // 2. Get token meta
        // TODO: Fix token meta fetching
        // let token_meta = match self.get_token_meta().await {
        //     Ok(token_meta) => token_meta,
        //     Err(e) => trap(format!("Failed to get token meta (ICPSWAP): {}", e).as_str()),
        // };

        // let tokens_fee = self.get_tokens_fee(&token_meta);
        // let token_in_fee = tokens_fee.token_in_fee.unwrap_or(Nat::from(0u8));
        // let token_out_fee = tokens_fee.token_out_fee.unwrap_or(Nat::from(0u8));

        //TODO: Remove hardcoded fees
        // let token0_fee = Nat::from(10_000u128); // For ICP
        // let token1_fee = Nat::from(10u8); // For ckBTC

        let token0_fee = Nat::from(10_000u128); // For PANDA

        // 3. Get metadata
        let metadata = match self.metadata().await {
            Ok(metadata) => metadata,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.add_liquidity_to_pool: metadata fetching error: {:?}", error));
            }
        };
        
        // 4. Approve before deposit
        match self.icrc2_approve(self.token0.clone(), amount.clone()).await {
            Ok(_) => (),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.add_liquidity_to_pool: icrc2_approve error: {:?}", error));
            }
        };

        // 5. Deposit
        let amount0_deposited = match self.deposit_from(
            self.token0.clone(),
            amount.clone(),
            token0_fee
        ).await {
            Ok(amount0_deposited) => amount0_deposited,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.add_liquidity_to_pool: deposit_from error: {:?}", error));
            }
        };

        let amount0_for_swap = amount0_deposited.clone().div(2u32);
        let amount0_for_pool = amount0_deposited.clone() - amount0_for_swap.clone();

        // 6. Quote
        // ICPSWAP provider is more convenient for swap for adding liquidity to ICPSwap pool
        let quote_amount = match self.quote(
            amount0_for_swap.clone(),
            self.is_zero_for_one_swap_direction(),
            Nat::from(0u128)
        ).await {
            Ok(quote_amount) => quote_amount,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.add_liquidity_to_pool: quote error: {:?}", error));
            }
        };

        let amount1_min_after_swap = quote_amount.clone()
            .div(1000u128) * (1000u128 - SLIPPAGE_TOLERANCE); // consider slippage tolerance

        // panic!("amount0_for_swap {:?}, amount0_for_pool {:?}, amount1_min_after_swap {:?}, quote_amount {:?}", amount0_for_swap, amount0_for_pool, amount1_min_after_swap, quote_amount);

        // 7. Swap half of the token0 amount for the pool
        // ICPSWAP provider is more convenient for swap for adding liquidity to ICPSwap pool
        let amount1_swapped_for_pool = match self.swap(
            amount0_for_swap.clone(),
            self.is_zero_for_one_swap_direction(),
            amount1_min_after_swap
        ).await {
            Ok(amount1_swapped) => amount1_swapped,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.add_liquidity_to_pool: swap error: {:?}", error));
            }
        };

        // Token0 and token1 in the pool are determined by the token0 and token1 in the metadata
        // So we need to determine the tokens amount order in the pool for minting new position or increasing liquidity
        let (amount0_for_position, amount1_for_position) = match (
            self.token0.ledger.to_string() == metadata.token0.address,
            self.token1.ledger.to_string() == metadata.token1.address,
            self.token0.ledger.to_string() == metadata.token1.address,
            self.token1.ledger.to_string() == metadata.token0.address,
        ) {
            // Token0 is token0 in the pool and token1 is token1 in the pool
            (true, true, _, _) => (amount0_for_pool.to_string(), amount1_swapped_for_pool.to_string()),
            // Token1 is token0 in the pool and token0 is token1 in the pool
            (_, _, true, true) => (amount1_swapped_for_pool.to_string(), amount0_for_pool.to_string()),
            _ => {
                return Err("ICPSwapLiquidityClient.add_liquidity_to_pool: token order does not match pool metadata".to_string());
            }
        };

        match user_position_ids.as_slice() {
            [] => {
                // 8. Mint new position if no position exists
                match self.mint(
                    metadata.token0.address.clone(),
                    metadata.token1.address.clone(),
                    amount0_for_position.to_string(),
                    amount1_for_position.to_string(),
                    Nat::from(metadata.fee),
                    TICK_LOWER,
                    TICK_UPPER,
                ).await {
                    Ok(position_id) => Ok(AddLiquidityResponse {
                        token_0_amount: Nat::from(amount0_for_pool),
                        token_1_amount: Nat::from(amount1_swapped_for_pool),
                        request_id: nat_to_u64(&position_id),
                    }),
                    Err(e) => Err(format!("ICPSwapLiquidityClient.add_liquidity_to_pool: mint error: {:?}", e)),
                }
            }
            [position_id, ..] => {
                // 8. Increase liquidity if position already exists
                match self.increase_liquidity(
                    position_id.clone(),
                    amount0_for_position.to_string(),
                    amount1_for_position.to_string(),
                ).await {
                    Ok(position_id) => Ok(AddLiquidityResponse {
                        token_0_amount: Nat::from(amount0_for_pool),
                        token_1_amount: Nat::from(amount1_swapped_for_pool),
                        request_id: nat_to_u64(&position_id),
                    }),
                    Err(e) => Err(format!("ICPSwapLiquidityClient.add_liquidity_to_pool: increase_liquidity error: {:?}", e)),
                }
            }
        }

        // TODO: Withdraw remaining token0 and token1 from canister after adding liquidity
    }

    async fn withdraw_liquidity_from_pool(&self, total_shares: Nat, shares: Nat) -> Result<WithdrawFromPoolResponse, String> {
        // Flow:
        // 1. Get user position ids
        // 2. Get token meta
        // 3. Get user position
        // 4. Calculate how much liquidity to withdraw
        // 5. Decrease liquidity
        // 6. Determine which token is token0 and which is token1
        // 7. Withdraw token0
        // 8. Withdraw token1

        // 1. Get user position ids
        let user_position_ids = match self.get_user_position_ids_by_principal().await {
            Ok(position_ids) => position_ids,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.withdraw_liquidity_from_pool: get user position ids error: {:?}", error));
            }
        };

        if user_position_ids.is_empty() {
            return Err(format!("ICPSwapLiquidityClient.withdraw_liquidity_from_pool: no position ids found"));
        }

        let position_id = user_position_ids[0].clone();

        let metadata = match self.metadata().await {
            Ok(metadata) => metadata,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.withdraw_liquidity_from_pool: metadata fetching error: {:?}", error));
            }
        };

        // 2. Get token meta
        // TODO: Fix token meta fetching
        // let token_meta = match self.get_token_meta().await {
        //     Ok(token_meta) => token_meta,
        //     Err(e) => trap(format!("Failed to get token meta (ICPSWAP): {}", e).as_str()),
        // };

        // let tokens_fee = self.get_tokens_fee(&token_meta);
        // let token_in_fee = tokens_fee.token_in_fee.unwrap_or(Nat::from(0u8));
        // let token_out_fee = tokens_fee.token_out_fee.unwrap_or(Nat::from(0u8));

        //TODO: Remove hardcoded fees
        let token0_fee = Nat::from(10_000u128); // For PANDA
        let token1_fee = Nat::from(10_000u128); // For ICP

        // 3. Get user position
        let user_position = match self.get_user_position(position_id.clone()).await {
            Ok(user_position) => user_position,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.withdraw_liquidity_from_pool: get_user_position error: {:?}", error));
            }
        };

        let liquidity = user_position.liquidity;

        // 4. Calculate how much liquidity to withdraw
        let liquidity_to_withdraw = liquidity
            .clone()
            .mul(shares.clone())
            .div(total_shares.clone());

        // 5. Decrease liquidity
        let decrease_liquidity_response = match self.decrease_liquidity(
            position_id,
            liquidity_to_withdraw.to_string()
        ).await {
            Ok(decrease_liquidity_response) => decrease_liquidity_response,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.withdraw_liquidity_from_pool: decrease_liquidity error: {:?}", error));
            }
        };

        // Determine which token is token0 and which is token1
        let (amount0_to_withdraw, amount1_to_withdraw) = match (
            self.token0.ledger.to_string() == metadata.token0.address,
            self.token1.ledger.to_string() == metadata.token1.address,
            self.token0.ledger.to_string() == metadata.token1.address,
            self.token1.ledger.to_string() == metadata.token0.address,
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
                return Err("ICPSwapLiquidityClient.withdraw_liquidity_from_pool: token order does not match pool metadata".to_string());
            }
        };

        // 6. Withdraw token0
        let token_0_amount_out = match self.withdraw(
            self.token0.clone(),
            amount0_to_withdraw.clone(),
            token0_fee
        ).await {
            Ok(amount_out) => amount_out,
            Err(error) => {
                return Err(format!(
                    "ICPSwapLiquidityClient.withdraw_liquidity_from_pool: \
                    withdrawing token0 error with amount0_to_withdraw={:?}, token0={:?}: {:?}",
                    amount0_to_withdraw,
                    self.token0.ledger,
                    error
                ));
            }
        };

        // 7. Withdraw token1
        let token_1_amount_out = match self.withdraw(
            self.token1.clone(),
            amount1_to_withdraw.clone(),
            token1_fee
        ).await {
            Ok(amount_out) => amount_out,
            Err(error) => {
                return Err(format!(
                    "ICPSwapLiquidityClient.withdraw_liquidity_from_pool: \
                    withdrawing token1 error with amount1_to_withdraw={:?}, token1={:?}: {:?}",
                    amount1_to_withdraw,
                    self.token1.ledger,
                    error
                ));
            }
        };

        Ok(WithdrawFromPoolResponse {
            token_0_amount: token_0_amount_out,
            token_1_amount: token_1_amount_out,
        })
    }

    async fn get_position_by_id(&self, position_id: Nat) -> Result<GetPositionByIdResponse, String> {
        // 3. Get metadata
        let metadata = match self.metadata().await {
            Ok(metadata) => metadata,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_position_by_id: metadata fetching error: {:?}", error));
            }
        };

        let sqrt_price_x96 = metadata.sqrtPriceX96;

        // 3. Get user position
        let user_position = match self.get_user_position(position_id.clone()).await {
            Ok(user_position) => user_position,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_position_by_id: get_user_position error: {:?}", error));
            }
        };

        let token0_owed = user_position.tokensOwed0; // Amount of token0 from fees
        let token1_owed = user_position.tokensOwed1; // Amount of token1 from fees
        let liquidity = user_position.liquidity;
        let tick_lower = user_position.tickLower;
        let tick_upper = user_position.tickUpper;

        let token_amounts = match self.get_token_amount_by_liquidity(
            sqrt_price_x96,
            tick_lower,
            tick_upper,
            liquidity
        ).await {
            Ok(token_amount) => token_amount,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_position_by_id: get_token_amount_by_liquidity error: {:?}", error));
            }
        };

        let token0_amount = token_amounts.amount0 + token0_owed;
        let token1_amount = token_amounts.amount1 + token1_owed;

        let all_tokens = match self.get_all_tokens().await {
            Ok(tokens) => tokens,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_position_by_id: get_all_tokens error: {:?}", error));
            }
        };

        let mut token0_price = 0.0;
        let mut token1_price = 0.0;

        for token in &all_tokens {
            match token.address.as_str() {
                addr if addr == self.token0.ledger.to_string() => token0_price = token.priceUSD,
                addr if addr == self.token1.ledger.to_string() => token1_price = token.priceUSD,
                _ => {}
            }
            if token0_price != 0.0 && token1_price != 0.0 {
                break;
            }
        }

        let token0_usd_amount = token0_amount.clone().mul(Nat::from(token0_price as u128));
        let token1_usd_amount = token1_amount.clone().mul(Nat::from(token1_price as u128));

        Ok(GetPositionByIdResponse {
            position_id: position_id,
            token_0_amount: token0_amount,
            token_1_amount: token1_amount,
            usd_amount_0: token0_usd_amount,
            usd_amount_1: token1_usd_amount,
        })
    }

    async fn get_pool_data(&self) -> Result<GetPoolData, String> {
        let tvl_storage_canister_id  = match self.get_tvl_storage_canister().await {
            Ok(tvl_storage_canister_id) => Principal::from_text(tvl_storage_canister_id).unwrap(),
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_pool_data: get_tvl_storage_canister error: {:?}", error));
            }
        };

        let pool_chart_tvl_response = match self.get_pool_chart_tvl(tvl_storage_canister_id).await {
            Ok(pool_chart_tvl) => pool_chart_tvl,
            Err(error) => {
                return Err(format!("ICPSwapLiquidityClient.get_pool_data: get_pool_chart_tvl error: {:?}", error));
            }
        };

        let tvl = Nat::from(pool_chart_tvl_response.last().unwrap().tvlUSD as u128);

        Ok(GetPoolData {
            tvl: tvl,
        })
    }
}
