use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub principal: String,
}

pub type Response = ICPSwapSwapPoolResult<UserUnusedBalance>;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct UserUnusedBalance {
    pub balance0: Nat,
    pub balance1: Nat,
}
