use candid::Nat;

use liquidity::liquidity_router::get_liquidity_client;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use errors::internal_error::error::InternalError;
use liquidity::liquidity_client::LiquidityClient;
use types::context::Context;

use crate::event_logs::event_log_params_builder::EventLogParamsBuilder;
use crate::pools::pool::Pool;
use crate::event_logs::event_log_service;

pub async fn add_liquidity_to_pool(
    context: Context,
    pool: Pool,
    amount: Nat
) -> Result<AddLiquidityResponse, InternalError> {
    let liquidity_client = liquidity_client(pool.clone()).await;

    liquidity_client.add_liquidity_to_pool(
        amount.clone()
    ).await
        .map_err(|error| {
            event_log_service::create_event_log(
                EventLogParamsBuilder::add_liquidity_to_pool_failed()
                    .pool_id(pool.id.clone())
                    .amount0(amount.clone())
                    .build(),
                context.correlation_id.clone(),
                context.user.clone(),
                Some(error.clone()),
            );

            error
        })
}

pub async fn remove_liquidity_from_pool(context: Context, pool: Pool) -> Result<WithdrawFromPoolResponse, InternalError> {
    let liquidity_client = liquidity_client(pool.clone()).await;

    // Remove 100% liquidity from pool
    let total_shares = Nat::from(1 as u8);
    let shares = Nat::from(1 as u8);

    let response = liquidity_client.withdraw_liquidity_from_pool(
        total_shares,
        shares
    ).await
        .map_err(|error| {
            event_log_service::create_event_log(
                EventLogParamsBuilder::remove_liquidity_from_pool_failed()
                    .pool_id(pool.id.clone())
                    .build(),
                context.correlation_id,
                context.user,
                Some(error.clone()),
            );

            error
        })?;

    Ok(response)
}

async fn liquidity_client(pool: Pool) -> Box<dyn LiquidityClient> {
    get_liquidity_client(
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await
}
