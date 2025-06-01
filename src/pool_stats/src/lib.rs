use candid::{CandidType, Deserialize, Principal, Nat};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::{call, id, trap, update};
use ic_cdk::api::call::CallResult;
use candid::export_service;

use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use types::pool_stats::PoolByTokens;
use crate::pools::pool_snapshot::PoolSnapshot;

use crate::snapshots::snapshot_service::{start_pool_snapshots_timer, stop_pool_snapshots_timer};
use crate::pools::pool::Pool;
use crate::pools::pool_metrics::PoolMetrics;
use crate::pools::pool_metrics_service;
use crate::repository::pools_repo;
use crate::liquidity::liquidity_service;

pub mod pools;
pub mod liquidity;
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

// Pools management

#[update]
pub fn add_pool(pool_by_tokens: PoolByTokens) {
    Pool::new(
        pools_repo::get_pool_count().to_string(),
        pool_by_tokens.token0,
        pool_by_tokens.token1,
        pool_by_tokens.provider,
    ).save();
}

#[update]
pub fn delete_pool(pool_by_tokens: PoolByTokens) {
    if let Some(pool) = pools_repo::get_pool_by_tokens(pool_by_tokens) {
        pool.delete();
    }
}

#[update]
pub fn get_pools() -> Vec<Pool> {
    pools_repo::get_pools()
}

#[update]
pub fn get_pool_by_tokens(pool_by_tokens: PoolByTokens) -> Option<Pool> {
    pools_repo::get_pool_by_tokens(pool_by_tokens)
}

// Pool metrics

#[update]
pub fn get_pool_metrics(pool_by_tokens: Vec<PoolByTokens>) -> HashMap<PoolByTokens, PoolMetrics> {
    pool_by_tokens.into_iter()
        .filter_map(|pool_by_tokens| {
            pools_repo::get_pool_by_tokens(pool_by_tokens.clone())
                .map(|pool| (pool_by_tokens, pool_metrics_service::create_pool_metrics(pool)))
        })
        .collect()
}

#[update]
pub fn get_pools_snapshots(pools_by_tokens: Vec<PoolByTokens>) -> HashMap<PoolByTokens, Vec<PoolSnapshot>> {
    pools_by_tokens.into_iter()
        .filter_map(|pool_by_tokens| {
            pools_repo::get_pool_by_tokens(pool_by_tokens.clone())
                .map(|pool| (pool_by_tokens, pools_repo::get_pool_snapshots(pool.id).unwrap_or_default()))
        })
        .collect()
}

// Liquidity management

#[update]
pub async fn add_liquidity_to_pool(pool_id: String, amount: Nat) -> Result<AddLiquidityResponse, String> {
    liquidity_service::add_liquidity_to_pool(pool_id, amount).await
}

#[update]
pub async fn remove_liquidity_from_pool(pool_id: String) -> Result<WithdrawFromPoolResponse, String> {
    liquidity_service::remove_liquidity_from_pool(pool_id).await
}

#[update]
pub fn add_pool_snapshot(snapshot: PoolSnapshot) {
    pools_repo::save_pool_snapshot(snapshot);
}

// Vault management

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

// Sets the operator principal.
#[update]
async fn set_operator(operator: Principal) {
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
