use candid::Nat;

use types::context::Context;
use types::liquidity::{AddLiquidityResponse, WithdrawLiquidityResponse};
use liquidity::liquidity_router::get_liquidity_client;
use errors::internal_error::error::InternalError;
use swap::swap_service;

use crate::pools::pool_data::PoolData;
use crate::pools::pool::Pool;
use crate::pool_stats::pool_stats_service;

pub async fn get_pools_data(pools: Vec<Pool>) -> Vec<PoolData> {
    let pool_ids: Vec<String> = pools.iter().map(|pool| pool.id.clone()).collect();
    let pool_metrics = pool_stats_service::get_pool_metrics(pool_ids).await;

    let pool_data: Vec<PoolData> = pools
        .into_iter()
        .zip(pool_metrics.into_iter())
        .map(|(pool, pool_metric)|
            PoolData {
                pool: pool.clone(),
                apy: pool_metric.1.apy.tokens_apy,
            }
        )
        .collect();

    pool_data
}

pub async fn add_liquidity_to_pool(
    context: Context,
    amount: Nat,
    pool: Pool
) -> Result<AddLiquidityResponse, InternalError> {
    let liquidity_client = get_liquidity_client(
        pool.token0,
        pool.token1,
        pool.provider
    ).await;

    liquidity_client.add_liquidity_to_pool(amount.clone()).await
}

pub async fn withdraw_liquidity_from_pool(
    context: Context,
    total_shares: Nat,
    shares: Nat,
    pool: Pool
) -> Result<WithdrawLiquidityResponse, InternalError> {
    let liquidity_client = get_liquidity_client(
        pool.token0,
        pool.token1,
        pool.provider
    ).await;

    liquidity_client.withdraw_liquidity_from_pool(total_shares.clone(), shares.clone()).await
}

pub async fn withdraw_liquidity_from_pool_and_swap(
    context: Context,
    total_shares: Nat,
    shares: Nat,
    pool: Pool
) -> Result<Nat, InternalError> {
    let withdraw_response = withdraw_liquidity_from_pool(
        context.clone(),
        total_shares.clone(),
        shares.clone(),
        pool.clone(),
    ).await?;

    // Swap withdrawn token_1 to token_0 (base token)
    let swap_response = swap_service::swap_icrc2_optimal(
        pool.token1,
        pool.token0,
        withdraw_response.token_1_amount,
    ).await?;

    let amount_0_to_withdraw = withdraw_response.token_0_amount + swap_response.amount_out;

    Ok(amount_0_to_withdraw)
}