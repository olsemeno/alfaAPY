use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use candid::Nat;
use std::collections::HashMap;

use types::CanisterId;
use providers::{icpswap as icpswap_provider};
use icpswap_swap_factory_canister::ICPSwapPool;
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;
use types::liquidity::TokensFee;
use utils::util::nat_to_u128;
use utils::token_fees::get_token_fee;
use errors::internal_error::error::InternalError;
use errors::internal_error::error::build_error_code;

use crate::token_swaps::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};

pub const SLIPPAGE_TOLERANCE: u128 = 50; // 50 slippage tolerance points == 5%

pub struct ICPSwapSwapClient {
    canister_id: Option<CanisterId>,
    token0: CanisterId,
    token1: CanisterId,
    pool: Option<ICPSwapPool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepositFromSuccess {
    pub deposited_amount: u128,
}

impl ICPSwapSwapClient {
    pub fn new(token0: CanisterId, token1: CanisterId) -> ICPSwapSwapClient {
        ICPSwapSwapClient {
            canister_id: None,
            token0, // token0 may be token1 in the pool and vice versa
            token1, // token1 may be token0 in the pool and vice versa
            pool: None,
        }
    }

    pub async fn with_pool(mut self) -> Result<Self, InternalError> {
        let pool = Self::get_pool(self.token0.clone(), self.token1.clone()).await?;

        self.pool = Some(pool.clone());
        self.canister_id = Some(pool.canisterId);

        Ok(self)
    }

    fn is_zero_for_one_swap_direction(&self) -> Result<bool, InternalError> {
        let token0_str = self.token0.to_text();
        let token1_str = self.token1.to_text();

        let pool = self.pool.as_ref().unwrap();

        match (pool.token0.address.as_str(), pool.token1.address.as_str()) {
            (t0, t1) if t0 == token0_str && t1 == token1_str => Ok(true),
            (t0, t1) if t0 == token1_str && t1 == token0_str => Ok(false),
            (t0, t1) => Err(InternalError::business_logic(
                build_error_code(2002, 3, 1), // 2002 03 01
                "ICPSwapSwapClient::is_zero_for_one_swap_direction".to_string(),
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

    fn get_tokens_fee(&self, token_meta: &TokenMeta) -> Result<TokensFee, InternalError> {
        let token0_str = self.token0.to_text();
        let token1_str = self.token1.to_text();

        let pool = self.pool.as_ref().unwrap();

        match (pool.token0.address.as_str(), pool.token1.address.as_str()) {
            (t0, t1) if t0 == token0_str && t1 == token1_str => Ok(TokensFee {
                token0_fee: token_meta.token0Fee.clone(),
                token1_fee: token_meta.token1Fee.clone(),
            }),
            (t0, t1) if t0 == token1_str && t1 == token0_str => Ok(TokensFee {
                token0_fee: token_meta.token1Fee.clone(),
                token1_fee: token_meta.token0Fee.clone(),
            }),
            (t0, t1) => Err(InternalError::business_logic(
                build_error_code(2002, 3, 2), // 2002 03 02
                "ICPSwapSwapClient::get_tokens_fee".to_string(),
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

    async fn get_pool(token0: CanisterId, token1: CanisterId) -> Result<ICPSwapPool, InternalError> {
        icpswap_provider::get_pool(token0.clone(), token1.clone()).await
    }

    async fn get_token_meta(&self) -> Result<TokenMeta, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        icpswap_provider::get_token_meta(canister_id.clone()).await
    }
    
    async fn deposit_from(&self, amount: Nat, token_fee: Nat) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        icpswap_provider::deposit_from(
            canister_id.clone(),
            self.token0.clone(),
            amount.clone(),
            token_fee.clone()
        ).await
    }

    async fn withdraw(&self, amount: Nat, token_fee: Nat) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        icpswap_provider::withdraw(
            canister_id.clone(),
            self.token1.clone(),
            amount.clone(),
            token_fee.clone()
        ).await
    }

    async fn quote(&self, amount: Nat) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();
        let is_zero_for_one_swap_direction = self.is_zero_for_one_swap_direction()?;

        icpswap_provider::quote(
            canister_id.clone(),
            amount.clone(),
            is_zero_for_one_swap_direction,
            amount.clone()
        ).await
    }

    async fn swap(&self, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, InternalError> {
        let canister_id = self.canister_id.as_ref().unwrap();

        icpswap_provider::swap(
            canister_id.clone(),
            amount_in.clone(),
            zero_for_one,
            amount_out_minimum.clone()
        ).await
    }
}

#[async_trait]
impl SwapClient for ICPSwapSwapClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id.as_ref().unwrap().clone()
    }

    async fn swap(&self, amount: Nat) -> Result<SwapSuccess, InternalError> {
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
        let token0_fee = get_token_fee(self.token0.clone()).await;
        let token1_fee = get_token_fee(self.token1.clone()).await;

        // 1. Deposit
        let deposited_amount = self.deposit_from(
            amount.clone(),
            token0_fee.clone()
        ).await?;

        // 2. Quote
        let expected_out = self.quote(deposited_amount.clone()).await?;

        // 3. Swap
        let expected_out_u128 = nat_to_u128(&expected_out);
        // Сonsider slippage tolerance
        let amount_out_minimum = Nat::from(expected_out_u128 * (1000 - SLIPPAGE_TOLERANCE) / 1000u128);

        let amount_out = self.swap(
            deposited_amount.clone(),
            self.is_zero_for_one_swap_direction()?,
            amount_out_minimum.clone(),
        ).await?;

        // 4. Withdraw
        let withdrawn_amount = self.withdraw(amount_out, token1_fee.clone()).await?;

        Ok(SwapSuccess {
            amount_out: nat_to_u128(&withdrawn_amount),
            withdrawal_success: Some(true),
        })
    }

    async fn quote(&self, amount: Nat) -> Result<QuoteSuccess, InternalError> {
        let quote_amount = self.quote(amount.clone()).await?;

        Ok(QuoteSuccess {
            amount_out: nat_to_u128(&quote_amount),
        })
    }
}
