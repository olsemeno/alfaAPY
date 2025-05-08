use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Args {
    pub amountIn: String,
    pub zeroForOne: bool,
    pub amountOutMinimum: String,
}

pub type Response = Result<Nat, String>;
