use candid::Nat;

use liquidity::liquidity_router::get_liquidity_client;
use types::liquidity::{AddLiquidityResponse, WithdrawLiquidityResponse};
use errors::internal_error::error::InternalError;
use liquidity::liquidity_client::LiquidityClient;
use types::context::Context;

use crate::pools::pool::Pool;
use crate::event_records::event_record_service;
use crate::event_records::event_record::Event;
use crate::utils::provider_impls::get_environment_provider_impls;


pub async fn add_liquidity_to_pool(
    context: Context,
    pool: Pool,
    amount: Nat
) -> Result<AddLiquidityResponse, InternalError> {
    let user = context.user.clone().unwrap();

    // Event: Add liquidity to pool started
    event_record_service::create_event_record(
        Event::add_liquidity_to_pool_started(pool.id.clone(), Some(amount.clone()), None),
        context.correlation_id.clone(),
        Some(user),
    );

    let liquidity_client = liquidity_client(pool.clone()).await;

    let add_liquidity_response = liquidity_client.add_liquidity_to_pool(
        amount.clone()
    ).await
        .map_err(|error| {
            // Event: Add liquidity to pool failed
            event_record_service::create_event_record(
                Event::add_liquidity_to_pool_failed(pool.id.clone(), Some(amount.clone()), error.clone()),
                context.correlation_id.clone(),
                context.user.clone(),
            );

            error
        })?;

    // Event: Add liquidity to pool completed
    event_record_service::create_event_record(
        Event::add_liquidity_to_pool_completed(pool.id, Some(amount), None),
        context.correlation_id.clone(),
        Some(user),
    );

    Ok(add_liquidity_response)
}

pub async fn withdraw_liquidity_from_pool(context: Context, pool: Pool) -> Result<WithdrawLiquidityResponse, InternalError> {
    let user = context.user.clone().unwrap();
    // Remove 100% liquidity from pool
    let total_shares = Nat::from(1 as u8);
    let shares = Nat::from(1 as u8);

    // Event: Withdraw liquidity from pool started
    event_record_service::create_event_record(
        Event::withdraw_liquidity_from_pool_started(pool.id.clone(), total_shares.clone(), shares.clone()),
        context.correlation_id.clone(),
        Some(user),
    );

    let liquidity_client = liquidity_client(pool.clone()).await;

    let withdraw_liquidity_response = liquidity_client.withdraw_liquidity_from_pool(
        total_shares.clone(),
        shares.clone()
    ).await
        .map_err(|error| {
            // Event: Withdraw liquidity from pool failed
            event_record_service::create_event_record(
                Event::withdraw_liquidity_from_pool_failed(
                    pool.id.clone(),
                    total_shares.clone(),
                    shares.clone(),
                    error.clone()
                ),
                context.correlation_id.clone(),
                Some(user),
            );

            error
        })?;

    // Event: Withdraw liquidity from pool completed
    event_record_service::create_event_record(
        Event::withdraw_liquidity_from_pool_completed(
            pool.id.clone(),
            total_shares,
            shares,
            withdraw_liquidity_response.token_0_amount.clone(),
            withdraw_liquidity_response.token_1_amount.clone()
        ),
        context.correlation_id.clone(),
        Some(user),
    );

    Ok(withdraw_liquidity_response)
}

async fn liquidity_client(pool: Pool) -> Box<dyn LiquidityClient> {
    get_liquidity_client(
        get_environment_provider_impls(),
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await
}
