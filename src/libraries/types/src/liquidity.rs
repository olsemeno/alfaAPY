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
    pub request_id: u64, // TODO: rename to position_id
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct TokensFee {
    pub token0_fee: Option<Nat>,
    pub token1_fee: Option<Nat>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct GetPositionByIdResponse {
    pub position_id: u64,
    pub token_0_amount: Nat,
    pub token_1_amount: Nat,
    pub usd_amount_0: Nat,
    pub usd_amount_1: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct GetPoolDataResponse {
    pub tvl: Nat,
}
