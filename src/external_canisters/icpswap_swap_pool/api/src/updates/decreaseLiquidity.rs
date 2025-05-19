use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub liquidity: String,
    pub positionId: Nat,
}

pub type Response = ICPSwapSwapPoolResult<DecreaseLiquidityResponse>;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct DecreaseLiquidityResponse {
    pub amount0: Nat,
    pub amount1: Nat,
}
