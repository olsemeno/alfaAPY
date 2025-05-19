use async_trait::async_trait;
use ic_cdk::trap;
use candid::{Nat, Principal};
use std::ops::{Div, Mul};
use num_traits::ToPrimitive;

use crate::util::util::{nat_to_u128, nat_to_f64};
use crate::liquidity::liquidity_client::LiquidityClient;
use crate::types::types::{AddLiquidityResponse, WithdrawFromPoolResponse, PoolNew};
use types::CanisterId;
use kongswap_canister::PoolReply;
use crate::providers::icpswap::icpswap::{
    metadata,
    get_token_meta,
    get_price,
    quote,
    mint,
    deposit_from,
    get_pool,
    increase_liquidity,
    decrease_liquidity,
    get_user_position_ids_by_principal,
    get_user_position,
    withdraw,
    claim,
};
use icpswap_swap_pool_canister::getTokenMeta::{TokenMetadataRecord, TokenMetadataValue};
use icpswap_swap_pool_canister::metadata::Metadata;
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;
use icpswap_swap_pool_canister::decreaseLiquidity::DecreaseLiquidityResponse;
use icpswap_swap_pool_canister::getUserPosition::UserPosition;
use icpswap_swap_pool_canister::claim::ClaimResponse;
use icpswap_swap_factory_canister::ICPSwapPool;
use crate::swap::swap_service::swap_icrc2_icpswap;
use crate::strategies::calculator::Calculator;
use icrc_ledger_canister::icrc2_approve::ApproveArgs;
use types::exchanges::TokenInfo;

const TICK_LOWER: i32 = -887220;
const TICK_UPPER: i32 = 887220;
pub struct ICPSwapLiquidityClient {
    canister_id: CanisterId,
    token0: TokenInfo,
    token1: TokenInfo,
    pool: ICPSwapPool,
    token_meta: TokenMeta,
}

impl ICPSwapLiquidityClient {
    pub async fn new(token0: TokenInfo, token1: TokenInfo) -> ICPSwapLiquidityClient {
        let pool = match Self::get_pool(token0.clone(), token1.clone()).await {
            Ok(pool) => pool,
            Err(e) => trap(format!("Failed to get pool (ICPSWAP): {}", e).as_str()),
        };

        let canister_id = pool.canisterId;

        let token_meta = match Self::get_token_meta(canister_id).await {
            Ok(token_meta) => token_meta,
            Err(e) => trap(format!("Failed to get token meta (ICPSWAP): {}", e).as_str()),
        };

        ICPSwapLiquidityClient {
            canister_id,
            token0,
            token1,
            pool,
            token_meta,
        }
    }

