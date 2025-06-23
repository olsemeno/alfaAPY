use candid::Nat;

use liquidity::liquidity_router::get_liquidity_client;
use types::liquidity::{AddLiquidityResponse, WithdrawLiquidityResponse};
use errors::internal_error::error::InternalError;
use liquidity::liquidity_client::LiquidityClient;
use types::context::Context;

use crate::pools::pool::Pool;
use crate::event_records::event_record_service;
use crate::event_records::event_record::Event;

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
            event_record_service::create_event_record(
                Event::add_liquidity_to_pool_failed(Some(pool.id), Some(amount), None),
                context.correlation_id.clone(),
                context.user.clone(),
                Some(error.clone()),
            );

            error
        })
}

pub async fn withdraw_liquidity_from_pool(context: Context, pool: Pool) -> Result<WithdrawLiquidityResponse, InternalError> {
    let liquidity_client = liquidity_client(pool.clone()).await;

    // Remove 100% liquidity from pool
    let total_shares = Nat::from(1 as u8);
    let shares = Nat::from(1 as u8);

    let response = liquidity_client.withdraw_liquidity_from_pool(
        total_shares,
        shares
    ).await
        .map_err(|error| {
            event_record_service::create_event_record(
                Event::withdraw_liquidity_from_pool_failed(pool.id, None, None),
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
