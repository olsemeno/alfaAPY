use async_trait::async_trait;
use candid::{Deserialize, Nat};
use serde::Serialize;
use types::CanisterId;

use errors::internal_error::error::InternalError;

#[async_trait]
pub trait SwapClient: Send + Sync + 'static {
    fn canister_id(&self) -> CanisterId;
    async fn swap(&self, amount: Nat) -> Result<SwapSuccess, InternalError>;
    async fn quote(&self, amount: Nat) -> Result<QuoteSuccess, InternalError>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SwapSuccess {
    pub amount_out: u128,
    pub withdrawal_success: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuoteSuccess {
    pub amount_out: u128,
}
