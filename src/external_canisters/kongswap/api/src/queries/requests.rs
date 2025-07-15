use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use crate::add_liquidity::{TransferIdReply, TxId};

pub use crate::PoolsReply;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Args {
    pub request_id: Option<u64>,
}

pub type Response = Result<Vec<RequestReply>, String>;

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct RequestReply {
    pub request_id: u64,
    pub statuses: Vec<String>,
    pub request: Request,
    pub reply: Reply,
    pub ts: u64,
}
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    AddPool(AddPoolArgs),
    AddLiquidity(AddLiquidityArgs),
    RemoveLiquidity(RemoveLiquidityArgs),
    Swap(SwapArgs),
    Claim(u64),
    Send(SendArgs),
}
#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddPoolArgs {
    pub token_0: String,
    pub amount_0: Nat,
    pub tx_id_0: Option<TxId>,
    pub token_1: String,
    pub amount_1: Nat,
    pub tx_id_1: Option<TxId>,
    pub lp_fee_bps: Option<u8>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct AddLiquidityArgs {
    pub token_0: String,
    pub amount_0: Nat,
    pub tx_id_0: Option<TxId>,
    pub token_1: String,
    pub amount_1: Nat,
    pub tx_id_1: Option<TxId>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct RemoveLiquidityArgs {
    pub token_0: String,
    pub token_1: String,
    pub remove_lp_token_amount: Nat,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct SwapArgs {
    pub pay_token: String,
    pub pay_amount: Nat,
    pub pay_tx_id: Option<TxId>,
    pub receive_token: String,
    pub receive_amount: Option<Nat>,
    pub receive_address: Option<String>,
    pub max_slippage: Option<f64>,
    pub referred_by: Option<String>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct SendArgs {
    pub token: String,
    pub amount: Nat,
    pub to_address: String,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum Reply {
    Pending,
    AddPool(AddPoolReply),
    AddLiquidity(AddLiquidityReply),
    RemoveLiquidity(RemoveLiquidityReply),
    Swap(SwapReply),
    Claim(ClaimReply),
    Send(SendReply),
}


#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct AddPoolReply {
    pub tx_id: u64,
    #[serde(default = "zero_u32")]
    pub pool_id: u32,
    pub request_id: u64,
    pub status: String,
    #[serde(default = "empty_string")]
    pub name: String,
    pub symbol: String,
    pub chain_0: String,
    #[serde(default = "empty_string")]
    pub address_0: String,
    pub symbol_0: String,
    pub amount_0: Nat,
    pub balance_0: Nat,
    pub chain_1: String,
    #[serde(default = "empty_string")]
    pub address_1: String,
    pub symbol_1: String,
    pub amount_1: Nat,
    pub balance_1: Nat,
    pub lp_fee_bps: u8,
    pub lp_token_symbol: String,
    pub add_lp_token_amount: Nat,
    pub transfer_ids: Vec<TransferIdReply>,
    pub claim_ids: Vec<u64>,
    #[serde(default = "false_bool")]
    pub is_removed: bool,
    pub ts: u64,
}

fn zero_u32() -> u32 {
    0
}

fn empty_string() -> String {
    String::new()
}

fn false_bool() -> bool {
    false
}


#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
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


#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct RemoveLiquidityReply {
    pub tx_id: u64,
    pub request_id: u64,
    pub status: String,
    pub symbol: String,
    pub chain_0: String,
    #[serde(default = "empty_string")]
    pub address_0: String,
    pub symbol_0: String,
    pub amount_0: Nat,
    pub lp_fee_0: Nat,
    pub chain_1: String,
    #[serde(default = "empty_string")]
    pub address_1: String,
    pub symbol_1: String,
    pub amount_1: Nat,
    pub lp_fee_1: Nat,
    pub remove_lp_token_amount: Nat,
    pub transfer_ids: Vec<TransferIdReply>,
    pub claim_ids: Vec<u64>,
    pub ts: u64,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct SwapReply {
    pub tx_id: u64,
    pub request_id: u64,
    pub status: String,
    pub pay_chain: String,
    #[serde(default = "empty_string")]
    pub pay_address: String,
    pub pay_symbol: String,
    pub pay_amount: Nat,
    pub receive_chain: String,
    #[serde(default = "empty_string")]
    pub receive_address: String,
    pub receive_symbol: String,
    pub receive_amount: Nat,
    pub mid_price: f64,
    pub price: f64,
    pub slippage: f64,
    pub txs: Vec<SwapTxReply>,
    pub transfer_ids: Vec<TransferIdReply>,
    pub claim_ids: Vec<u64>,
    pub ts: u64,
}


#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct SwapTxReply {
    pub pool_symbol: String,
    pub pay_chain: String,
    #[serde(default = "empty_string")]
    pub pay_address: String,
    pub pay_symbol: String,
    pub pay_amount: Nat,
    pub receive_chain: String,
    #[serde(default = "empty_string")]
    pub receive_address: String,
    pub receive_symbol: String,
    pub receive_amount: Nat, // including fees
    pub price: f64,
    pub lp_fee: Nat,  // will be in receive_symbol
    pub gas_fee: Nat, // will be in receive_symbol
    pub ts: u64,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct ClaimReply {
    pub claim_id: u64,
    pub status: String,
    pub chain: String,
    pub symbol: String,
    pub amount: Nat,
    pub fee: Nat,
    pub to_address: String,
    pub desc: String,
    pub transfer_ids: Vec<TransferIdReply>,
    pub ts: u64,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct SendReply {
    pub tx_id: u64,
    pub request_id: u64,
    pub status: String,
    pub chain: String,
    pub symbol: String,
    pub amount: Nat,
    pub to_address: String,
    pub ts: u64,
}






