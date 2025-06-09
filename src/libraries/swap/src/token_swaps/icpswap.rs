use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use ic_cdk::trap;
use candid::Nat;
use ic_response_codes::RejectCode;

use types::CanisterId;
use providers::{icpswap as icpswap_provider};
use icpswap_swap_factory_canister::ICPSwapPool;
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;
use types::liquidity::TokensFee;
use utils::util::nat_to_u128;

use crate::token_swaps::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};

pub const SLIPPAGE_TOLERANCE: u128 = 50; // 50 slippage tolerance points == 5%

pub struct ICPSwapClient {
    canister_id: CanisterId,
    token0: CanisterId,
    token1: CanisterId,
    pool: ICPSwapPool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepositFromSuccess {
    pub deposited_amount: u128,
}

impl ICPSwapClient {
    pub async fn new(token0: CanisterId, token1: CanisterId) -> ICPSwapClient {
        let pool = match Self::get_pool(token0.clone(), token1.clone()).await {
            Ok(pool) => pool,
            Err(e) => trap(format!("Failed to get pool (ICPSWAP): {}", e).as_str()),
        };

        let canister_id = pool.canisterId;

        ICPSwapClient {
            canister_id,
            token0, // token0 may be token1 in the pool and vice versa
            token1, // token1 may be token0 in the pool and vice versa
            pool,
        }
    }

    fn is_zero_for_one_swap_direction(&self) -> bool {
        let token0_str = self.token0.to_text();
        let token1_str = self.token1.to_text();

        match (self.pool.token0.address.as_str(), self.pool.token1.address.as_str()) {
            (t0, t1) if t0 == token0_str && t1 == token1_str => true,
            (t0, t1) if t0 == token1_str && t1 == token0_str => false,
            (t0, t1) => trap(
                format!(
                    "Invalid token configuration for ICPSwap pool: Expected tokens {:?} and {:?}, but got pool with token0={}, token1={}",
                    self.token0,
                    self.token1,
                    t0,
                    t1
                ).as_str()
            ),
        }
    }

    fn get_tokens_fee(&self, token_meta: &TokenMeta) -> TokensFee {
        let token0_str = self.token0.to_text();
        let token1_str = self.token1.to_text();

        match (self.pool.token0.address.as_str(), self.pool.token1.address.as_str()) {
            (t0, t1) if t0 == token0_str && t1 == token1_str => TokensFee {
                token0_fee: token_meta.token0Fee.clone(),
                token1_fee: token_meta.token1Fee.clone(),
            },
            (t0, t1) if t0 == token1_str && t1 == token0_str => TokensFee {
                token0_fee: token_meta.token1Fee.clone(),
                token1_fee: token_meta.token0Fee.clone(),
            },
            (t0, t1) => trap(
                format!(
                    "Invalid token configuration for ICPSwap pool: Expected tokens {:?} and {:?}, but got pool with token0={}, token1={}", 
                    self.token0,
                    self.token1,
                    t0,
                    t1
                ).as_str()
            ),
        }
    }

    async fn get_pool(token0: CanisterId, token1: CanisterId) -> Result<ICPSwapPool, String> {
        match icpswap_provider::get_pool(token0, token1).await {
            Ok(pool) => Ok(pool),
            Err(e) => Err(format!("Failed to get pool (ICPSWAP): {}", e)),
        }
    }

    async fn get_token_meta(&self) -> Result<TokenMeta, String> {
        match icpswap_provider::get_token_meta(self.canister_id).await {
            Ok(token_meta) => Ok(token_meta),
            Err(e) => Err(format!("Failed to get token meta (ICPSWAP): {}", e)),
        }
    }
    
    async fn deposit_from(&self, amount: Nat, token_fee: Nat) -> Result<Nat, String> {
        match icpswap_provider::deposit_from(self.canister_id, self.token0.clone(), amount, token_fee).await {
            Ok(deposited_amount) => Ok(deposited_amount),
            Err(e) => Err(format!("Failed to deposit_from (ICPSWAP): {}", e)),
        }
    }

    async fn withdraw(&self, amount: Nat, token_fee: Nat) -> Result<Nat, String> {
        match icpswap_provider::withdraw(self.canister_id, self.token1.clone(), amount, token_fee).await {
            Ok(withdrawn_amount) => Ok(withdrawn_amount),
            Err(e) => Err(format!("Failed to withdraw (ICPSWAP): {}", e)),
        }
    }

