mod strategies;
pub mod liquidity;
mod repository;
mod user;
mod types;
mod events;
mod enums;
mod pools;
mod pool_stats;

use serde::Serialize;
use std::cell::RefCell;
use candid::{candid_method, CandidType, Deserialize, Nat};
use candid::{export_service, Principal};
use ic_cdk::{caller, trap};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};

use providers::{icpswap as icpswap_provider};
use ::types::CanisterId;

use crate::repository::repo::{stable_restore, stable_save};
use crate::repository::strategies_repo;
use crate::strategies::strategy_service::{get_actual_strategies, init_strategies};
use crate::user::user_service::accept_deposit;
use crate::events::event_service;
use crate::events::event::{SystemEvent, UserEvent};
use crate::types::types::{
    AcceptInvestmentArgs,
    DepositResponse,
    Icrc28TrustedOriginsResponse,
    StrategyResponse,
    SupportedStandard,
    UserStrategyResponse,
    WithdrawArgs,
    WithdrawResponse
};

thread_local! {
    pub static CONF: RefCell<Conf> = RefCell::new(Conf::default());
    pub static HEARTBEAT: RefCell<u64> = RefCell::new(0);
}

#[derive(CandidType, Deserialize, Clone, Debug, Hash, PartialEq, Serialize)]
pub struct Conf {
    pub controllers: Option<Vec<Principal>>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            controllers: Default::default()
        }
    }
}

/// Initializes the canister with the given configuration.
///
/// # Arguments
///
/// * `conf` - An optional configuration object of type `Conf`.
///
/// # Description
///
/// This function sets the initial configuration for the canister. If a configuration
/// is provided, it replaces the default configuration with the provided one. It also
/// initializes the strategies by calling `init_strategies()`.
#[init]
#[candid_method(init)]
fn init(conf: Option<Conf>) {
    match conf {
        None => {}
        Some(conf) => {
            CONF.with(|c| c.replace(conf));
        }
    };
    init_strategies();
}

/// The heartbeat function is called periodically to perform maintenance tasks.
///
/// This function increments a counter and checks if a day has passed (based on the counter value).
/// TODO make unique for each strategy
/// If a day has passed, it triggers the rebalance operation for all strategies.
// #[heartbeat]
#[allow(unused)]
fn heartbeat() {
    let n = (3600 * 24) as u64;
    HEARTBEAT.with(|store| {
        // TODO: uncomment
        // let count = store.borrow_mut().clone();
        // if count % n == 0 {
        //     STRATEGIES.with(|strategies| {
        //         let mut strategies = strategies.borrow_mut();
        //         for strategy in strategies.iter_mut() {
        //             strategy.rebalance();
        //         }
        //     });
        // }
        // store.replace(count + 1)
    });
}

// =============== Test functions ===============

// TODO: Test function. Remove after testing.
#[update]
async fn icpswap_withdraw(token_out: CanisterId, amount: Nat, token_fee: Nat) -> Nat {
    let canister_id = Principal::from_text("5fq4w-lyaaa-aaaag-qjqta-cai").unwrap();

    let icpswap_quote_result = icpswap_provider::withdraw(
        canister_id,
        token_out,
        amount,
        token_fee
    ).await;

    icpswap_quote_result.unwrap()
}

// TODO: Test function. Remove after testing.
#[update]
async fn reset_strategy(strategy_id: u16) {
    let mut strategy = strategies_repo::get_strategy_by_id(strategy_id).unwrap();
    strategy.reset_strategy().await;
}

// =============== Events ===============

#[update]
async fn get_system_events(offset: u64, limit: u64) -> Vec<SystemEvent> {
    event_service::get_system_events(offset as usize, limit as usize)
}

#[update]
async fn get_user_events(user: Principal, offset: u64, limit: u64) -> Vec<UserEvent> {
    event_service::get_user_events(user, offset as usize, limit as usize)
}

// =============== Strategies ===============

