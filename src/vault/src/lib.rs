mod swap;
mod providers;
mod strategies;
mod liquidity;
mod repo;
mod user;
mod util;
mod types;

use crate::providers::kong::kong::{ user_balances};
use crate::repo::repo::{get_all_strategies, get_strategy_by_id, stable_restore, stable_save};
use crate::strategies::strategy_service::{get_actual_strategies, init_strategies};
use crate::types::types::{DepositResponse, StrategyId, StrategyResponse, WithdrawResponse};
use crate::user::user_service::{accept_deposit};
use candid::{candid_method, CandidType, Deserialize, Nat};
use candid::{export_service, Principal};
use ic_cdk::{caller, id, print, trap};
use ic_cdk_macros::{ init, post_upgrade, pre_upgrade, query, update};
pub use kongswap_canister::pools::{PoolsReply, Response};
use kongswap_canister::user_balances::UserBalancesReply;
use providers::kong::kong::pools;
use serde::Serialize;
use std::cell::RefCell;
use ::types::CanisterId;

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


#[candid_method(update)]
#[update]
fn init_s(conf: Option<Conf>) {
    init_strategies();
}

//TODO remove / test method
#[update]
async fn kong_pools() -> PoolsReply {
    match pools().await {
        Ok(reply) => reply,
        Err(err) => {
            trap(format!("Error: {}", err).as_str());
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct AcceptInvestmentArgs {
    ledger: CanisterId,
    amount: Nat,
    strategy_id: StrategyId,
}

#[update]
async fn accept_investment(args: AcceptInvestmentArgs) -> DepositResponse {
    let _ = accept_deposit(args.amount.clone(), args.ledger, args.strategy_id).await;
    let mut str = get_strategy_by_id(args.strategy_id).unwrap();
    str.deposit(caller(), args.amount).await
}

// #[heartbeat]
#[allow(unused)]
fn heartbeat() {
    let n = 5 as u64;
    HEARTBEAT.with(|store| {
        let count = store.borrow_mut().clone();
        if count % n == 0 {
            // rebalance_all
        }
        store.replace(count + 1)
    });

    print("heartbeat");
}

#[update]
async fn user_balance_all() -> Vec<UserBalancesReply> {
    let canister_id = id();
    match user_balances(canister_id.to_text()).await.0 {
        Ok(reply) => reply,
        Err(err) => {
            trap(format!("Error: {}", err).as_str());
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct UserStrategyResponse {
    pub strategy_id: StrategyId,
    pub strategy_name: String,
    pub strategy_current_pool: String,
    pub total_shares: Nat,
    pub user_shares: Nat,
    pub initial_deposit: Nat,
}

#[update]
async fn user_strategies(user: Principal) -> Vec<UserStrategyResponse> {
    let strategies = get_all_strategies();
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
                    strategy_current_pool: pool.symbol,
                    total_shares: strategy.get_total_shares(),
                    user_shares: user_shares,
                    initial_deposit: initial_deposit,
                });
            }
        }
    }

    user_strategies
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct WithdrawArgs {
    ledger: CanisterId,
    amount: Nat, // TODO: rename to shares
    strategy_id: StrategyId,
}

#[update]
async fn withdraw(args: WithdrawArgs) -> WithdrawResponse {
    let mut str = get_strategy_by_id(args.strategy_id).unwrap();
    let response = str.withdraw(args.amount).await;

    WithdrawResponse {
        amount: response.amount,
        current_shares: response.current_shares,
    }
}

#[query]
fn get_config() -> Conf {
    CONF.with(|c| c.borrow().clone())
}

#[query]
fn get_strategies() -> Vec<StrategyResponse> {
    get_actual_strategies()
}

#[pre_upgrade]
fn pre_upgrade() {
    stable_save();
}

#[derive(CandidType, Deserialize, Eq, PartialEq, Debug)]
pub struct SupportedStandard {
    pub url: String,
    pub name: String,
}

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

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Icrc28TrustedOriginsResponse {
    pub trusted_origins: Vec<String>,
}

// list every base URL that users will authenticate to your app from
#[update]
fn icrc28_trusted_origins() -> Icrc28TrustedOriginsResponse {
    let trusted_origins = vec![
        String::from("https://47r3x-paaaa-aaaao-qj6ha-cai.icp0.io"),
    ];

    Icrc28TrustedOriginsResponse { trusted_origins }
}

#[post_upgrade]
pub async fn post_upgrade() {
    stable_restore()
}
export_service!();

#[ic_cdk_macros::query(name = "export_candid")]
fn export_candid() -> String {
    __export_service()
}
