pub use crate::PoolsReply;
use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Args {
    pub token_0: String,
    pub token_1: String,
    pub remove_lp_token_amount: Nat,
}

pub type Response = Result<RemoveLiquidityAmountsReply, String>;


#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveLiquidityAmountsReply {
    pub symbol: String,
    pub chain_0: String,
    pub address_0: String,
    pub symbol_0: String,
    pub amount_0: Nat,
    pub lp_fee_0: Nat,
    pub chain_1: String,
    pub address_1: String,
    pub symbol_1: String,
    pub amount_1: Nat,
    pub lp_fee_1: Nat,
    pub remove_lp_token_amount: Nat,
}