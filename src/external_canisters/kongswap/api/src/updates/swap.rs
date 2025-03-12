use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};
use crate::add_liquidity::TransferIdReply;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub pay_token: String,
    pub pay_amount: Nat,
    // pub pay_tx_id: Option<Nat>,
    pub receive_token: String,
    // pub receive_amount: Option<Nat>,
    // pub receive_address: Option<String>,
    pub max_slippage: Option<f64>,
    // pub referred_by: Option<String>,
}

pub type Response = Result<SwapReply, String>;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SwapReply {
    pub tx_id: u64,
    pub request_id: u64,
    pub status: String,
    pub pay_chain: String,
    pub pay_symbol: String,
    pub pay_amount: Nat,
    pub receive_chain: String,
    pub receive_symbol: String,
    pub receive_amount: Nat,
    pub mid_price: f64,
    pub price: f64,
    pub slippage: f64,
    // pub txs: Vec<SwapTxReply>,
    pub transfer_ids: Vec<TransferIdReply>,
    pub claim_ids: Vec<u64>,
    pub ts: u64,
}


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ICTransferReply {
    pub chain: String,
    pub symbol: String,
    pub is_send: bool,
    pub amount: Nat,
    pub canister_id: String,
    pub block_index: Nat,
}
