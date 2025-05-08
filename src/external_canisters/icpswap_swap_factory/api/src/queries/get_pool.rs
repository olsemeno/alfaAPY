use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

pub use crate::Token;
pub use crate::Pool;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub fee: Nat,
    pub token0: Token,
    pub token1: Token,
}

pub type Response = Result<Pool, String>;