    async fn quote(&self, amount: Nat) -> Result<Nat, String> {
        match icpswap_provider::quote(self.canister_id, amount.clone(), self.is_zero_for_one_swap_direction(), amount).await {
            Ok(quote_amount) => Ok(quote_amount),
            Err(e) => Err(format!("Failed to quote (ICPSWAP): {}", e)),
        }
    }

    async fn swap(&self, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, String> {
        match icpswap_provider::swap(self.canister_id, amount_in, zero_for_one, amount_out_minimum).await {
            Ok(amount_out) => Ok(amount_out),
            Err(e) => Err(format!("Failed to swap (ICPSWAP): {}", e)),
        }
    }
}

#[async_trait]
impl SwapClient for ICPSwapClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn swap(&self, amount: u128) -> Result<Result<SwapSuccess, String>, (RejectCode, String)> {
        // Flow:
        // 1. Deposit from token0 to ICPSwap
        // 2. Swap
        // 3. Withdraw from ICPSwap to token1

        // TODO: Fix token meta fetching
        // let token_meta = match self.get_token_meta().await {
        //     Ok(token_meta) => token_meta,
        //     Err(e) => trap(format!("Failed to get token meta (ICPSWAP): {}", e).as_str()),
        // };

        // let tokens_fee = self.get_tokens_fee(&token_meta);
        // let token0_fee = tokens_fee.token0_fee.unwrap_or(Nat::from(0u8));
        // let token1_fee = tokens_fee.token1_fee.unwrap_or(Nat::from(0u8));

        //TODO: Remove hardcoded fees
        let token0_fee = Nat::from(10_000u128); // For PANDA
        let token1_fee = Nat::from(10_000u128); // For ICP

        // 1. Deposit
        let deposited_amount = match self.deposit_from(
            Nat::from(amount as u128),
            token0_fee
        ).await {
            Ok(amt) => amt,
            Err(e) => trap(format!("Failed to deposit_from (ICPSWAP): {}", e).as_str()),
        };

        // 2. Quote
        let expected_out = match self.quote(deposited_amount.clone()).await {
            Ok(result) => result,
            Err(e) => trap(format!("Failed to quote (ICPSWAP): {:?}", e).as_str()),
        };

        // 3. Swap
        let expected_out_u128 = nat_to_u128(&expected_out);
        let amount_out_minimum = Nat::from(
            expected_out_u128 * (1000 - SLIPPAGE_TOLERANCE) / 1000u128 // consider slippage tolerance
        );

        // panic!("amount_out_minimum {:?}", amount_out_minimum);

        let amount_out = match self.swap(deposited_amount.clone(), self.is_zero_for_one_swap_direction(), amount_out_minimum.clone()).await {
            Ok(amt) => amt,
            Err(e) => {
                // If swap fails, withdraw the deposited amount
                match self.withdraw(deposited_amount.clone(), token1_fee).await {
                    Ok(amt) => amt,
                    Err(e) => trap(format!("Failed to withdraw after failed swap (ICPSWAP): {}", e).as_str()),
                };

                trap(format!("Swap error 2 (ICPSWAP): {:?}", e).as_str());
            }
        };

        // panic!("amount {:?}, deposited_amount {:?}, amount_out {:?}, amount_out_minimum {:?}, expected_out {:?}", amount, deposited_amount, amount_out, amount_out_minimum, expected_out);

        // 4. Withdraw
        let withdrawn_amount = match self.withdraw(amount_out, token1_fee).await {
            Ok(amt) => amt,
            Err(e) => trap(format!("Failed to withdraw (ICPSWAP): {}", e).as_str()),
        };

        Ok(Ok(SwapSuccess {
            amount_out: nat_to_u128(&withdrawn_amount),
            withdrawal_success: Some(true),
        }))
    }

    async fn quote(&self, amount: u128) -> Result<Result<QuoteSuccess, String>, (RejectCode, String)> {
        match self.quote(Nat::from(amount as u128)).await {
            Ok(quote_amount) => Ok(Ok(QuoteSuccess {
                amount_out: nat_to_u128(&quote_amount),
            })),
            Err(e) => Err((RejectCode::CanisterError, format!("Failed to quote (ICPSWAP): {}", e))),
        }
    }
}
