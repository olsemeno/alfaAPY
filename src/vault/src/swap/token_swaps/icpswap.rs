use super::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};
use serde::{Deserialize, Serialize};
use crate::swap::token_swaps::nat_to_u128;
use async_trait::async_trait;
use ic_cdk::trap;
use candid::Nat;

use ic_response_codes::RejectCode;
use types::exchanges::TokenInfo;
use types::CanisterId;

use crate::providers::icpswap::icpswap::{get_pool, get_token_meta, deposit_from, withdraw, quote, swap};
use icpswap_swap_factory_canister::ICPSwapPool;
use icpswap_swap_pool_canister::ICPSwapSwapPoolResult;
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;

pub const SLIPPAGE_TOLERANCE: u128 = 50; // 5%

pub struct ICPSwapClient {
    canister_id: CanisterId,
    token_in: TokenInfo,
    token_out: TokenInfo,
    pool: ICPSwapPool,
    token_meta: TokenMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepositFromSuccess {
    pub deposited_amount: u128,
}

impl ICPSwapClient {
    pub async fn new(token_in: TokenInfo, token_out: TokenInfo) -> ICPSwapClient {
        let pool = match Self::get_pool(token_in.clone(), token_out.clone()).await {
            Ok(pool) => pool,
            Err(e) => trap(format!("Failed to get pool (ICPSWAP): {}", e).as_str()),
        };

        let canister_id = pool.canisterId;

        let token_meta = match Self::get_token_meta(canister_id).await {
            Ok(token_meta) => token_meta,
            Err(e) => trap(format!("Failed to get token meta (ICPSWAP): {}", e).as_str()),
        };

        ICPSwapClient {
            canister_id,
            token_in,
            token_out,
            pool,
            token_meta,
        }
    }

    fn is_zero_for_one_swap_direction(&self) -> bool {
        let token_in_str = self.token_in.ledger.to_string();
        let token_out_str = self.token_out.ledger.to_string();

        match (self.pool.token0.address.as_str(), self.pool.token1.address.as_str()) {
            (t0, t1) if t0 == token_in_str && t1 == token_out_str => true,
            (t0, t1) if t0 == token_out_str && t1 == token_in_str => false,
            (t0, t1) => trap(
                format!(
                    "Invalid token configuration for ICPSwap pool: Expected tokens {:?} and {:?}, but got pool with token0={}, token1={}", 
                    self.token_in,
                    self.token_out,
                    t0,
                    t1
                ).as_str()
            ),
        }
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

    async fn get_pool(token_in: TokenInfo, token_out: TokenInfo) -> Result<ICPSwapPool, String> {
        match get_pool(token_in, token_out).await {
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
    
    async fn deposit_from(&self, amount: Nat) -> Result<Nat, String> {
        match deposit_from(self.canister_id, self.token_in.clone(), amount, self.get_token_fee(&self.token_in)).await {
            Ok(deposited_amount) => Ok(deposited_amount),
            Err(e) => Err(format!("Failed to deposit_from (ICPSWAP): {}", e)),
        }
    }

    async fn withdraw(&self, amount: Nat) -> Result<Nat, String> {
        match withdraw(self.canister_id, self.token_out.clone(), amount, self.get_token_fee(&self.token_out)).await {
            Ok(withdrawn_amount) => Ok(withdrawn_amount),
            Err(e) => Err(format!("Failed to withdraw (ICPSWAP): {}", e)),
        }
    }

    async fn quote(&self, amount: Nat) -> Result<Nat, String> {
        match quote(self.canister_id, amount.clone(), self.is_zero_for_one_swap_direction(), amount).await {
            Ok(quote_amount) => Ok(quote_amount),
            Err(e) => Err(format!("Failed to quote (ICPSWAP): {}", e)),
        }
    }

    async fn swap(&self, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, String> {
        match swap(self.canister_id, amount_in, zero_for_one, amount_out_minimum).await {
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
        // 1. Deposit from token_in to ICPSwap
        // 2. Swap
        // 3. Withdraw from ICPSwap to token_out

        // 1. Deposit
        let deposited_amount = match self.deposit_from(Nat::from(amount as u128)).await {
            Ok(amt) => amt,
            Err(e) => trap(format!("Failed to deposit_from (ICPSWAP): {}", e).as_str()),
        };

        // 2. Quote
        let expected_out = match self.quote(deposited_amount.clone()).await {
            Ok(result) => result,
            Err(e) => trap(format!("Failed to quote (ICPSWAP): {:?}", e).as_str()),
        };

        // 3. Swap
        let expected_out_u128 = nat_to_u128(expected_out);
        let amount_out_minimum = Nat::from(expected_out_u128 * (1000 - SLIPPAGE_TOLERANCE) / 1000u128);

        let amount_out = match self.swap(deposited_amount.clone(), self.is_zero_for_one_swap_direction(), amount_out_minimum).await {
            Ok(amt) => amt,
            Err(e) => {
                // If swap fails, withdraw the deposited amount
                match self.withdraw(deposited_amount.clone()).await {
                    Ok(amt) => amt,
                    Err(e) => trap(format!("Failed to withdraw after failed swap (ICPSWAP): {}", e).as_str()),
                };

                trap(format!("Swap error 2 (ICPSWAP): {:?}", e).as_str());
            }
        };

        // 4. Withdraw
        let withdrawn_amount = match self.withdraw(amount_out).await {
            Ok(amt) => amt,
            Err(e) => trap(format!("Failed to withdraw (ICPSWAP): {}", e).as_str()),
        };

        Ok(Ok(SwapSuccess {
            amount_out: nat_to_u128(withdrawn_amount),
            withdrawal_success: Some(true),
        }))
    }

    async fn quote(&self, amount: u128) -> Result<Result<QuoteSuccess, String>, (RejectCode, String)> {
        match self.quote(Nat::from(amount as u128)).await {
            Ok(quote_amount) => Ok(Ok(QuoteSuccess {
                amount_out: nat_to_u128(quote_amount),
            })),
            Err(e) => Err((RejectCode::CanisterError, format!("Failed to quote (ICPSWAP): {}", e))),
        }
    }
}