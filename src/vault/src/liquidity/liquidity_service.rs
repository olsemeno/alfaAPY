use candid::Nat;
use ic_cdk::trap;

use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use types::exchanges::TokenInfo;
use types::exchange_id::ExchangeId;
use liquidity::liquidity_router::get_liquidity_client;

use crate::pools::pool_data::PoolData;
use crate::pools::pool::Pool;

pub async fn get_pools_data(required_pools: Vec<Pool>) -> Vec<PoolData> {
    // TODO: return APY for pools

    // match pools().await {
    //     Ok(response) => {
    //         let pools = response.pools;
    //         let mut pool_data = Vec::new();
    //         for pool in required_pools {
    //             match pools.iter().find(|&x| x.symbol == pool.pool_symbol)
    //             {
    //                 None => {}
    //                 Some(x) => {
    //                     pool_data.push(x.to_owned());
    //                 }
    //             }
    //         }
    //         pool_data
    //     }
    //     Err(error) => {
    //         trap(error.as_str());
    //     }
    // }

    vec![]
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


// TODO: remove this test methods below

pub async fn add_liquidity_to_pool_kong(amount: Nat, token0: TokenInfo, token1: TokenInfo) -> AddLiquidityResponse {
    let liquidity_client = get_liquidity_client(
        token0.clone(), 
        token1.clone(), 
        ExchangeId::KongSwap
    ).await;

    match liquidity_client.add_liquidity_to_pool(amount).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

pub async fn withdraw_from_pool_kong(total_shares: Nat, shares: Nat, token0: TokenInfo, token1: TokenInfo) -> WithdrawFromPoolResponse {
    let liquidity_client = get_liquidity_client(
        token0.clone(), 
        token1.clone(), 
        ExchangeId::KongSwap
    ).await;

    match liquidity_client.withdraw_liquidity_from_pool(total_shares, shares).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

pub async fn add_liquidity_to_pool_icpswap(amount: Nat, token0: TokenInfo, token1: TokenInfo) -> AddLiquidityResponse {
    let liquidity_client = get_liquidity_client(
        token0.clone(), 
        token1.clone(), 
        ExchangeId::ICPSwap
    ).await;

    match liquidity_client.add_liquidity_to_pool(amount).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

pub async fn withdraw_from_pool_icpswap(total_shares: Nat, shares: Nat, token0: TokenInfo, token1: TokenInfo) -> WithdrawFromPoolResponse {
    let liquidity_client = get_liquidity_client(
        token0.clone(), 
        token1.clone(), 
        ExchangeId::ICPSwap
    ).await;

    match liquidity_client.withdraw_liquidity_from_pool(total_shares, shares).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

// End of test methods
