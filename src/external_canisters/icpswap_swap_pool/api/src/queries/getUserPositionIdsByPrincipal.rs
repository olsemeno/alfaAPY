use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

pub type Args = (String,);

pub type Response = ICPSwapSwapPoolResult<Vec<Nat>, String>;
