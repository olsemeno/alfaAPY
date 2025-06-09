use candid::Nat;

use liquidity::liquidity_router::get_liquidity_client;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};

use crate::repository::pools_repo;
use crate::pools::pool::Position;

pub async fn add_liquidity_to_pool(pool_id: String, amount: Nat) -> Result<AddLiquidityResponse, String> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());
    if let Some(mut pool) = pool {
        let liquidity_client = get_liquidity_client(
            pool.token0.clone(), 
            pool.token1.clone(), 
            pool.provider.clone()
        ).await;

        match liquidity_client.add_liquidity_to_pool(amount).await {
            Ok(response) => {
                // Update pool position
                pool.initial_position = Some(Position {
                    id: Nat::from(response.request_id as u64),
                    initial_amount0: response.token_0_amount.clone(),
                    initial_amount1: response.token_1_amount.clone(),
                });
                pools_repo::update_pool(pool.id.clone(), pool.clone());
                Ok(response)
            }
            Err(error) => {
                Err(format!("Error adding liquidity to pool: {}", error))
            }
        }
    } else {
        Err(format!("Pool not found: {}", pool_id))
    }
}

pub async fn remove_liquidity_from_pool(pool_id: String) -> Result<WithdrawFromPoolResponse, String> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());
    if let Some(mut pool) = pool {
        let liquidity_client = get_liquidity_client(
            pool.token0.clone(), 
            pool.token1.clone(), 
            pool.provider.clone()
        ).await;

        // Remove all liquidity from pool
        let total_shares = Nat::from(1 as u8);
        let shares = Nat::from(1 as u8);

        match liquidity_client.withdraw_liquidity_from_pool(total_shares, shares).await {
            Ok(response) => {
                // Update pool position
                pool.initial_position = None;
                pools_repo::update_pool(pool.id.clone(), pool.clone());
                Ok(response)
            }
            Err(error) => {
                Err(format!("Error withdrawing from pool: {}", error))
            }
        }
    } else {
        Err(format!("Pool not found: {}", pool_id))
    }
}
