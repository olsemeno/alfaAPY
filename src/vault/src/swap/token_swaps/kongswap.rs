use super::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};
use crate::swap::token_swaps::nat_to_u128;
use async_trait::async_trait;
use ic_cdk::trap;
use ic_response_codes::RejectCode;
use types::exchanges::TokenInfo;
use types::CanisterId;

pub struct KongSwapClient {
    canister_id: CanisterId,
    token_in: TokenInfo,
    token_out: TokenInfo,
}

impl KongSwapClient {
    pub fn new(canister_id: CanisterId, token_in: TokenInfo, token_out: TokenInfo) -> KongSwapClient {
        KongSwapClient {
            canister_id,
            token_in,
            token_out,
        }
    }
}

// #[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
// pub struct SwapArgs {
//     pub pay_token: String,
//     pub pay_amount: Nat,
//     // pub pay_tx_id: Option<TxId>,
//     pub receive_token: String,
//     pub receive_amount: Option<Nat>,
//     pub receive_address: Option<String>,
//     pub max_slippage: Option<f64>,
//     pub referred_by: Option<String>,
// }


#[async_trait]
impl SwapClient for KongSwapClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn swap(&self, amount: u128) -> Result<Result<SwapSuccess, String>, (RejectCode, String)> {
        let args = &kongswap_canister::swap::Args {
            pay_amount: amount.into(),
            pay_token: format!("IC.{}", self.token_in.ledger),
            receive_token: format!("IC.{}", self.token_out.ledger),
            max_slippage: Some(10.0),
        };

        match kongswap_canister_c2c_client::swap(
            self.canister_id,
            args,
        )
        .await
        {
            Ok(response) => {
                match response {
                    Ok(response) => {
                        let amount_out = nat_to_u128(response.receive_amount);
                        Ok(Ok(SwapSuccess {
                            amount_out,
                            withdrawal_success: Some(response.claim_ids.is_empty()),
                        }))
                    }
                    Err(error) => {
                        trap(format!("Swap error 3 : {:?} arguments {:?}", error, args).as_str());
                    }
                }
            }
            Err(error) => {
                trap(format!("Swap error 4 : {:?} arguments {:?}", error, args).as_str());
            },
        }
    }

    async fn quote(&self, amount: u128) -> Result<Result<QuoteSuccess, String>, (RejectCode, String)> {
        let pay_token = format!("IC.{}", self.token_in.ledger);
        let amount_nat = candid::Nat::from(amount);
        let receive_token = format!("IC.{}", self.token_out.ledger);

        let args = (
            pay_token,
            amount_nat,
            receive_token,
        );

        let args_debug = format!("{:?}", args);

        match kongswap_canister_c2c_client::swap_amounts(self.canister_id, args).await {
            Ok((response,)) => {
                match response {
                    Ok(response) => {
                        Ok(Ok(QuoteSuccess {
                            amount_out: nat_to_u128(response.receive_amount),
                        }))
                    }
                    Err(error) => {
                        trap(format!("Quote error 1 (KONGSWAP) : {:?} arguments {}", error, args_debug).as_str());
                    }
                }
            }
            Err(error) => {
                trap(format!("Quote error 2 (KONGSWAP) : {:?} arguments {}", error, args_debug).as_str());
            }
        }
    }
}
