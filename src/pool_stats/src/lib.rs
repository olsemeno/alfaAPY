use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::cell::RefCell;
use ic_cdk::{call, id, trap, update};
use ic_cdk::api::call::CallResult;
use candid::{candid_method, export_service};

use crate::snapshots::snapshot_service::{start_pool_snapshots_timer, stop_pool_snapshots_timer};


use candid::Nat;

use types::exchanges::TokenInfo;
use types::exchange_id::ExchangeId;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};

use crate::pools::pool::Pool;
use crate::pools::pool_metrics::PoolMetrics;
use crate::repository::pools_repo;
use crate::liquidity::liquidity_service;

pub mod pools;
pub mod liquidity;
// pub mod service;
pub mod repository;
pub mod snapshots;

const SNAPSHOTS_FETCHING_INTERVAL: u64 = 3600; // 1 hour

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct CanisterIdRequest {
    #[serde(rename = "canister_id")]
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
struct Config {
    operator: Option<Principal>,
}

thread_local! {
    static CONFIG: RefCell<Config> = RefCell::new(
        Config {
            operator: None
        },
    );
}


#[update]
pub fn add_pool(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) {
    Pool::new(
        pools_repo::get_pool_count().to_string(),
        token0,
        token1,
        provider,
    ).save();
}

#[update]
pub fn delete_pool(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) {
    if let Some(pool) = pools_repo::get_pool_by_tokens(token0, token1, provider) {
        pool.delete();
    }
}

#[update]
pub fn get_pools() -> Vec<Pool> {
    pools_repo::get_pools()
}

#[update]
pub fn get_pool_by_tokens(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) -> Option<Pool> {
    pools_repo::get_pool_by_tokens(token0, token1, provider)
}

#[update]
pub fn get_pool_metrics(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) -> Option<PoolMetrics> {
    let pool = pools_repo::get_pool_by_tokens(token0, token1, provider);

    if let Some(pool) = pool {
        Some(PoolMetrics::build(pool))
    } else {
        None
    }
}

pub async fn add_liquidity_to_pool(pool_id: &str, amount: Nat) -> Result<AddLiquidityResponse, String> {
    liquidity_service::add_liquidity_to_pool(pool_id, amount).await
}


pub async fn remove_liquidity_from_pool(pool_id: &str) -> Result<WithdrawFromPoolResponse, String> {
    liquidity_service::remove_liquidity_from_pool(pool_id).await
}




#[ic_cdk::init]
async fn init() {
    start_pool_snapshots_timer(SNAPSHOTS_FETCHING_INTERVAL);
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    stop_pool_snapshots_timer();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    start_pool_snapshots_timer(SNAPSHOTS_FETCHING_INTERVAL);
}

// Sets the admin principal.
#[update]
async fn set_admin(operator: Principal) {
    let controllers = get_controllers().await;
    if !controllers.contains(&ic_cdk::caller()) {
        trap("Unauthorized: caller is not a controller");
    }
    CONFIG.with(|config| {
        let mut config = config.borrow_mut();
        config.operator = Some(operator);
    });
}

async fn get_controllers() -> Vec<Principal> {
    let res: CallResult<(ic_cdk::api::management_canister::main::CanisterStatusResponse,)> = call(
        Principal::management_canister(),
        "canister_status",
        (CanisterIdRequest { canister_id: id() },),
    )
        .await;
    res
        .expect("Get controllers function exited unexpectedly: inter-canister call to management canister for canister_status returned an empty result.")
        .0.settings.controllers
}

export_service!();

#[ic_cdk_macros::query(name = "export_candid")]
fn export_candid() -> String {
    __export_service()
}
