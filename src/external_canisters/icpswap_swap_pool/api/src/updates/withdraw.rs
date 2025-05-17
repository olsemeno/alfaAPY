use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub token : String,
    pub amount : Nat,
    pub fee : Nat,
}

pub type Response = ICPSwapSwapPoolResult<Nat>;
