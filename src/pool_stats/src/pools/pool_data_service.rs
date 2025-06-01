use candid::{CandidType, Deserialize};
use serde::Serialize;
use candid::Nat;

use crate::pools::pool::Pool;

use liquidity::liquidity_router;
use liquidity::liquidity_client::LiquidityClient;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PositionData {
    pub id: Nat,
    pub amount0: Nat,
    pub amount1: Nat,
    pub usd_amount0: Nat,
    pub usd_amount1: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolData {
    pub tvl: Nat,
    // pub balance0: Nat,
    // pub balance1: Nat,
    // pub lp_fee_0: Nat,
    // pub lp_fee_1: Nat,
}

pub async fn get_current_position(pool: &Pool) -> Option<PositionData> {
    let liquidity_client = get_liquidity_client(pool).await;
    let position_id = pool.position.as_ref().unwrap().id.clone();

    match liquidity_client.get_position_by_id(position_id).await {
        Ok(position) => {
            let current_position = PositionData {
                id: position.position_id,
                amount0: position.token_0_amount,
                amount1: position.token_1_amount,
                usd_amount0: position.usd_amount_0,
                usd_amount1: position.usd_amount_1,
            };
            Some(current_position)
        }
        Err(_error) => {
            None
        }
    }
}

pub async fn get_current_data(pool: &Pool) -> Option<PoolData> {
    let liquidity_client = get_liquidity_client(pool).await;

    match liquidity_client.get_pool_data().await {
        Ok(pool_data) => {
            let pool_data = PoolData {
                tvl: pool_data.tvl,
            };

            Some(pool_data)
        }
        Err(_error) => {
            None
        }
    }
}

async fn get_liquidity_client(pool: &Pool) -> Box<dyn LiquidityClient> {
    liquidity_router::get_liquidity_client(
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await
}
