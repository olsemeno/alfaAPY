use candid::Nat;

use liquidity::liquidity_router::get_liquidity_client;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use errors::internal_error::error::InternalError;
use errors::internal_error::builder::InternalErrorBuilder;
use liquidity::liquidity_client::LiquidityClient;

use crate::repository::pools_repo;
use crate::pool_snapshots::pool_snapshot_service;
use crate::pools::pool::Pool;

pub async fn add_liquidity_to_pool(pool_id: String, amount: Nat) -> Result<AddLiquidityResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());
    if let Some(pool) = pool {
        let liquidity_client = liquidity_client(pool.clone()).await;

        match liquidity_client.add_liquidity_to_pool(amount).await {
            Ok(response) => {
                pool_snapshot_service::create_pool_snapshot(&pool).await;
                Ok(response)
            }
            Err(error) => {
                Err(
                    InternalErrorBuilder::business_logic()
                        .context("Liquidity service: add_liquidity_to_pool")
                        .message(format!("Error adding liquidity to pool: {}", error))
                        .build()
                )
            }
        }
    } else {
        Err(
            InternalErrorBuilder::not_found()
                .context("Liquidity service: add_liquidity_to_pool")
                .message(format!("Pool not found: {}", pool_id))
                .build()
        )
    }
}

pub async fn remove_liquidity_from_pool(pool_id: String) -> Result<WithdrawFromPoolResponse, InternalError> {
    let pool = pools_repo::get_pool_by_id(pool_id.clone());
    if let Some(pool) = pool {
        let liquidity_client = liquidity_client(pool.clone()).await;

        // Remove all liquidity from pool
        let total_shares = Nat::from(1 as u8);
        let shares = Nat::from(1 as u8);

        match liquidity_client.withdraw_liquidity_from_pool(total_shares, shares).await {
            Ok(response) => {
                Ok(response)
            }
            Err(error) => {
                let internal_error = InternalErrorBuilder::business_logic()
                    .context("Liquidity service: remove_liquidity_from_pool")
                    .message(format!("Error withdrawing from pool: {}", error))
                    .build();

                Err(internal_error)
            }
        }
    } else {
        let internal_error = InternalErrorBuilder::not_found()
            .context("Liquidity service: remove_liquidity_from_pool")
            .message(format!("Pool not found: {}", pool_id))
            .build();

        Err(internal_error)
    }
}

async fn liquidity_client(pool: Pool) -> Box<dyn LiquidityClient> {
    get_liquidity_client(
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await
}
