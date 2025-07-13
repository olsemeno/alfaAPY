use std::collections::HashMap;
use candid::{CandidType, Deserialize};
use serde::Serialize;

use ::types::liquidity::{AddLiquidityResponse, WithdrawLiquidityResponse};
use errors::response_error::error::ResponseError;

use crate::pools::pool::Pool;
use crate::pool_metrics::pool_metrics::PoolMetrics;
use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::event_records::event_record::EventRecord;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WithdrawLiquidityResult(pub Result<WithdrawLiquidityResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AddLiquidityResult(pub Result<AddLiquidityResponse, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct AddPoolResult(pub Result<String, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct DeletePoolResult(pub Result<(), ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetPoolsResult(pub Result<Vec<Pool>, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetPoolByIdResult(pub Result<Pool, ResponseError>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetPoolMetricsResult(pub HashMap<String, PoolMetrics>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetPoolsSnapshotsResult(pub HashMap<String, Vec<PoolSnapshot>>);

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GetEventRecordsResult(pub Result<Vec<EventRecord>, ResponseError>);

