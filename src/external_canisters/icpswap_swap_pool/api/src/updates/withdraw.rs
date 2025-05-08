use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub token : String,
    pub fee : Nat,
    pub amount : Nat,
}

pub type Response = Result<Nat, String>;
