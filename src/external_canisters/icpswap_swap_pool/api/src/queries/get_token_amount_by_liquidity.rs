use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

pub type Args = (Nat, i32, i32, Nat,);

pub type Response = Result<GetTokenAmountByLiquidityReply, String>;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenAmountByLiquidityReply {
    pub amount0: i32,
    pub amount1: i32,
}
