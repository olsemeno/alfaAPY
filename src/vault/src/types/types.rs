use std::collections::HashMap;
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;

use types::CanisterId;
use errors::response_error::error::ResponseError;

use crate::pools::pool::Pool;
use crate::event_records::event_record::EventRecord;

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct StrategyDepositArgs {
    pub ledger: CanisterId,
    pub amount: Nat,
    pub strategy_id: StrategyId,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct StrategyWithdrawArgs {
    pub ledger: CanisterId,
    pub percentage: Nat,
    pub strategy_id: StrategyId,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyDepositResponse {
    pub amount: Nat,
    pub shares: Nat,
    pub tx_id: u64,
    pub position_id: u64,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyWithdrawResponse {
    pub amount: Nat,
    pub current_shares: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyRebalanceResponse {
    pub previous_pool: Pool,
    pub current_pool: Pool,
    pub is_rebalanced: bool,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct StrategyResponse {
    pub name: String,
    pub id: StrategyId,
    pub description: String,
    pub pools: Vec<Pool>,
    pub current_pool: Option<Pool>,
    pub total_balance: Nat,
    pub total_shares: Nat,
    pub user_shares: HashMap<Principal, Nat>,
    pub initial_deposit: HashMap<Principal, Nat>,
    pub users_count: u32,
    pub current_liquidity: Option<Nat>,
    pub current_liquidity_updated_at: Option<u64>,
}

// TODO: rename to UserPositionResponse
#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct UserStrategyResponse {
    pub strategy_id: StrategyId,
    pub strategy_name: String,
    pub strategy_current_pool: Pool,
    pub total_shares: Nat,
    pub user_shares: Nat,
    pub initial_deposit: Nat,
    pub users_count: u32,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Icrc28TrustedOriginsResponse {
    pub trusted_origins: Vec<String>,
}

#[derive(CandidType, Deserialize, Eq, PartialEq, Debug)]
pub struct SupportedStandard {
    pub url: String,
    pub name: String,
}

pub type StrategyId = u16;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyDepositResult(pub Result<StrategyDepositResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct StrategyWithdrawResult(pub Result<StrategyWithdrawResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetEventRecordsResult(pub Result<EventRecordsPaginationResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EventRecordsPaginationResponse(pub ListItemsPaginationResponse<EventRecord>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ListItemsPaginationRequest {
    pub page: u64,
    pub page_size: u64,
    pub sort_order: SortOrder,
    pub search: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ListItemsPaginationResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
