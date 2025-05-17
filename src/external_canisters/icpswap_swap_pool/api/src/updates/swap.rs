use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub amountIn : String,
    pub zeroForOne : bool,
    pub amountOutMinimum : String,
}

pub type Response = ICPSwapSwapPoolResult<Nat>;
