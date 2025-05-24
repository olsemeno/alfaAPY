use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

pub type Args = (Nat, Nat, Nat);
pub type Response = (f64,);
