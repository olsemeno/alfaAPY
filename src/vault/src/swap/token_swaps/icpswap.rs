use super::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};
use serde::{Deserialize, Serialize};
use candid::Nat;
use crate::swap::token_swaps::nat_to_u128;
use async_trait::async_trait;
use ic_cdk::trap;
use ic_response_codes::RejectCode;
use types::exchanges::TokenInfo;
use types::CanisterId;

pub struct ICPSwapClient {
    canister_id: CanisterId,
    token_in: TokenInfo,
    token_out: TokenInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepositFromSuccess {
    pub deposited_amount: u128,
}

impl ICPSwapClient {
    pub fn new(canister_id: CanisterId, token_in: TokenInfo, token_out: TokenInfo) -> ICPSwapClient {
        ICPSwapClient {
            canister_id,
            token_in,
            token_out,
        }
    }
    
    async fn deposit_from(&self, amount: u128) -> Result<u128, String> {
        let args = &icpswap_swap_pool_canister::deposit_from::Args {
            token: format!("IC.{}", self.token_in.ledger),
            amount: amount.into(),
            fee: candid::Nat::from(10000 as u128), // TODO: Move to constant or config
        };

        match icpswap_swap_pool_canister_c2c_client::deposit_from(self.canister_id, args).await {
            Ok(response) => {
                match response {
                    Ok(deposited_amount_nat) => {
                        Ok(nat_to_u128(deposited_amount_nat))
                    }
                    Err(error) => {
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
            token: format!("IC.{}", self.token_out.ledger),
            amount: amount.into(),
            fee: candid::Nat::from(10000 as u128), // TODO: Move to constant or config
        };

        match icpswap_swap_pool_canister_c2c_client::withdraw(self.canister_id, args).await {
            Ok(response) => {
                match response {
                    Ok(withdrawn_amount_nat) => {
                        Ok(nat_to_u128(withdrawn_amount_nat))
                    }
                    Err(error) => {
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

        let swap_args = &icpswap_swap_pool_canister::swap::Args {
            amountIn: deposited_amount.to_string(),
            zeroForOne: true,
            amountOutMinimum: deposited_amount.to_string(), // TODO: Check for slippage
        };

        match icpswap_swap_pool_canister_c2c_client::swap(self.canister_id, swap_args).await {
            Ok(response) => {
                match response {
                    Ok(amount_out_nat) => {
                        let amount_out = nat_to_u128(amount_out_nat);
                        let withdrawn_amount = match self.withdraw(amount_out).await {
                            Ok(amt) => amt,
                            Err(e) => trap(format!("Failed to withdraw (ICPSWAP): {}", e).as_str()),
                        };
                        Ok(Ok(SwapSuccess {
                            amount_out: withdrawn_amount,
                            withdrawal_success: Some(true),
                        }))
                    }
                    Err(error) => {
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
            zeroForOne: true,
            amountOutMinimum: amount.to_string(),
        };

        match icpswap_swap_pool_canister_c2c_client::quote(self.canister_id, quote_args).await {
            Ok(response) => {
                match response {
                    Ok(amount_out_nat) => {
                        Ok(Ok(QuoteSuccess {
                            amount_out: nat_to_u128(amount_out_nat),
                        }))
                    }
                    Err(error) => {
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
