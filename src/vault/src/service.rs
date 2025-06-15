use std::collections::HashMap;

use ::types::context::Context;
use errors::internal_error::error::InternalError;

use crate::repository::strategies_repo;
use crate::user::user_service;
use crate::strategies::strategy::IStrategy;
use crate::types::types::*;

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
                "vault::deposit".to_string(),
                "Strategy not found".to_string(),
                None,
                Some(HashMap::from([
                    ("strategy_id".to_string(), args.strategy_id.to_string())
                ]))
            )
        })?;

    user_service::accept_deposit(context.clone(), args.amount.clone(), args.ledger, args.strategy_id).await
        .map_err(|error| {
            error.wrap(
                "vault::deposit".to_string(),
                "Error calling 'user_service::accept_deposit'".to_string(),
                Some(HashMap::from([
                    ("strategy_id".to_string(), args.strategy_id.to_string()),
                    ("ledger".to_string(), args.ledger.to_string()),
                    ("amount".to_string(), args.amount.to_string()),
                    ("user".to_string(), context.user.unwrap().to_string()),
                ]))
            )
        })?;


    strategy.deposit(context.clone(), context.user.unwrap(), args.amount.clone()).await
        .map_err(|error| {
            error.wrap(
                "vault::deposit".to_string(),
                "Error calling 'strategy::deposit'".to_string(),
                Some(HashMap::from([
                    ("strategy_id".to_string(), args.strategy_id.to_string()),
                    ("ledger".to_string(), args.ledger.to_string()),
                    ("amount".to_string(), args.amount.to_string()),
                    ("user".to_string(), context.user.unwrap().to_string()),
                ]))
            )
        })
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
                "vault::withdraw".to_string(),
                "Strategy not found".to_string(),
                None,
                Some(HashMap::from([
                    ("strategy_id".to_string(), args.strategy_id.to_string()),
                ]))
            )
        })?;

    strategy.withdraw(context.clone(), args.amount.clone()).await
        .map_err(|error| {
            error.wrap(
                "vault::withdraw".to_string(),
                "Error calling 'strategy::withdraw'".to_string(),
                Some(HashMap::from([
                    ("strategy_id".to_string(), args.strategy_id.to_string()),
                    ("amount".to_string(), args.amount.to_string()),
                    ("user".to_string(), context.user.unwrap().to_string()),
                ]))
            )
        })
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