/// Accepts an investment into a specified strategy.
///
/// # Arguments
///
/// * `args` - An `AcceptInvestmentArgs` struct containing the ledger, amount, and strategy ID.
///
/// # Returns
///
/// A `DepositResponse` struct containing the amount, shares, transaction ID, and request ID.
///
/// # Errors
///
/// This function will trap if the strategy ID is not found
#[update]
async fn accept_investment(args: AcceptInvestmentArgs) -> DepositResponse {
    match accept_deposit(args.amount.clone(), args.ledger, args.strategy_id).await {
        Ok(_) => {
            let mut strategy = strategies_repo::get_strategy_by_id(args.strategy_id).unwrap();
            strategy.deposit(caller(), args.amount).await
        }
        Err(e) => {
            trap(format!("Error accepting investment: {}", e).as_str());
        }
    }
}

/// Withdraws an amount from a specified strategy.
///
/// # Arguments
///
/// * `args` - A `WithdrawArgs` struct containing the ledger, amount, and strategy ID.
///
/// # Returns
///
/// A `WithdrawResponse` struct containing the amount and current shares.
///
/// # Errors
///
/// This function will trap if the strategy ID is not found.
#[update]
async fn withdraw(args: WithdrawArgs) -> WithdrawResponse {
    let mut strategy = strategies_repo::get_strategy_by_id(args.strategy_id).unwrap();
    let withdraw_response = strategy.withdraw(args.amount).await;

    WithdrawResponse {
        amount: withdraw_response.amount,
        current_shares: withdraw_response.current_shares,
    }
}

/// Retrieves the strategies for a specific user.
///
/// # Arguments
///
/// * `user` - The `Principal` of the user.
///
/// # Returns
///
/// A vector of `UserStrategyResponse` containing the strategies information for the user.
#[update]
async fn user_strategies(user: Principal) -> Vec<UserStrategyResponse> {
    // TODO: rename user_strategies to user_positions

    let strategies = strategies_repo::get_user_strategies(user);
    let mut user_strategies = Vec::new();

    for strategy in strategies {
        let user_shares = strategy.get_user_shares().get(&user).cloned().unwrap_or(Nat::from(0u64));
        let initial_deposit = strategy.get_initial_deposit().get(&user).cloned().unwrap_or(Nat::from(0u64));
        let current_pool = strategy.get_current_pool();

        if let Some(pool) = current_pool {
            // Add only if current pool is set and user has shares
            if user_shares > Nat::from(0u64) {
                user_strategies.push(UserStrategyResponse {
                    strategy_id: strategy.get_id(),
                    strategy_name: strategy.get_name(),
                    strategy_current_pool: pool,
                    total_shares: strategy.get_total_shares(),
                    user_shares,
                    initial_deposit,
                    users_count: strategy.get_users_count(),
                });
            }
        }
    }

    user_strategies
}

#[query]
fn get_strategies() -> Vec<StrategyResponse> {
    get_actual_strategies()
}

/// Retrieves the current configuration.
///
/// # Returns
///
/// A `Conf` struct containing the current configuration.
#[query]
fn get_config() -> Conf {
    CONF.with(|c| c.borrow().clone())
}

// =============== ICRC ===============

/// Retrieves the supported standards for ICRC-10.
///
/// # Returns
///
/// A vector of `SupportedStandard` containing the supported standards.
#[query]
fn icrc10_supported_standards() -> Vec<SupportedStandard> {
    vec![
        SupportedStandard {
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-10/ICRC-10.md".to_string(),
            name: "ICRC-10".to_string(),
        },
        SupportedStandard {
            url: "https://github.com/dfinity/wg-identity-authentication/blob/main/topics/icrc_28_trusted_origins.md".to_string(),
            name: "ICRC-28".to_string(),
        },
    ]
}

/// Retrieves the trusted origins for ICRC-28.
///
/// # Returns
///
/// An `Icrc28TrustedOriginsResponse` struct containing the trusted origins.
#[update]
fn icrc28_trusted_origins() -> Icrc28TrustedOriginsResponse {
    let trusted_origins = vec![
        String::from("https://47r3x-paaaa-aaaao-qj6ha-cai.icp0.io"),
    ];

    Icrc28TrustedOriginsResponse { trusted_origins }
}

// =============== Upgrade ===============

#[pre_upgrade]
fn pre_upgrade() {
    stable_save()
}

#[post_upgrade]
fn post_upgrade() {
    stable_restore()
}

export_service!();

#[ic_cdk_macros::query(name = "export_candid")]
fn export_candid() -> String {
    __export_service()
}
