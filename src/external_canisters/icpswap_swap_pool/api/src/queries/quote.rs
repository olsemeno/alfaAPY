use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Args {
    pub amountIn: String,
    pub zeroForOne: bool,
    pub amountOutMinimum: String,
}

pub type Response = ICPSwapSwapPoolResult<Nat>;
