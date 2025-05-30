use candid::{CandidType, Nat, Int};
use serde::{Deserialize, Serialize};

pub type Response = Vec<TokenData>;

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct TokenData {
    pub id: Nat,
    pub volumeUSD1d: f64,
    pub volumeUSD7d: f64,
    pub totalVolumeUSD: f64,
    pub name: String,
    pub volumeUSD: f64,
    pub feesUSD: f64,
    pub priceUSDChange: f64,
    pub address: String,
    pub txCount: Int,
    pub priceUSD: f64,
    pub standard: String,
    pub symbol: String,
}
