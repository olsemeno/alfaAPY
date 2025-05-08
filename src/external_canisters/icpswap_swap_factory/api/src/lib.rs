pub mod queries;
pub mod updates;

use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;
pub use updates::*;
pub use queries::*;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub address: String,
    pub standard: String,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    pub fee: Nat,
    pub key: String,
    pub tickSpacing: i32,
    pub token0: Token,
    pub token1: Token,
    pub canisterId: String,
}
