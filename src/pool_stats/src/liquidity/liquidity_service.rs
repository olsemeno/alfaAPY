use candid::Nat;
use std::collections::HashMap;

use liquidity::liquidity_router::get_liquidity_client;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use errors::internal_error::error::InternalError;
use liquidity::liquidity_client::LiquidityClient;
use types::context::Context;

use crate::event_logs::event_log_params_builder::EventLogParamsBuilder;
use crate::repository::pools_repo;
use crate::pool_snapshots::pool_snapshot_service;
use crate::pools::pool::Pool;
use crate::event_logs::event_log_service;
use errors::internal_error::error::build_error_code;

pub async fn add_liquidity_to_pool(context: Context, pool_id: String, amount: Nat) -> Result<AddLiquidityResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());
    if let Some(mut pool) = pool {
        let liquidity_client = liquidity_client(pool.clone()).await;

        let response = liquidity_client.add_liquidity_to_pool(
            amount.clone()
        ).await
            .map_err(|error| {
                event_log_service::create_event_log(
                    EventLogParamsBuilder::add_liquidity_to_pool_failed()
                        .pool_id(pool_id.clone())
                        .amount0(amount.clone())
                        .build(),
                    context.correlation_id.clone(),
                    context.user.clone(),
                    Some(error.clone()),
                );

                error
            })?;

        // TODO: move to service

        // Update pool with liquidity position id
        pool.position_id = Some(response.request_id);
        pools_repo::update_pool(pool_id.clone(), pool.clone());

        // Create initial snapshot for pool
        pool_snapshot_service::create_pool_snapshot(context, &pool).await?;

        Ok(response)
    } else {
        let error = InternalError::not_found(
            build_error_code(4100, 1, 1), // 4100 01 01
            "liquidity_service::add_liquidity_to_pool".to_string(),
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

        Err(error)
    }
}

pub async fn remove_liquidity_from_pool(context: Context, pool_id: String) -> Result<WithdrawFromPoolResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());
    if let Some(pool) = pool {
        let liquidity_client = liquidity_client(pool.clone()).await;

        // Remove all liquidity from pool
        let total_shares = Nat::from(1 as u8);
        let shares = Nat::from(1 as u8);

        let response = liquidity_client.withdraw_liquidity_from_pool(
            total_shares, 
            shares
        ).await
            .map_err(|error| {
                event_log_service::create_event_log(
                    EventLogParamsBuilder::remove_liquidity_from_pool_failed()
                        .pool_id(pool_id)
                        .build(),
                    context.correlation_id,
                    context.user,
                    Some(error.clone()),
                );

                error
            })?;

        Ok(response)
    } else {
        let error = InternalError::not_found(
            build_error_code(4100, 1, 2), // 4100 01 02
            "liquidity_service::remove_liquidity_from_pool".to_string(),
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

        Err(error)
    }
}

async fn liquidity_client(pool: Pool) -> Box<dyn LiquidityClient> {
    get_liquidity_client(
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await
}
