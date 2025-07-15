use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub positionId: Nat,
}

pub type Response = ICPSwapSwapPoolResult<ClaimResponse>;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct ClaimResponse {
    pub amount0: Nat,
    pub amount1: Nat,
}
