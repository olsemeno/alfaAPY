use candid::{CandidType, Int, Nat};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct TokenMeta {
    pub token0: Vec<TokenMetadataRecord>,
    pub token1: Vec<TokenMetadataRecord>,
    pub token0Fee: Option<Nat>,
    pub token1Fee: Option<Nat>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadataRecord(
    #[serde(rename = "0")] pub String,
    #[serde(rename = "1")] pub TokenMetadataValue,
);

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum TokenMetadataValue {
    #[serde(rename = "Int")]
    Int(i128),
    #[serde(rename = "Nat")]
    Nat(Nat),
    #[serde(rename = "Blob")]
    Blob(Vec<u8>),
    #[serde(rename = "Text")]
    Text(String),
}

pub type Response = ICPSwapSwapPoolResult<TokenMeta>;
