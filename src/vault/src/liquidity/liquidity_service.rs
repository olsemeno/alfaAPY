use candid::{Nat, Principal};
use ic_cdk::trap;
use kongswap_canister::PoolReply;

use crate::liquidity::clients::kongswap::KongSwapLiquidityClient;
use crate::liquidity::clients::icpswap::ICPSwapLiquidityClient;
use crate::providers::kong::kong::{add_liquidity, add_liquidity_amounts, pools, remove_liquidity, swap_amounts, user_balances};
use crate::types::types::{AddLiquidityResponse, Pool, TokensInfo, WithdrawFromPoolResponse};
use types::exchanges::TokenInfo;
use crate::swap::swap_service::KONG_BE_CANISTER;
use crate::liquidity::liquidity_client::LiquidityClient;

pub async fn get_pools_data(required_pools: Vec<Pool>) -> Vec<PoolReply> {
    match pools().await {
        Ok(response) => {
            let pools = response.pools;
            let mut pool_data = Vec::new();
            for pool in required_pools {
                match pools.iter().find(|&x| x.symbol == pool.pool_symbol)
                {
                    None => {}
                    Some(x) => {
                        pool_data.push(x.to_owned());
                    }
                }
            }
            pool_data
        }
        Err(error) => {
            trap(error.as_str());
        }
    }
}

pub async fn add_liquidity_to_pool(amount: Nat, token0: TokenInfo, token1: TokenInfo) -> AddLiquidityResponse {
    // return add_liquidity_to_pool_kong(amount, pool, token0, token1).await;
    return add_liquidity_to_pool_icpswap(amount, token0, token1).await;
}

pub async fn withdraw_from_pool(total_shares: Nat, shares: Nat, token0: TokenInfo, token1: TokenInfo) -> WithdrawFromPoolResponse {
    return withdraw_from_pool_kong(total_shares, shares, token0, token1).await;
    // return withdraw_from_pool_icpswap(total_shares, shares, token0, token1).await;
}

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
