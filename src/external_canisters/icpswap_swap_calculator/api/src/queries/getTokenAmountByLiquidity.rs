use candid::{CandidType, Nat, Int};
use serde::{Deserialize, Serialize};

pub type Args = (Nat, Int, Int, Nat);
pub type Response = (GetTokenAmountByLiquidityResponse,);

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenAmountByLiquidityResponse {
    pub amount0: Nat,
    pub amount1: Nat,
}
