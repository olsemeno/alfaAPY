use super::swap_client::{SwapClient, SwapSuccess};
use crate::token_swaps::nat_to_u128;
use async_trait::async_trait;
use ic_response_codes::RejectCode;
use types::exchanges::TokenInfo;
use types::CanisterId;
use crate::token_swaps::swap_service::SNS_GOVERNANCE_CANISTER_ID;

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

#[async_trait]
impl SwapClient for KongSwapClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn swap(&self, amount: u128, min_amount_out: u128) -> Result<Result<SwapSuccess, String>, (RejectCode, String)> {
        match kongswap_canister_c2c_client::swap(
            self.canister_id,
            &kongswap_canister::swap::Args {
                pay_amount: amount.into(),
                pay_token: format!("IC.{}", self.token_in.ledger),
                receive_amount: Some(min_amount_out.into()),
                receive_token: format!("IC.{}", self.token_out.ledger),
                referred_by: Some(SNS_GOVERNANCE_CANISTER_ID.to_string()),
            },
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
                    Err(error) => Ok(Err(error))
                }
            }
            Err(error) => Ok(Err(format!("{:?}", error))),
        }
    }
}
