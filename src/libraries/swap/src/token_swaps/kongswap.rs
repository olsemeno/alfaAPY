use async_trait::async_trait;
use types::CanisterId;
use candid::Nat;
use std::sync::Arc;

use super::swap_client::{SwapClient, SwapSuccess, QuoteSuccess};
use errors::internal_error::error::InternalError;
use providers::kongswap::KongSwapProvider;
use utils::util::nat_to_u128;

const SLIPPAGE_PERCENTAGE: f64 = 40.0; // TODO: Improve slippage settings

pub struct KongSwapSwapClient {
    provider_impl: Arc<dyn KongSwapProvider + Send + Sync>,
    canister_id: CanisterId,
    token_in: CanisterId,
    token_out: CanisterId,
}

impl KongSwapSwapClient {
    pub fn new(
        provider_impl: Arc<dyn KongSwapProvider + Send + Sync>,
        canister_id: CanisterId,
        token_in: CanisterId,
        token_out: CanisterId,
    ) -> Self {
        Self {
            provider_impl,
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
        let result = self.provider_impl.swap(
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
        let result = self.provider_impl.swap_amounts(
            self.token_in.clone(),
            amount.clone(),
            self.token_out.clone(),
        ).await?;

        Ok(QuoteSuccess {
            amount_out: nat_to_u128(&result.receive_amount),
        })
    }
}
