use std::collections::HashMap;
use candid::Nat;
use ic_cdk::caller;

use types::exchange_id::ExchangeId;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use types::context::Context;
use types::CanisterId;
use types::pool::PoolTrait;
use errors::internal_error::error::InternalError;
use utils::token_transfer::icrc2_transfer_from_user;

use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::pools::pool::Pool;
use crate::pool_metrics::pool_metrics::PoolMetrics;
use crate::pool_metrics::pool_metrics_service;
use crate::repository::pools_repo;
use crate::liquidity::liquidity_service;

// ========================== Pools management ==========================


pub fn add_pool(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> Result<String, InternalError> {
    let pool = Pool::build(token0, token1, provider);
    pool.save();
    Ok(pool.id)
}

pub fn delete_pool(id: String) -> Result<(), InternalError> {
    pools_repo::get_pool_by_id(id.clone())
        .map(|pool| {
            pool.delete();
            Ok(())
        })
        .unwrap_or(Err(InternalError::not_found(
            "pool_stats::delete_pool".to_string(),
            "Pool not found".to_string(),
            Some(HashMap::from([
                ("id".to_string(), id.clone())
            ]))
        )))
}

pub fn get_pools() -> Result<Vec<Pool>, InternalError> {
    Ok(pools_repo::get_pools())
}

pub fn get_pool_by_id(id: String) -> Result<Pool, InternalError> {
    pools_repo::get_pool_by_id(id.clone())
        .ok_or_else(|| InternalError::not_found(
            "pool_stats::get_pool_by_id".to_string(),
            "Pool not found".to_string(),
            Some(HashMap::from([
                ("id".to_string(), id)
            ]))
        ))
}

// ========================== Pool metrics ==========================

pub fn get_pool_metrics(pool_ids: Vec<String>) -> Result<HashMap<String, PoolMetrics>, InternalError> {
    let pool_metrics = pool_ids.into_iter()
        .filter_map(|pool_id| {
            pools_repo::get_pool_by_id(pool_id.clone())
                .map(|pool| (pool_id, pool_metrics_service::create_pool_metrics(pool)))
        })
        .collect();

    Ok(pool_metrics)
}

pub fn get_pools_snapshots(pool_ids: Vec<String>) -> Result<HashMap<String, Vec<PoolSnapshot>>, InternalError> {
    let pool_snapshots = pool_ids.into_iter()
        .filter_map(|pool_id| {
            pools_repo::get_pool_by_id(pool_id.clone())
                .map(|pool| (pool_id, pools_repo::get_pool_snapshots(pool.id).unwrap_or_default()))
        })
        .collect();

    Ok(pool_snapshots)
}

// ========================== Liquidity management ==========================

pub async fn add_liquidity_to_pool(
    context: Context,
    ledger: CanisterId,
    pool_id: String,
    amount: Nat
) -> Result<AddLiquidityResponse, InternalError> {
    icrc2_transfer_from_user(caller(), ledger, amount.clone()).await?;
    liquidity_service::add_liquidity_to_pool(context, pool_id, amount).await
}

pub async fn remove_liquidity_from_pool(
    context: Context,
    pool_id: String
) -> Result<WithdrawFromPoolResponse, InternalError> {
    liquidity_service::remove_liquidity_from_pool(context, pool_id).await
}
