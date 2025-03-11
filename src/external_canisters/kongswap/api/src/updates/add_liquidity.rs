use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};
use crate::swap::ICTransferReply;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Args {
    pub token_0: String,
    pub amount_0: Nat,
    pub tx_id_0: Option<TxId>,
    pub token_1: String,
    pub amount_1: Nat,
    pub tx_id_1: Option<TxId>,
}
#[derive(CandidType, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxId {
    BlockIndex(Nat),
    TransactionHash(String),
}

pub type Response = Result<AddLiquidityReply, String>;

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct AddLiquidityReply {
    pub tx_id: u64,
    pub request_id: u64,
    pub status: String,
    pub symbol: String,
    pub chain_0: String,
    pub address_0: String,
    pub symbol_0: String,
    pub amount_0: Nat,
    pub chain_1: String,
    pub address_1: String,
    pub symbol_1: String,
    pub amount_1: Nat,
    pub add_lp_token_amount: Nat,
    pub transfer_ids: Vec<TransferIdReply>,
    pub claim_ids: Vec<u64>,
    pub ts: u64,
}
#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct TransferIdReply {
    pub transfer_id: u64,
    pub transfer: TransferReply,
}



#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub enum TransferReply {
    IC(ICTransferReply),
}


