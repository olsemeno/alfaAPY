use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;
pub use crate::PoolsReply;

pub type Response = Result<SwapAmountsReply, String>;

#[derive(CandidType, Serialize, Deserialize)]
pub struct Args {
    pub pay_token: String,
    pub pay_amount: Nat,
    pub receive_token: String
}


#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct SwapAmountsTxReply {
    pub pool_symbol: String,
    pub pay_chain: String,
    pub pay_symbol: String,
    pub pay_amount: Nat,
    pub pay_address: String,
    pub receive_chain: String,
    pub receive_symbol: String,
    pub receive_address: String,
    pub receive_amount: Nat,
    pub price: f64,
    pub lp_fee: Nat,
    pub gas_fee: Nat,
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct SwapAmountsReply {
    pub pay_chain: String,
    pub pay_symbol: String,
    pub pay_address: String,
    pub pay_amount: Nat,
    pub receive_chain: String,
    pub receive_symbol: String,
    pub receive_address: String,
    pub receive_amount: Nat,
    pub price: f64,
    pub mid_price: f64,
    pub slippage: f64,
    pub txs: Vec<SwapAmountsTxReply>,
}


