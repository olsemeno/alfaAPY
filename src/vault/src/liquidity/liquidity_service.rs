use candid::Nat;
use ic_cdk::trap;

use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use liquidity::liquidity_router::get_liquidity_client;

use crate::pools::pool_data::PoolData;
use crate::pools::pool::Pool;
use crate::pool_stats::pool_stats_service;

pub async fn get_pools_data(pools: Vec<Pool>) -> Vec<PoolData> {
    let pool_metrics = pool_stats_service::get_pool_metrics(pools.clone()).await;

    let pool_data: Vec<PoolData> = pools
        .into_iter()
        .zip(pool_metrics.into_iter())
        .map(|(pool, pool_metric)| PoolData {
            pool: pool.clone(),
            apy: pool_metric.1.apy.month.tokens_apy,
        })
        .collect();

    pool_data
}

pub async fn add_liquidity_to_pool(amount: Nat, pool: Pool) -> AddLiquidityResponse {
    let liquidity_client = get_liquidity_client(
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await;

    match liquidity_client.add_liquidity_to_pool(amount).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

pub async fn withdraw_liquidity_from_pool(total_shares: Nat, shares: Nat, pool: Pool) -> WithdrawFromPoolResponse {
    let liquidity_client = get_liquidity_client(
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await;

    match liquidity_client.withdraw_liquidity_from_pool(total_shares, shares).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}
