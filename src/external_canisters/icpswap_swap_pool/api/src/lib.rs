pub mod queries;
pub mod updates;

use candid::{CandidType, Deserialize, Nat};
use serde::{Serialize, Deserialize as SerdeDeserialize};
use types::ResultLowercase;
pub use updates::*;
pub use queries::*;

pub type ICPSwapSwapPoolResult<T> = ResultLowercase<T, ICPSwapSwapPoolError>;

#[derive(CandidType, Debug, Clone, Serialize, SerdeDeserialize)]
pub enum ICPSwapSwapPoolError {
    CommonError,
    InternalError(String),
    UnsupportedToken(String),
    InsufficientFunds,
}

#[derive(CandidType, Debug, Clone, Serialize, SerdeDeserialize)]
pub struct Token {
    pub address: String,
    pub standard: String,
}
