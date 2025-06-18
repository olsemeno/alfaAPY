use std::collections::HashMap;

use ::types::context::Context;
use errors::internal_error::error::InternalError;
use errors::internal_error::error::build_error_code;

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
                build_error_code(3000, 1, 1), // 3000 01 01
                "vault::deposit".to_string(),
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
                "vault::withdraw".to_string(),
                "Strategy not found".to_string(),
                Some(HashMap::from([
                    ("strategy_id".to_string(), args.strategy_id.to_string()),
                ]))
            )
        })?;

    strategy.withdraw(context.clone(), args.amount.clone()).await
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
