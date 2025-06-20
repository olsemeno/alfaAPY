use async_trait::async_trait;
use types::CanisterId;
use candid::Nat;

use super::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};
use errors::internal_error::error::InternalError;
use providers::kongswap as kongswap_provider;
use utils::util::nat_to_u128;

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
        ).await?;

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
        ).await?;

        Ok(QuoteSuccess {
            amount_out: nat_to_u128(&result.receive_amount),
        })
    }
}
