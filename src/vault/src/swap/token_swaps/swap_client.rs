use async_trait::async_trait;
use candid::Deserialize;
use ic_response_codes::RejectCode;
use serde::Serialize;
use types::CanisterId;

#[async_trait]
pub trait SwapClient {
    fn canister_id(&self) -> CanisterId;
    async fn swap(&self, amount: u128) -> Result<Result<SwapSuccess, String>, (RejectCode, String)>;
    async fn quote(&self, amount: u128) -> Result<Result<QuoteSuccess, String>, (RejectCode, String)>;
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
