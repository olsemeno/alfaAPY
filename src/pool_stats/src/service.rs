use std::collections::HashMap;
use candid::Nat;
use ic_cdk::caller;

use types::exchange_id::ExchangeId;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use types::context::Context;
use types::CanisterId;
use types::pool::PoolTrait;
use errors::internal_error::error::InternalError;
use icrc_ledger_client;
use errors::internal_error::error::build_error_code;

use crate::pool_snapshots::pool_snapshot_service;
use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::pools::pool::Pool;
use crate::pool_metrics::pool_metrics::PoolMetrics;
use crate::pool_metrics::pool_metrics_service;
use crate::repository::pools_repo;
use crate::liquidity::liquidity_service;
use crate::event_logs::event_log_service;
use crate::event_logs::event_log_params_builder::EventLogParamsBuilder;

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
            build_error_code(4000, 1, 1), // 4000 01 01
            "service::delete_pool".to_string(),
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
            build_error_code(4000, 1, 2), // 4000 01 02
            "service::get_pool_by_id".to_string(),
            "Pool not found".to_string(),
            Some(HashMap::from([
                ("id".to_string(), id)
            ]))
        ))
}

// ========================== Pool metrics ==========================

pub fn get_pool_metrics(pool_ids: Vec<String>) -> HashMap<String, PoolMetrics> {
    pool_ids.into_iter()
        .filter_map(|pool_id| {
            pools_repo::get_pool_by_id(pool_id.clone())
                .map(|pool| (pool_id, pool_metrics_service::create_pool_metrics(pool)))
        })
        .collect()
}

pub fn get_pools_snapshots(pool_ids: Vec<String>) -> HashMap<String, Vec<PoolSnapshot>> {
    pool_ids.into_iter()
        .filter_map(|pool_id| {
            pools_repo::get_pool_by_id(pool_id.clone())
                .map(|pool| (pool_id, pools_repo::get_pool_snapshots(pool.id).unwrap_or_default()))
        })
        .collect()
}

// ========================== Liquidity management ==========================

pub async fn add_liquidity_to_pool(
    context: Context,
    ledger: CanisterId,
    pool_id: String,
    amount: Nat
) -> Result<AddLiquidityResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());

    if pool.is_none() {
        let error = InternalError::not_found(
            build_error_code(4000, 1, 3), // 4000 01 03
            "service::add_liquidity_to_pool".to_string(),
            "Pool not found".to_string(),
            Some(HashMap::from([
                ("pool_id".to_string(), pool_id.clone()),
            ])),
        );

        event_log_service::create_event_log(
            EventLogParamsBuilder::add_liquidity_to_pool_failed()
                .pool_id(pool_id.clone())
                .amount0(amount)
                .build(),
            context.correlation_id,
            context.user,
            Some(error.clone()),
        );

        return Err(error);
    }

    let mut pool = pool.unwrap();

    if pool.position_id.is_some() {
        let error = InternalError::business_logic(
            build_error_code(4000, 3, 4), // 4000 01 04
            "service::add_liquidity_to_pool".to_string(),
            "Pool already has liquidity".to_string(),
            Some(HashMap::from([
                ("pool_id".to_string(), pool_id.clone()),
            ])),
        );

        return Err(error);
    }

    icrc_ledger_client::icrc2_transfer_from(caller(), ledger, amount.clone()).await?;

    let response = liquidity_service::add_liquidity_to_pool(
        context.clone(),
        pool.clone(),
        amount
    ).await?;

    pool.position_id = Some(response.request_id);
    pools_repo::update_pool(pool_id.clone(), pool.clone());

    pool_snapshot_service::create_pool_snapshot(context, &pool).await?;

    Ok(response)
}

pub async fn remove_liquidity_from_pool(
    context: Context,
    pool_id: String
) -> Result<WithdrawFromPoolResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());

    if pool.is_none() {
        let error = InternalError::not_found(
            build_error_code(4000, 1, 5), // 4000 01 05
            "service::remove_liquidity_from_pool".to_string(),
            "Pool not found".to_string(),
            Some(HashMap::from([
                ("pool_id".to_string(), pool_id.clone()),
            ])),
        );

        event_log_service::create_event_log(
            EventLogParamsBuilder::remove_liquidity_from_pool_failed()
                .pool_id(pool_id)
                .build(),
            context.correlation_id,
            context.user,
            Some(error.clone()),
        );

        return Err(error);
    }

    let mut pool = pool.unwrap();

    if pool.position_id.is_none() {
        let error = InternalError::business_logic(
            build_error_code(4000, 3, 6), // 4000 01 06
            "service::remove_liquidity_from_pool".to_string(),
            "Pool has no liquidity".to_string(),
            Some(HashMap::from([
                ("pool_id".to_string(), pool_id.clone()),
            ])),
        );

        return Err(error);
    }

    let response = liquidity_service::remove_liquidity_from_pool(
        context,
        pool.clone()
    ).await?;

    pool.position_id = None;
    pools_repo::update_pool(pool_id.clone(), pool.clone());

    Ok(response)
}
