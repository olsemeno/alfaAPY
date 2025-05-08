pub mod queries;
pub mod updates;

use candid::{CandidType, Deserialize};
use serde::Serialize;
pub use updates::*;
pub use queries::*;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub address: String,
    pub standard: String,
}
