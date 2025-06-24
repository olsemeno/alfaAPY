use std::collections::HashMap;
use candid::Nat;

use ::types::context::Context;
use errors::internal_error::error::InternalError;
use errors::internal_error::error::build_error_code;
use liquidity::liquidity_router;
use swap::swap_service;
use types::exchange_id::ExchangeId;

use crate::repository::strategies_repo;
use crate::user::user_service;
use crate::strategies::strategy::IStrategy;
use crate::types::types::*;
use crate::repository::event_records_repo;
use crate::event_records::event_record::EventRecord;

// ========================== Strategies ==========================

/// Accepts an investment into a specified strategy.
///
/// # Arguments
///
/// * `args` - An `StrategyDepositArgs` struct containing the ledger, amount, and strategy ID.
///
/// # Returns
///
/// A `Result` containing a `StrategyDepositResponse` struct (with the amount, shares, transaction ID, and request ID)
/// or a `InternalError` if the strategy is not found or the deposit fails.
///
/// # Errors
///
/// Returns a `InternalError` if the strategy is not found or if the deposit operation fails.
pub async fn deposit(context: Context, args: StrategyDepositArgs) -> Result<StrategyDepositResponse, InternalError> {
    let mut strategy = get_strategy_by_id(args.strategy_id.clone())
        .ok_or_else(|| {
            InternalError::not_found(
                build_error_code(3000, 1, 1), // 3000 01 01
                "service::deposit".to_string(),
                "Strategy not found".to_string(),
                Some(HashMap::from([
                    ("strategy_id".to_string(), args.strategy_id.to_string())
                ]))
            )
        })?;

    user_service::accept_deposit(context.clone(), args.amount.clone(), args.ledger, args.strategy_id).await?;

    strategy.deposit(context.clone(), context.user.unwrap(), args.amount.clone()).await
}

/// Withdraws an amount from a specified strategy.
///
/// # Arguments
///
/// * `args` - A `StrategyWithdrawArgs` struct containing the ledger, amount, and strategy ID.
///
/// # Returns
///
/// A `Result` containing a `StrategyWithdrawResponse` struct (with the withdrawn amount and current shares)
/// or a `InternalError` if the strategy is not found or the withdrawal fails.
///
/// # Errors
///
/// Returns a `InternalError` if the strategy is not found or if the withdrawal operation fails.
pub async fn withdraw(context: Context, args: StrategyWithdrawArgs) -> Result<StrategyWithdrawResponse, InternalError> {
    let mut strategy = get_strategy_by_id(args.strategy_id.clone())
        .ok_or_else(|| {
            InternalError::not_found(
                build_error_code(3100, 1, 2), // 3100 01 02
                "service::withdraw".to_string(),
                "Strategy not found".to_string(),
                Some(HashMap::from([
                    ("strategy_id".to_string(), args.strategy_id.to_string()),
                ]))
            )
        })?;

    strategy.withdraw(context.clone(), args.percentage.clone()).await
}

pub async fn strategy_liquidity(context: Context, strategy_id: u16) -> Result<Nat, InternalError> {
    let strategy = get_strategy_by_id(strategy_id)
        .ok_or_else(|| {
            InternalError::not_found(
                build_error_code(3000, 1, 3), // 3000 01 03
                "service::strategy_liquidity".to_string(),
                "Strategy not found".to_string(),
                Some(HashMap::from([
                    ("strategy_id".to_string(), strategy_id.to_string()),
                ]))
            )
        })?;

    let current_pool = strategy.get_current_pool();

    if current_pool.is_none() {
        return Err(InternalError::business_logic(
            build_error_code(3000, 1, 4), // 3000 0103 04
            "service::strategy_liquidity".to_string(),
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
                build_error_code(3000, 3, 5), // 3000 03 05
                "service::strategy_liquidity".to_string(),
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

// ========================== Event records ==========================

pub fn get_event_records(offset: u64, limit: u64) -> Result<Vec<EventRecord>, InternalError> {
    let result = event_records_repo::get_event_records(offset as usize, limit as usize);
    Ok(result)
}


/// Retrieves a strategy by its ID.
///
/// # Arguments
///
/// * `id` - The ID of the strategy to retrieve.
///
/// # Returns
///
/// A `Box<dyn IStrategy>` containing the strategy.
fn get_strategy_by_id(id: u16) -> Option<Box<dyn IStrategy>> {
    strategies_repo::get_strategy_by_id(id)
}
