mod swap;
mod providers;
mod strategies;
mod liquidity;
mod repo;
mod user;

use crate::repo::repo::{get_all_strategies, get_strategy_by_id, stable_restore, stable_save};
use crate::strategies::strategy::{DepositResponse, StrategyId, StrategyResponse};
use crate::strategies::strategy_candid::StrategyCandid;
use crate::strategies::strategy_service::{get_actual_strategies, init_strategies};
use crate::swap::swap_service::{swap_icrc2_kong, KONG_BE_CANISTER};
use crate::user::user_service::{accept_deposit, withdraw_from_strategy};
use candid::{candid_method, CandidType, Deserialize, Nat};
use candid::{export_service, Principal};
use ic_cdk::{call, caller, print, trap};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
pub use kongswap_canister::pools::{PoolsReply, Response};
use providers::kong::kong::pools;
use serde::Serialize;
use std::cell::RefCell;
use ic_cdk::api::call::CallResult;
use kongswap_canister::add_liquidity_amounts::AddLiquidityAmountsReply;
use types::exchanges::TokenInfo;
use types::swap_tokens::{Response as R2, SuccessResult};
use types::CanisterId;
use crate::providers::kong::kong::add_liquidity_amounts;

thread_local! {
    pub static CONF: RefCell<Conf> = RefCell::new(Conf::default());
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


//dummy test method
#[update]
async fn swap() -> SuccessResult {
    let source = TokenInfo {
        ledger: CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai".to_string()).unwrap(),
        symbol: "ICP".to_string(),
    };

    let target = TokenInfo {
        ledger: CanisterId::from_text("xevnm-gaaaa-aaaar-qafnq-cai".to_string()).unwrap(),
        symbol: "ICP".to_string(),
    };

    swap_icrc2_kong(source, target, 1000).await
}


#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct AcceptInvestmentArgs {
    ledger: CanisterId,
    amount: Nat,
    strategy_id: StrategyId,
}

#[update]
async fn accept_investment(args: AcceptInvestmentArgs) -> DepositResponse  {
    //1000 ICP ryjl3-tyaaa-aaaaa-aaaba-cai 2
    accept_deposit(args.amount.clone(), args.ledger, args.strategy_id).await;


    let  mut str = get_strategy_by_id(args.strategy_id).unwrap() ;
    str.deposit(caller(), args.amount).await
}
use kongswap_canister::queries::add_liquidity_amounts::{Args as AddLiquidityAmountsArgs, Response as AddLiquidityAmountsResponse};


#[update]
async fn accept_investment2 () -> Result<AddLiquidityAmountsReply, String>  {
    //1000 ICP ryjl3-tyaaa-aaaaa-aaaba-cai 2

    let a: CallResult<(Result<AddLiquidityAmountsReply, String>,)> = call(
        KONG_BE_CANISTER,
        "add_liquidity_amounts",
        ( String::from("ICP"), Nat::from(100 as usize), String::from("ckUSDT"),
        )
    ).await;

    match a {
        Ok(x) => {
            x.0
        }
        Err(l) => {
           trap(format!("Error: {}", l.1).as_str());
        }
    }
}


#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct WithdrawArgs {
    ledger: CanisterId,
    amount: Nat,
    strategy_id: StrategyId,
}

#[update]
async fn withdraw(args: WithdrawArgs)  -> Result<Nat, String>  {
    withdraw_from_strategy(args.strategy_id, args.amount, args.ledger).await
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

#[post_upgrade]
pub async fn post_upgrade() {
    stable_restore()
}
export_service!();

#[ic_cdk_macros::query(name = "export_candid")]
fn export_candid() -> String {
    __export_service()
}