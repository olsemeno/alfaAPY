use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub fee: Nat,
	pub token0: String,
	pub token1: String, 
	pub amount0Desired: String, 
	pub amount1Desired: String, 
	pub tickUpper: i32,
	pub tickLower: i32,
}

pub type Response = ICPSwapSwapPoolResult<Nat>;
