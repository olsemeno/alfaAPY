#![allow(deprecated)]
use crate::CanisterId;
use candid::CandidType;
use serde::{Deserialize, Serialize};


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TokenInfo {
    pub symbol: String,
    pub ledger: CanisterId,
}

