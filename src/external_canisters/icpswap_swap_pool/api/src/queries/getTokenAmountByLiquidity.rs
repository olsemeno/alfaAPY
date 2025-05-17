use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

pub type Args = (Nat, i32, i32, Nat,);

pub type Response = ICPSwapSwapPoolResult<GetTokenAmountByLiquidityReply, String>;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenAmountByLiquidityReply {
    pub amount0: i32,
    pub amount1: i32,
}
