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
use ic_cdk::{call, caller, id, print, trap};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
pub use kongswap_canister::pools::{PoolsReply, Response};
use providers::kong::kong::pools;
use serde::Serialize;
use std::cell::RefCell;
use std::env::args;
use ic_cdk::api::call::CallResult;
use kongswap_canister::add_liquidity_amounts::AddLiquidityAmountsReply;
use kongswap_canister::user_balances::UserBalancesReply;
use types::exchanges::TokenInfo;
use types::swap_tokens::{Response as R2, SuccessResult};
use types::CanisterId;
use crate::providers::kong::kong::{add_liquidity_amounts, user_balances};

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



#[query]
async fn user_balance_all(user: String) -> Vec<UserBalancesReply>  {
    //1000 ICP ryjl3-tyaaa-aaaaa-aaaba-cai 2

   match user_balances(user).await.0 {
         Ok(reply) => reply,
         Err(err) => {
              trap(format!("Error: {}", err).as_str());
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
    let  mut str = get_strategy_by_id(args.strategy_id).unwrap() ;
   let resp =  str.withdraw(caller(), args.amount).await;
    Ok(resp.amount)
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
    pub trusted_origins: Vec<String>
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