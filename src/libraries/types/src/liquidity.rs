use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct WithdrawFromPoolResponse {
    pub token_0_amount: Nat,
    pub token_1_amount: Nat,
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct AddLiquidityResponse {
    pub token_0_amount: Nat,
    pub token_1_amount: Nat,
    pub request_id: u64,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct TokensFee {
    pub token0_fee: Option<Nat>,
    pub token1_fee: Option<Nat>,
}
