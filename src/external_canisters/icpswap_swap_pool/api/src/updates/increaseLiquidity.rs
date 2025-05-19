use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub positionId: Nat,
    pub amount0Desired: String,
    pub amount1Desired: String,
}

pub type Response = ICPSwapSwapPoolResult<Nat>;