    fn extract_token_decimals(&self, meta: &[TokenMetadataRecord]) -> Option<u128> {
        meta.iter()
            .find_map(|TokenMetadataRecord(key, value)| {
                if key == "decimals" {
                    if let TokenMetadataValue::Nat(n) = value {
                        Some(n.0.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .and_then(|biguint| biguint.to_u128())
    }

    fn get_token_fee(&self, token: &TokenInfo) -> Nat {
        let token_address = token.ledger.to_string();

        match (self.pool.token0.address.as_str(), self.pool.token1.address.as_str()) {
            (t0, _) if t0 == token_address => {
                // If token is token0, use token0Fee
                match &self.token_meta.token0Fee {
                    Some(fee) => fee.clone(),
                    None => Nat::from(0u8)
                }
            },
            (_, t1) if t1 == token_address => {
                // If token is token1, use token1Fee
                match &self.token_meta.token1Fee {
                    Some(fee) => fee.clone(),
                    None => Nat::from(0u8)
                }
            },
            _ => {
                // Should fall into one of the above cases
                Nat::from(0u8)
            }
        }
    }
    
    async fn icrc2_approve(&self, token: TokenInfo, amount: Nat) -> Result<(), String> {
        let approve_result = match icrc_ledger_canister_c2c_client::icrc2_approve(
            token.ledger.clone(),
            &ApproveArgs {
                from_subaccount: None,
                spender: self.canister_id().into(),
                amount: amount,
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
            Ok(Err(error)) => Err(format!("ICRC2 approve SWAP (ICPSWAP) {error:?}")),
            Err(error) => Err(format!("ICRC2 approve SWAP (ICPSWAP) {error:?}")),
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
        match get_pool(token0, token1).await {
            Ok(pool) => Ok(pool),
            Err(e) => Err(format!("Failed to get pool (ICPSWAP): {}", e)),
        }
    }
    
    async fn get_token_meta(canister_id: CanisterId) -> Result<TokenMeta, String> {
        match get_token_meta(canister_id).await {
            Ok(token_meta) => Ok(token_meta),
            Err(e) => Err(format!("Failed to get token meta (ICPSWAP): {}", e)),
        }
    }
    
    async fn deposit_from(&self, token: TokenInfo, amount: Nat) -> Result<Nat, String> {
        match deposit_from(self.canister_id, token.clone(), amount, self.get_token_fee(&token)).await {
            Ok(deposited_amount) => Ok(Nat::from(deposited_amount)),
            Err(error) => {
                return Err(format!("Deposit from error (ICPSWAP) : {:?}", error));
            }
        }
    }

    async fn metadata(&self) -> Result<Metadata, String> {
        match metadata(self.canister_id).await {
            Ok(metadata) => Ok(metadata),
            Err(error) => {
                return Err(format!("Metadata error (ICPSWAP) : {:?}", error));
            }
        }
    }
    
    async fn get_price(&self, sqrt_price_x96: Nat, token_0_decimals: Nat, token_1_decimals: Nat) -> Result<f64, String> {
        match get_price(sqrt_price_x96, token_0_decimals, token_1_decimals).await {
            Ok(price) => Ok(price),
            Err(error) => {
                return Err(format!("Price error (ICPSWAP) : {:?}", error));
            }
        }
    }

    async fn quote(&self, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, String> {
        match quote(self.canister_id, amount_in, zero_for_one, amount_out_minimum).await {
            Ok(amount_out) => Ok(amount_out),
            Err(error) => {
                return Err(format!("Quote error (ICPSWAP) : {:?}", error));
            }
        }
    }

    async fn mint(&self,
        token0: TokenInfo, 
        token1: TokenInfo, 
        amount0_desired: String, 
        amount1_desired: String, 
        fee: Nat, 
        tick_lower: i32, 
        tick_upper: i32
    ) -> Result<u128, String> {
        match mint(
            self.canister_id,
            token0.ledger.to_text(),
            token1.ledger.to_text(),
            amount0_desired,
            amount1_desired,
            fee,
            tick_lower,
            tick_upper
        ).await {
            Ok(minted_amount) => Ok(minted_amount),
            Err(error) => {
                return Err(format!("Mint error (ICPSWAP) : {:?}", error));
            }
        }
    }

    async fn increase_liquidity(&self, position_id: Nat, amount0_desired: String, amount1_desired: String) -> Result<Nat, String> {
        match increase_liquidity(self.canister_id, position_id, amount0_desired, amount1_desired).await {
            Ok(amount_out_nat) => Ok(amount_out_nat),
            Err(error) => {
                return Err(format!("Increase liquidity error (ICPSWAP) : {:?}", error));
            }
        }
    }

    async fn decrease_liquidity(&self, position_id: Nat, liquidity: String) -> Result<DecreaseLiquidityResponse, String> {
        match decrease_liquidity(self.canister_id, position_id, liquidity).await {
            Ok(amount_out_nat) => Ok(amount_out_nat),
            Err(error) => {
                return Err(format!("Decrease liquidity error (ICPSWAP) : {:?}", error));
            }
        }
    }

    async fn withdraw(&self, token_out: TokenInfo, amount: Nat, token_fee: Nat) -> Result<Nat, String> {
        match withdraw(self.canister_id, token_out, amount, token_fee).await {
            Ok(amount_out_nat) => Ok(amount_out_nat),
            Err(error) => {
                return Err(format!("Withdraw error (ICPSWAP) : {:?}", error));
            }
        }
    }

    async fn claim(&self, position_id: Nat) -> Result<ClaimResponse, String> {
        match claim(self.canister_id, position_id).await {
            Ok(claim_response) => Ok(claim_response),
            Err(error) => {
                return Err(format!("Claim error (ICPSWAP) : {:?}", error));
            }
        }
    }

    async fn get_user_position_ids_by_principal(&self) -> Result<Vec<Nat>, String> {
        let principal = ic_cdk::api::id().to_string();

        match get_user_position_ids_by_principal(self.canister_id, principal).await {
            Ok(position_ids) => Ok(position_ids),
            Err(error) => {
                return Err(format!("Get user position ids by principal error (ICPSWAP) : {:?}", error));
            }
        }
    }

    async fn get_user_position(&self, position_id: Nat) -> Result<UserPosition, String> {
        match get_user_position(self.canister_id, position_id).await {
            Ok(user_position) => Ok(user_position),
            Err(error) => {
                return Err(format!("Get user position error (ICPSWAP) : {:?}", error));
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
        // 2. Get metadata
        // 3. getPrice
        // 4. quote
        // 5. calculate pool ratio and swap price
        // 6. swap
        // 7. icrc2_approve
        // 8. depositFrom
        // 9. mint or add liquidity

        // 1. Get user position ids
        let user_position_ids = match self.get_user_position_ids_by_principal().await {
            Ok(position_ids) => position_ids,
            Err(error) => {
                return Err(format!("Get user position ids error (ICPSWAP) : {:?}", error));
            }
        };

        // 2. Get metadata
        let metadata = match self.metadata().await {
            Ok(metadata) => metadata,
            Err(error) => {
                return Err(format!("Metadata error (ICPSWAP) : {:?}", error));
            }
        };
        
        let sqrt_price_x96 = Nat::from(metadata.sqrtPriceX96);
        let token_0_decimals = Nat::from(self.extract_token_decimals(&self.token_meta.token0).unwrap_or(0));
        let token_1_decimals = Nat::from(self.extract_token_decimals(&self.token_meta.token1).unwrap_or(0));

        // 3. Get price
        let price = match self.get_price(sqrt_price_x96, token_0_decimals, token_1_decimals).await {
            Ok(price) => price,
            Err(error) => {
                return Err(format!("Price error (ICPSWAP) : {:?}", error));
            }
        };

        // 4. Get quote
        let amount_out = match self.quote(
            amount.clone(),
            true,
            amount.clone()
        ).await {
            Ok(quote_amount) => quote_amount,
            Err(error) => {
                return Err(format!("Quote error (ICPSWAP) : {:?}", error));
            }
        };

         // 5. Calculate pool ratio and swap price
        let swap_price = amount.clone() / amount_out;
         // Calculate how much token_0 and token_1 to swap and add to pool
         // TODO: visibility
        let calculator_response = Calculator::calculate_pool_liquidity_amounts(
            nat_to_f64(&amount),
            price.clone(),
            nat_to_f64(&swap_price),
        );

        let token_0_for_swap = calculator_response.token_0_for_swap;
        let token_0_for_pool = calculator_response.token_0_for_pool;
        let token_1_for_pool = calculator_response.token_1_for_pool;

        // 6. Swap
        let _ = swap_icrc2_icpswap(
            self.token0.clone(),
            self.token1.clone(),
            token_0_for_swap as u128,
        ).await;

        // 7. Approve
        self.icrc2_approve(self.token0.clone(), Nat::from(token_0_for_pool as u128)).await;
        self.icrc2_approve(self.token1.clone(), Nat::from(token_1_for_pool as u128)).await;

        // 8. Deposit
        let deposited_amount_0 = match self.deposit_from(self.token0.clone(), Nat::from(token_0_for_pool as u128)).await {
            Ok(amt) => amt,
            Err(e) => trap(format!("Failed to deposit_from (ICPSWAP): {}", e).as_str()),
        };

        let deposited_amount_1 = match self.deposit_from(self.token1.clone(), Nat::from(token_1_for_pool as u128)).await {
            Ok(amt) => amt,
            Err(e) => trap(format!("Failed to deposit_from (ICPSWAP): {}", e).as_str()),
        };

        if user_position_ids.is_empty() {
            // 9. Mint new position
            match self.mint(
                self.token0.clone(),
                self.token1.clone(),
                deposited_amount_0.to_string(),
                deposited_amount_1.to_string(),
                Nat::from(metadata.fee),
                TICK_LOWER,
                TICK_UPPER,
            ).await {
                Ok(_) => Ok(AddLiquidityResponse {
                    token_0_amount: Nat::from(deposited_amount_0),
                    token_1_amount: Nat::from(deposited_amount_1),
                    request_id: 0, // TODO: remove this
                }),
                Err(e) => Err(format!("Failed to mint (ICPSWAP): {}", e)),
            }
        } else {
            // 9. Increase liquidity
            match self.increase_liquidity(
                user_position_ids[0].clone(),
                deposited_amount_0.to_string(),
                deposited_amount_1.to_string()
            ).await {
                Ok(_) => Ok(AddLiquidityResponse {
                    token_0_amount: Nat::from(deposited_amount_0),
                    token_1_amount: Nat::from(deposited_amount_1),
                    request_id: 0, // TODO: remove this
                }),
                Err(e) => Err(format!("Failed to increase liquidity (ICPSWAP): {}", e)),
            }
        }
    }

    async fn withdraw_from_pool(&self, total_shares: Nat, shares: Nat) -> Result<WithdrawFromPoolResponse, String> {
        // Flow:
        // 1. Get user position ids
        // 2. Get user position
        // 3. Get liquidity
        // 4. Calculate how much LP tokens to withdraw
        // 5. Decrease liquidity
        // 6. Withdraw

        // 1. Get user position ids
        let user_position_ids = match self.get_user_position_ids_by_principal().await {
            Ok(position_ids) => position_ids,
            Err(error) => {
                return Err(format!("Get user position ids error (ICPSWAP) : {:?}", error));
            }
        };

        if user_position_ids.is_empty() {
            return Err(format!("No position ids found (ICPSWAP)"));
        }

        let position_id = user_position_ids[0].clone();

        // 2. Get user position
        let user_position = match self.get_user_position(position_id.clone()).await {
            Ok(user_position) => user_position,
            Err(error) => {
                return Err(format!("Get user position error (ICPSWAP) : {:?}", error));
            }
        };

        let liquidity = user_position.liquidity;

        // 3. Calculate how much LP tokens to withdraw
        let liquidity_to_withdraw: f64 = nat_to_f64(&liquidity) * nat_to_f64(&shares) / nat_to_f64(&total_shares) * 100000000.0;

        // 4. Decrease liquidity
        let decrease_liquidity_response = match self.decrease_liquidity(
            position_id, 
            liquidity_to_withdraw.to_string()
        ).await {
            Ok(decrease_liquidity_response) => decrease_liquidity_response,
            Err(error) => {
                return Err(format!("Decrease liquidity error (ICPSWAP) : {:?}", error));
            }
        };

        let token_0_amount_to_withdraw = Nat::from(decrease_liquidity_response.amount0);
        let token_1_amount_to_withdraw = Nat::from(decrease_liquidity_response.amount1);

        // 5. Withdraw
        let token_0_amount_out = match self.withdraw(
            self.token0.clone(),
            token_0_amount_to_withdraw,
            Nat::from(self.get_token_fee(&self.token0))
        ).await {
            Ok(amount_out) => amount_out,
            Err(error) => {
                return Err(format!("Withdraw error (ICPSWAP) : {:?}", error));
            }
        };

        let token_1_amount_out = match self.withdraw(
            self.token1.clone(), 
            token_1_amount_to_withdraw, 
            Nat::from(self.get_token_fee(&self.token1))
        ).await {
            Ok(amount_out) => amount_out,
            Err(error) => {
                return Err(format!("Withdraw error (ICPSWAP) : {:?}", error));
            }
        };

        Ok(WithdrawFromPoolResponse {
            token_0_amount: token_0_amount_out,
            token_1_amount: token_1_amount_out,
        })
    }
}
