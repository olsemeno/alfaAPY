pub mod queries;

use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;
use types::ResultLowercase;
pub use queries::*;

pub type ICPSwapSwapFactoryResult<T> = ResultLowercase<T, ICPSwapSwapFactoryError>;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum ICPSwapSwapFactoryError {
    CommonError,
    InternalError(String),
    UnsupportedToken(String),
    InsufficientFunds,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct ICPSwapToken {
    pub address: String,
    pub standard: String,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct ICPSwapPool {
    pub fee: Nat,
    pub key: String,
    pub tickSpacing: i128,
    pub token0: ICPSwapToken,
    pub token1: ICPSwapToken,
    pub canisterId: candid::Principal,
}
