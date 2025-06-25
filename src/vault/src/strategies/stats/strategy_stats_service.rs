use std::time::Duration;
use ic_cdk_timers::TimerId;
use candid::Nat;
use std::collections::HashMap;
use std::cell::RefCell;

use errors::internal_error::error::{InternalError, build_error_code};
use types::exchange_id::ExchangeId;
use liquidity::liquidity_router;
use swap::swap_service;
use utils::util::current_timestamp;

use crate::repository::strategies_repo;
use crate::strategies::strategy::IStrategy;

thread_local! {
    static STRATEGY_STATS_TIMER_ID: RefCell<Option<TimerId>> = RefCell::new(None);
}

fn set_timer_interval(
    interval: Duration,
    func: impl FnMut() + 'static,
) -> TimerId {
    ic_cdk_timers::set_timer_interval(interval, func)
}

pub fn start_strategy_stats_update_timer(interval: u64) {
    let timer_id = set_timer_interval(Duration::from_secs(interval), || {
        ic_cdk::spawn(async {
            update_all_strategy_liquidity().await;
        });
    });

    STRATEGY_STATS_TIMER_ID.with(|cell| {
        cell.replace(Some(timer_id));
    });
}

pub fn stop_strategy_stats_update_timer() {
    STRATEGY_STATS_TIMER_ID.with(|timer_id| {
        if let Some(timer_id) = timer_id.borrow_mut().take() {
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}

pub async fn update_all_strategy_liquidity() {
    let strategies = strategies_repo::get_all_strategies()
        .into_iter()
        .filter(|strategy| strategy.get_position_id().is_some())
        .collect::<Vec<_>>();

    for strategy in strategies {
        update_strategy_liquidity(strategy).await;
    }
}

pub async fn update_strategy_liquidity(mut strategy: Box<dyn IStrategy>) -> Result<(), InternalError> {
    let liquidity_amount = get_strategy_current_liquidity(strategy.as_ref()).await?;
    
    strategy.set_current_liquidity(Some(liquidity_amount));
    strategy.set_current_liquidity_updated_at(Some(current_timestamp()));

    strategies_repo::save_strategy(strategy);
    
    Ok(())
}

pub fn spawn_update_strategy_liquidity(strategy: Box<dyn IStrategy>) -> () {
    ic_cdk::spawn(async move {
        update_strategy_liquidity(strategy).await; // TODO: handle error
    });
}

pub async fn get_strategy_current_liquidity(strategy: &dyn IStrategy) -> Result<Nat, InternalError> {
    let strategy_id = strategy.get_id();
    let current_pool = strategy.get_current_pool();

    if current_pool.is_none() {
        return Err(InternalError::business_logic(
            build_error_code(3100, 3, 7), // 3100 03 07
            "strategy_stats_service::get_strategy_current_liquidity".to_string(),
            "Strategy has no current pool".to_string(),
            Some(HashMap::from([
                ("strategy_id".to_string(), strategy_id.to_string()),
            ]))
        ));
    }

    let pool = current_pool.unwrap();

    let liquidity_client = liquidity_router::get_liquidity_client(
        pool.token0,
        pool.token1,
        pool.provider
    ).await;

    let position_id = strategy.get_position_id()
        .ok_or_else(|| {
            InternalError::business_logic(
                build_error_code(3100, 3, 8), // 3100 03 08
                "strategy_stats_service::get_strategy_current_liquidity".to_string(),
                "Strategy has no position id".to_string(),
                Some(HashMap::from([
                    ("strategy_id".to_string(), strategy_id.to_string()),
                ]))
            )
        })?;

    let position_response = liquidity_client.get_position_by_id(position_id).await?;

    let quote_response = swap_service::quote_swap_icrc2(
        pool.token1,
        pool.token0,
        position_response.token_1_amount,
        ExchangeId::KongSwap
    ).await?;

    let base_token_amount = Nat::from(quote_response.amount_out) + position_response.token_0_amount;

    Ok(base_token_amount)
}
