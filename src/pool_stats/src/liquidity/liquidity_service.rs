use candid::Nat;

use liquidity::liquidity_router::get_liquidity_client;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};

use crate::repository::pools_repo;

pub async fn add_liquidity_to_pool(pool_id: &str, amount: Nat) -> Result<AddLiquidityResponse, String> {
    let pool = pools_repo::get_pool_by_id(pool_id);
    if let Some(pool) = pool {
        let liquidity_client = get_liquidity_client(
            pool.token0.clone(), 
            pool.token1.clone(), 
            pool.provider.clone()
        ).await;

        match liquidity_client.add_liquidity_to_pool(amount).await {
            Ok(response) => Ok(response),
            Err(error) => {
                Err(format!("Error adding liquidity to pool: {}", error))
            }
        }
    } else {
        Err(format!("Pool not found: {}", pool_id))
    }
}

pub async fn remove_liquidity_from_pool(pool_id: &str) -> Result<WithdrawFromPoolResponse, String> {
    let pool = pools_repo::get_pool_by_id(pool_id);
    if let Some(pool) = pool {
        let liquidity_client = get_liquidity_client(
            pool.token0.clone(), 
            pool.token1.clone(), 
            pool.provider.clone()
        ).await;

        // Remove all liquidity from pool
        let total_shares = Nat::from(1 as u8);
        let shares = Nat::from(1 as u8);

        match liquidity_client.withdraw_liquidity_from_pool(total_shares, shares).await {
            Ok(response) => Ok(response),
            Err(error) => {
                Err(format!("Error withdrawing from pool: {}", error))
            }
        }
    } else {
        Err(format!("Pool not found: {}", pool_id))
    }
}
