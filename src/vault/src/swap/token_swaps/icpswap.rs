use super::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};
use serde::{Deserialize, Serialize};
use candid::Nat;
use std::str::FromStr;
use crate::swap::token_swaps::nat_to_u128;
use async_trait::async_trait;
use ic_cdk::trap;

use ic_response_codes::RejectCode;
use types::exchanges::TokenInfo;
use types::CanisterId;
use icpswap_swap_factory_canister::ICPSwapToken;
use icpswap_swap_factory_canister::ICPSwapPool;
use icpswap_swap_factory_canister::ICPSwapSwapFactoryResult;
use icpswap_swap_pool_canister::ICPSwapSwapPoolResult;
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;

pub const SWAP_FACTORY_CANISTER: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 0, 208, 10, 215, 1, 1]);
pub const SWAP_FEE: u128 = 3000;
pub const ICRC2_TOKEN_STANDARD: &str = "ICRC2";
pub const ICP_TOKEN_STANDARD: &str = "ICP";
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

    fn make_icpswap_token(token: &TokenInfo) -> ICPSwapToken {
        let standard = match token.symbol.as_str() {
            "ICP" => ICP_TOKEN_STANDARD.to_string(),
            _ => ICRC2_TOKEN_STANDARD.to_string(),
        };

        ICPSwapToken {
            address: token.ledger.to_string(),
            standard,
        }
    }

    fn get_token_fee(&self, token: &TokenInfo) -> u128 {
        let token_address = token.ledger.to_string();

        match (self.pool.token0.address.as_str(), self.pool.token1.address.as_str()) {
            (t0, _) if t0 == token_address => {
                // If token is token0, use token0Fee
                match &self.token_meta.token0Fee {
                    Some(nat) => nat_to_u128(nat.clone()),
                    None => 0
                }
            },
            (_, t1) if t1 == token_address => {
                // If token is token1, use token1Fee
                match &self.token_meta.token1Fee {
                    Some(nat) => nat_to_u128(nat.clone()),
                    None => 0
                }
            },
            _ => {
                // Should fall into one of the above cases
                0
            }
        }
    }

    async fn get_pool(token_in: TokenInfo, token_out: TokenInfo) -> Result<ICPSwapPool, String> {
        let pool_args = &icpswap_swap_factory_canister::getPool::Args {
            fee: candid::Nat::from(SWAP_FEE as u128),
            token0: Self::make_icpswap_token(&token_in),
            token1: Self::make_icpswap_token(&token_out),
        };

        match icpswap_swap_factory_canister_c2c_client::getPool(SWAP_FACTORY_CANISTER, pool_args).await {
            Ok(response) => {
                match response {
                    ICPSwapSwapFactoryResult::Ok(pool) => {
                        Ok(pool)
                    },
                    ICPSwapSwapFactoryResult::Err(error) => {
                        Err(format!("ICPSwap get pool canister id failed 2: {:?}", error))
                    },
                }
            },
            Err(e) => {
                Err(format!("ICPSwap get pool canister id failed 1: {:?}", e))
            },
        }
    }

    async fn get_token_meta(canister_id: CanisterId) -> Result<TokenMeta, String> {
        match icpswap_swap_pool_canister_c2c_client::getTokenMeta(canister_id).await {
            Ok(response) => {
                match response {
                    ICPSwapSwapPoolResult::Ok(token_meta) => {
                        Ok(token_meta)
                    }
                    ICPSwapSwapPoolResult::Err(error) => {
                        Err(format!("ICPSwap get token meta canister id failed 2: {:?}", error))
                    }
                }
            }
            Err(e) => {
                Err(format!("ICPSwap get token meta canister id failed 1: {:?}", e))
            }
        }
    }
    
    async fn deposit_from(&self, amount: u128) -> Result<u128, String> {
        let args = &icpswap_swap_pool_canister::depositFrom::Args {
            token: self.token_in.ledger.to_string(),
            amount: amount.into(),
            fee: self.get_token_fee(&self.token_in).into(),
        };

        match icpswap_swap_pool_canister_c2c_client::depositFrom(self.canister_id, args).await {
            Ok(response) => {
                match response {
                    ICPSwapSwapPoolResult::Ok(deposited_amount_nat) => {
                        Ok(nat_to_u128(deposited_amount_nat))
                    }
                    ICPSwapSwapPoolResult::Err(error) => {
                        Err(format!("Deposit from error 2 (ICPSWAP) : {:?} arguments {:?}", error, args))
                    }
                }
            }
            Err(error) => {
                Err(format!("Deposit from error 1 (ICPSWAP) : {:?} arguments {:?}", error, args))
            }
        }
    }

    async fn withdraw(&self, amount: u128) -> Result<u128, String> {
        let args = &icpswap_swap_pool_canister::withdraw::Args {
            token: self.token_out.ledger.to_string(),
            amount: amount.into(),
            fee: self.get_token_fee(&self.token_out).into(),
        };

        match icpswap_swap_pool_canister_c2c_client::withdraw(self.canister_id, args).await {
            Ok(response) => {
                match response {
                    ICPSwapSwapPoolResult::Ok(withdrawn_amount_nat) => {
                        Ok(nat_to_u128(withdrawn_amount_nat))
                    }
                    ICPSwapSwapPoolResult::Err(error) => {
                        Err(format!("Withdraw error 2 (ICPSWAP) : {:?} arguments {:?}", error, args))
                    }
                }
            }
            Err(error) => {
                Err(format!("Withdraw error 1 (ICPSWAP) : {:?} arguments {:?}", error, args))
            }
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

        let deposited_amount = match self.deposit_from(amount).await {
            Ok(amt) => amt,
            Err(e) => trap(format!("Failed to deposit_from (ICPSWAP): {}", e).as_str()),
        };

        let expected_out = match self.quote(deposited_amount).await {
            Ok(result) => {
                match result {
                    Ok(quote) => quote.amount_out,
                    Err(e) => trap(format!("Failed to quote (ICPSWAP): {}", e).as_str()),
                }
            }
            Err(e) => trap(format!("Failed to quote (ICPSWAP): {:?}", e).as_str()),
        };

        let swap_args = &icpswap_swap_pool_canister::swap::Args {
            amountIn: deposited_amount.to_string(),
            zeroForOne: self.is_zero_for_one_swap_direction(),
            amountOutMinimum: (expected_out * (1000 - SLIPPAGE_TOLERANCE) / 1000).to_string(),
        };

        match icpswap_swap_pool_canister_c2c_client::swap(self.canister_id, swap_args).await {
            Ok(response) => {
                match response {
                    ICPSwapSwapPoolResult::Ok(amount_out_nat) => {
                        let amount_out = nat_to_u128(amount_out_nat) * 50 / 100;
                        let withdrawn_amount = match self.withdraw(amount_out).await {
                            Ok(amount) => amount,
                            Err(e) => trap(format!("Failed to withdraw (ICPSWAP): {}", e).as_str()),
                        };
                        Ok(Ok(SwapSuccess {
                            amount_out: withdrawn_amount,
                            withdrawal_success: Some(true),
                        }))
                    }
                    ICPSwapSwapPoolResult::Err(error) => {
                        match self.withdraw(deposited_amount).await {
                            Ok(amt) => amt,
                            Err(e) => trap(format!("Failed to withdraw after failed swap (ICPSWAP): {}", e).as_str()),
                        };

                        trap(format!("Swap error 2 : {:?} arguments {:?}", error, swap_args).as_str());
                    }
                }
            }
            Err(error) => {
                trap(format!("Swap error 1 : {:?} arguments {:?}", error, swap_args).as_str());
            },
        }
    }

    async fn quote(&self, amount: u128) -> Result<Result<QuoteSuccess, String>, (RejectCode, String)> {
        let quote_args = &icpswap_swap_pool_canister::quote::Args {
            amountIn: amount.to_string(),
            zeroForOne: self.is_zero_for_one_swap_direction(),
            amountOutMinimum: amount.to_string(),
        };

        match icpswap_swap_pool_canister_c2c_client::quote(self.canister_id, quote_args).await {
            Ok(response) => {
                match response {
                    ICPSwapSwapPoolResult::Ok(amount_out_nat) => {
                        Ok(Ok(QuoteSuccess {
                            amount_out: nat_to_u128(amount_out_nat),
                        }))
                    }
                    ICPSwapSwapPoolResult::Err(error) => {
                        trap(format!("Quote error 2 (ICPSWAP) : {:?} arguments {:?}", error, quote_args).as_str());
                    }
                }
            }
            Err(error) => {
                trap(format!("Quote error 1 (ICPSWAP) : {:?} arguments {:?}", error, quote_args).as_str());
            }
        }
    }
}

