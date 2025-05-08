use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

pub type Args = (String,);

pub type Response = Result<Vec<Nat>, String>;
