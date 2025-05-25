use candid::Nat;
use ic_cdk::trap;

use crate::liquidity::clients::kongswap::KongSwapLiquidityClient;
use crate::liquidity::clients::icpswap::ICPSwapLiquidityClient;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use crate::pools::pool_data::PoolData;
use crate::pools::pool::Pool;
use types::exchanges::{TokenInfo};
use crate::swap::swap_service::KONG_BE_CANISTER;
use crate::liquidity::liquidity_client::LiquidityClient;
use crate::liquidity::liquidity_router::get_liquidity_client;

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
    let liquidity_client = get_liquidity_client(&pool).await;

    match liquidity_client.add_liquidity_to_pool(amount).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

pub async fn withdraw_from_pool(total_shares: Nat, shares: Nat, pool: Pool) -> WithdrawFromPoolResponse {
    let liquidity_client = get_liquidity_client(&pool).await;

    match liquidity_client.withdraw_from_pool(total_shares, shares).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

// TODO: remove this test methods

pub async fn add_liquidity_to_pool_kong(amount: Nat, token0: TokenInfo, token1: TokenInfo) -> AddLiquidityResponse {
    let liquidity_client = kong_liquidity_client(token0, token1);

    match liquidity_client.add_liquidity_to_pool(amount).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

pub async fn withdraw_from_pool_kong(total_shares: Nat, shares: Nat, token0: TokenInfo, token1: TokenInfo) -> WithdrawFromPoolResponse {
    let liquidity_client = kong_liquidity_client(token0, token1);

    match liquidity_client.withdraw_from_pool(total_shares, shares).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

pub async fn add_liquidity_to_pool_icpswap(amount: Nat, token0: TokenInfo, token1: TokenInfo) -> AddLiquidityResponse {
    let liquidity_client = icpswap_liquidity_client(token0, token1).await;

    match liquidity_client.add_liquidity_to_pool(amount).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

pub async fn withdraw_from_pool_icpswap(total_shares: Nat, shares: Nat, token0: TokenInfo, token1: TokenInfo) -> WithdrawFromPoolResponse {
    let liquidity_client = icpswap_liquidity_client(token0, token1).await;

    match liquidity_client.withdraw_from_pool(total_shares, shares).await {
        Ok(response) => response,
        Err(error) => {
            trap(error.as_str());
        }
    }
}

async fn icpswap_liquidity_client(token0: TokenInfo, token1: TokenInfo) -> Box<dyn LiquidityClient> {
    Box::new(
        ICPSwapLiquidityClient::new(
            token0,
            token1,
        ).await
    )
}

fn kong_liquidity_client(token0: TokenInfo, token1: TokenInfo) -> Box<dyn LiquidityClient> {
    Box::new(
        KongSwapLiquidityClient::new(
            KONG_BE_CANISTER,
            token0,
            token1,
        )
    )
}

// End of test methods
