use super::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};
use utils::util::nat_to_u128;
use async_trait::async_trait;
use ic_cdk::trap;
use ic_response_codes::RejectCode;
use types::CanisterId;

const SLIPPAGE_PERCENTAGE: f64 = 40.0; // TODO: Fix this

pub struct KongSwapClient {
    canister_id: CanisterId,
    token_in: CanisterId,
    token_out: CanisterId,
}

impl KongSwapClient {
    pub fn new(canister_id: CanisterId, token_in: CanisterId, token_out: CanisterId) -> KongSwapClient {
        KongSwapClient {
            canister_id,
            token_in,
            token_out,
        }
    }

    fn token_kongswap_format(&self, token: CanisterId) -> String {
        format!("IC.{}", token.to_text())
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
            pay_token: self.token_kongswap_format(self.token_in.clone()),
            receive_token: self.token_kongswap_format(self.token_out.clone()),
            max_slippage: Some(SLIPPAGE_PERCENTAGE),
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
                        let amount_out = nat_to_u128(&response.receive_amount);
                        Ok(Ok(SwapSuccess {
                            amount_out,
                            withdrawal_success: Some(response.claim_ids.is_empty()),
                        }))
                    }
                    Err(error) => {
                        trap(format!("KongSwapClient::swap: swap error 1: {:?} arguments {:?}", error, args).as_str());
                    }
                }
            }
            Err(error) => {
                trap(format!("KongSwapClient::swap: swap error 2: {:?} arguments {:?}", error, args).as_str());
            },
        }
    }

    async fn quote(&self, amount: u128) -> Result<Result<QuoteSuccess, String>, (RejectCode, String)> {
        let pay_token = self.token_kongswap_format(self.token_in.clone());
        let amount_nat = candid::Nat::from(amount);
        let receive_token = self.token_kongswap_format(self.token_out.clone());

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
                            amount_out: nat_to_u128(&response.receive_amount),
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
