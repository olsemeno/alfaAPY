use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;
use ic_cdk_timers::TimerId;

use liquidity::liquidity_router;
use liquidity::liquidity_client::LiquidityClient;
use types::context::Context;
use errors::internal_error::error::InternalError;
use errors::internal_error::error::build_error_code;

use crate::pools::pool::Pool;
use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::pool_snapshots::position_data::position_data::PositionData;
use crate::pool_snapshots::pool_data::pool_data::PoolData;
use crate::repository::pools_repo;
use crate::utils::provider_impls::get_environment_provider_impls;

thread_local! {
    static POOL_SNAPSHOT_TIMER_ID: RefCell<Option<TimerId>> = RefCell::new(None);
}

fn set_timer_interval(
    interval: Duration,
    func: impl FnMut() + 'static,
) -> TimerId {
    ic_cdk_timers::set_timer_interval(interval, func)
}

pub fn start_pool_snapshots_timer(interval: u64) {
    let timer_id = set_timer_interval(Duration::from_secs(interval), || {
        ic_cdk::spawn(async {
            create_all_pool_snapshots().await;
        });
    });

    POOL_SNAPSHOT_TIMER_ID.with(|cell| {
        cell.replace(Some(timer_id));
    });
}

pub fn stop_pool_snapshots_timer() {
    POOL_SNAPSHOT_TIMER_ID.with(|timer_id| {
        if let Some(timer_id) = timer_id.borrow_mut().take() {
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}

pub async fn create_all_pool_snapshots() {
    let context = Context::generate(None);

    let pools = pools_repo::get_pools();

    for pool in pools.into_iter().filter(|p| p.position_id.is_some()) {
        create_pool_snapshot(context.clone(), &pool).await
        .map_err(|error| {
            // TODO: add event logging
        });
    }
}

pub async fn create_pool_snapshot(context: Context, pool: &Pool) -> Result<PoolSnapshot, InternalError> {
    if pool.position_id.is_none() {
        return Err(InternalError::business_logic(
            build_error_code(4100, 3, 1), // 4100 03 01
            "pool_snapshot_service::create_pool_snapshot".to_string(),
            "Pool has no position_id".to_string(),
            Some(HashMap::from([
                ("pool_id".to_string(), pool.id.to_string()),
            ])),
        ));
    }

    let pool_data = get_pool_data(context.clone(), pool).await?;
    let position_data = get_position_data(context, pool).await?;

    let snapshot = PoolSnapshot::create(pool.id.clone(), position_data, pool_data)?;

    Ok(snapshot)
}

async fn get_position_data(context: Context, pool: &Pool) -> Result<Option<PositionData>, InternalError> {
    let liquidity_client = get_liquidity_client(pool).await;

    if let Some(position_id) = pool.position_id.as_ref().cloned() {
        let position_response = liquidity_client.get_position_by_id(position_id).await?;

        let current_position = PositionData {
            id: position_response.position_id,
            amount0: position_response.token_0_amount,
            amount1: position_response.token_1_amount,
            usd_amount0: position_response.usd_amount_0,
            usd_amount1: position_response.usd_amount_1,
        };

        Ok(Some(current_position))
    } else {
        Ok(None)
    }
}

async fn get_pool_data(context: Context, pool: &Pool) -> Result<Option<PoolData>, InternalError> {
    let liquidity_client = get_liquidity_client(pool).await;
    let pool_data_response = liquidity_client.get_pool_data().await?;

    let pool_data = PoolData {
        tvl: pool_data_response.tvl,
    };

    Ok(Some(pool_data))
}

async fn get_liquidity_client(pool: &Pool) -> Box<dyn LiquidityClient> {
    liquidity_router::get_liquidity_client(
        get_environment_provider_impls(),
        pool.token0.clone(),
        pool.token1.clone(),
        pool.provider.clone()
    ).await
}
