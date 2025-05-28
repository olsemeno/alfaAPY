use candid::{CandidType, Deserialize};
use serde::Serialize;
use candid::Nat;

use crate::pools::pool::Pool;

use liquidity::liquidity_router::get_liquidity_client;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct CurrentPosition {
    pub id: Nat,
    pub current_amount0: Nat,
    pub current_amount1: Nat,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolCurrentData {
    pub balance0: Nat,
    pub balance1: Nat,
    pub lp_fee_0: Nat,
    pub lp_fee_1: Nat,
}

pub async fn get_current_position(pool: &Pool) -> Option<CurrentPosition> {
    let liquidity_client = get_liquidity_client(
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await;

    let position_id = pool.position.as_ref().unwrap().id.clone();

    let position = liquidity_client.get_position_by_id(position_id).await;

    match position {
        Ok(position) => {
            let current_position = CurrentPosition {
                id: position.position_id,
                current_amount0: position.token_0_amount,
                current_amount1: position.token_1_amount,
            };
            Some(current_position)
        }
        Err(_error) => {
            None
        }
    }
}

// TODO: implement get_current_data
pub async fn get_current_data(pool: &Pool) -> PoolCurrentData {
    // Call liquidity service to get current data
    PoolCurrentData {
        balance0: Nat::from(0 as u64),
        balance1: Nat::from(0 as u64),
        lp_fee_0: Nat::from(0 as u64),
        lp_fee_1: Nat::from(0 as u64),
    }
}