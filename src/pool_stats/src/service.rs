use candid::Nat;

use types::exchanges::TokenInfo;
use types::exchange_id::ExchangeId;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};

use crate::pools::pool::Pool;
use crate::pools::pool_metrics::PoolMetrics;
use crate::repository::pools_repo;
use crate::liquidity::liquidity_service;

pub fn add_pool(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) {
    Pool::new(
        pools_repo::get_pool_count().to_string(),
        token0,
        token1,
        provider,
    ).save();
}

pub fn delete_pool(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) {
    if let Some(pool) = pools_repo::get_pool_by_tokens(token0, token1, provider) {
        pool.delete();
    }
}

pub fn get_pools() -> Vec<Pool> {
    pools_repo::get_pools()
}

pub fn get_pool_by_tokens(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) -> Option<Pool> {
    pools_repo::get_pool_by_tokens(token0, token1, provider)
}

pub fn get_pool_metrics(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) -> Option<PoolMetrics> {
    let pool = pools_repo::get_pool_by_tokens(token0, token1, provider);

    if let Some(pool) = pool {
        Some(PoolMetrics::build(pool))
    } else {
        None
    }
}

pub async fn add_liquidity_to_pool(pool_id: &str, amount: Nat) -> Result<AddLiquidityResponse, String> {
    liquidity_service::add_liquidity_to_pool(pool_id, amount).await
}

pub async fn remove_liquidity_from_pool(pool_id: &str) -> Result<WithdrawFromPoolResponse, String> {
    liquidity_service::remove_liquidity_from_pool(pool_id).await
}
