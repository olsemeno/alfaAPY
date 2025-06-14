use super::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};
use utils::util::nat_to_u128;
use async_trait::async_trait;
use types::CanisterId;
use std::collections::HashMap;
use candid::Nat;

use errors::internal_error::error::InternalError;
use providers::kongswap as kongswap_provider;

const SLIPPAGE_PERCENTAGE: f64 = 40.0; // TODO: Fix this

pub struct KongSwapSwapClient {
    canister_id: CanisterId,
    token_in: CanisterId,
    token_out: CanisterId,
}

impl KongSwapSwapClient {
    pub fn new(canister_id: CanisterId, token_in: CanisterId, token_out: CanisterId) -> KongSwapSwapClient {
        KongSwapSwapClient {
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
impl SwapClient for KongSwapSwapClient {
    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    async fn swap(&self, amount: Nat) -> Result<SwapSuccess, InternalError> {
        let result = kongswap_provider::swap(
            self.token_in.clone(),
            amount.clone(),
            self.token_out.clone(),
            Some(SLIPPAGE_PERCENTAGE),
        ).await
        .map_err(|error| {
            error.wrap(
                "KongSwapSwapClient::swap".to_string(),
                "Error calling 'kongswap_provider::swap'".to_string(),
                Some(HashMap::from([
                    ("token_in".to_string(), self.token_in.to_text()),
                    ("token_out".to_string(), self.token_out.to_text()),
                    ("amount".to_string(), amount.to_string()),
                    ("max_slippage".to_string(), SLIPPAGE_PERCENTAGE.to_string()),
                ])),
            )
        })?;

        Ok(SwapSuccess {
            amount_out: nat_to_u128(&result.receive_amount),
            withdrawal_success: Some(result.claim_ids.is_empty()),
        })
    }

    async fn quote(&self, amount: Nat) -> Result<QuoteSuccess, InternalError> {
        let result = kongswap_provider::swap_amounts(
            self.token_in.clone(),
            amount.clone(),
            self.token_out.clone(),
        ).await
        .map_err(|error| {
            error.wrap(
                "KongSwapSwapClient::quote".to_string(),
                "Error calling 'kongswap_provider::swap_amounts'".to_string(),
                Some(HashMap::from([
                    ("token_in".to_string(), self.token_in.to_text()),
                    ("token_out".to_string(), self.token_out.to_text()),
                    ("amount".to_string(), amount.to_string()),
                ])),
            )
        })?;

        Ok(QuoteSuccess {
            amount_out: nat_to_u128(&result.receive_amount),
        })
    }
}
