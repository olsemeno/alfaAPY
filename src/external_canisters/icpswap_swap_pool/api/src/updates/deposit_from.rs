use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub token : String,
    pub amount : Nat,
    pub fee : Nat,  
}

pub type Response = Result<Nat, String>;
