use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

pub use crate::Pool;

pub type Response = Result<Vec<Pool>, String>;
