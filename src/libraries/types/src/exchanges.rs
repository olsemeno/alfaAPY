#![allow(deprecated)]
use crate::CanisterId;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TokenInfo {
    pub symbol: String,
    pub ledger: CanisterId,
}

