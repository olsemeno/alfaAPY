use candid::Nat;

use liquidity::liquidity_router::get_liquidity_client;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use errors::internal_error::error::InternalError;
use errors::internal_error::builder::InternalErrorBuilder;
use liquidity::liquidity_client::LiquidityClient;
use types::context::Context;

use crate::event_logs::event_log_params_builder::EventLogParamsBuilder;
use crate::repository::pools_repo;
use crate::pool_snapshots::pool_snapshot_service;
use crate::pools::pool::Pool;
use crate::event_logs::event_log_service;

pub async fn add_liquidity_to_pool(context: Context, pool_id: String, amount: Nat) -> Result<AddLiquidityResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());
    if let Some(pool) = pool {
        let liquidity_client = liquidity_client(pool.clone()).await;

        match liquidity_client.add_liquidity_to_pool(context.clone(), amount.clone()).await {
            Ok(response) => {
                pool_snapshot_service::create_pool_snapshot(context, &pool).await;
                Ok(response)
            }
            Err(error) => {
                // ========== Event log begin ==========
                let event_log_params = EventLogParamsBuilder::add_liquidity_to_pool_failed()
                    .pool_id(pool_id)
                    .amount0(amount)
                    .build();

                let internal_error = InternalErrorBuilder::business_logic()
                    .context("Liquidity service: add_liquidity_to_pool")
                    .message(format!("Error adding liquidity to pool: {}", error))
                    .build();

                event_log_service::create_event_log(
                    event_log_params,
                    context.correlation_id,
                    context.user,
                    Some(internal_error.clone()),
                );
                // ========== Event log end ==========
                Err(internal_error)
            }
        }
    } else {
        // ========== Event log begin ==========
        let event_log_params = EventLogParamsBuilder::add_liquidity_to_pool_failed()
            .pool_id(pool_id.clone())
            .amount0(amount)
            .build();

        let internal_error = InternalErrorBuilder::not_found()
            .context("Liquidity service: add_liquidity_to_pool")
            .message(format!("Pool not found: {}", pool_id))
            .build();

        event_log_service::create_event_log(
            event_log_params,
            context.correlation_id,
            context.user,
            Some(internal_error.clone()),
        );
        // ========== Event log end ==========

        Err(internal_error)
    }
}

pub async fn remove_liquidity_from_pool(context: Context, pool_id: String) -> Result<WithdrawFromPoolResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());
    if let Some(pool) = pool {
        let liquidity_client = liquidity_client(pool.clone()).await;

        // Remove all liquidity from pool
        let total_shares = Nat::from(1 as u8);
        let shares = Nat::from(1 as u8);

        match liquidity_client.withdraw_liquidity_from_pool(context.clone(), total_shares, shares).await {
            Ok(response) => Ok(response),
            Err(error) => {
                // ========== Event log begin ==========
                let event_log_params = EventLogParamsBuilder::remove_liquidity_from_pool_failed()
                    .pool_id(pool_id)
                    .build();

                let internal_error = InternalErrorBuilder::business_logic()
                    .context("Liquidity service: remove_liquidity_from_pool")
                    .message(format!("Error withdrawing from pool: {}", error))
                    .build();

                event_log_service::create_event_log(
                    event_log_params,
                    context.correlation_id,
                    context.user,
                    Some(internal_error.clone()),
                );
                // ========== Event log end ==========

                Err(internal_error)
            }
        }
    } else {
        // ========== Event log begin ==========
        let event_log_params = EventLogParamsBuilder::remove_liquidity_from_pool_failed()
            .pool_id(pool_id.clone())
            .build();

        let internal_error = InternalErrorBuilder::not_found()
            .context("Liquidity service: remove_liquidity_from_pool")
            .message(format!("Pool not found: {}", pool_id))
            .build();

        event_log_service::create_event_log(
            event_log_params,
            context.correlation_id,
            context.user,
            Some(internal_error.clone()),
        );
        // ========== Event log end ==========

        Err(internal_error)
    }
}

async fn liquidity_client(pool: Pool) -> Box<dyn LiquidityClient> {
    get_liquidity_client(
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await
}
