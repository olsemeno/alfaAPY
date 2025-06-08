use candid::{CandidType, Deserialize, Principal, Nat};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::{call, id, trap, update};
use ic_cdk::api::call::CallResult;
use candid::export_service;

use types::exchanges::TokenInfo;
use types::exchange_id::ExchangeId;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use types::pool_stats::PoolByTokens;
use utils::pool_id_util::generate_pool_id;

use crate::pools::pool_snapshot::PoolSnapshot;
use crate::snapshots::snapshot_service;
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


// TODO: test method, remove after testing
use crate::pools::pool_data_service::{PositionData, PoolData};

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolSnapshotArgs {
    pub pool_id: String,
    pub timestamp: u64,
    pub position_data: Option<PositionData>,
    pub pool_data: Option<PoolData>,
}

// TODO: test method, remove after testing
#[update]
pub fn add_pool_snapshot(args: PoolSnapshotArgs) {
    let snapshot = PoolSnapshot::new(
        (pools_repo::get_pool_snapshots_count(args.pool_id.clone()) + 1).to_string(),
        args.pool_id,
        args.timestamp,
        args.position_data,
        args.pool_data,
    );
    pools_repo::save_pool_snapshot(snapshot);
}

// TODO: test method, remove after testing
#[update]
pub fn delete_pool_snapshots(pool_id: String) {
    pools_repo::delete_pool_snapshots(pool_id);
}

// TODO: test method, remove after testing
#[update]
pub fn delete_pool_snapshot(pool_id: String, snapshot_id: String) {
    pools_repo::delete_pool_snapshot(pool_id, snapshot_id);
}

// TODO: test method, remove after testing
#[update]
pub fn update_pool_ids() -> bool {
    let pools = pools_repo::get_pools();
    for mut pool in pools {
        let new_id = generate_pool_id(&pool.token0, &pool.token1, &pool.provider);
        pool.id = new_id;
        pools_repo::save_pool(pool);
    }
    true
}

// TODO: test method, remove after testing
#[update]
pub fn delete_all_pools_and_snapshots() -> bool {
    pools_repo::delete_all_pools_and_snapshots();
    true
}

// End of test method



// Pools management

#[update]
pub fn add_pool(token0: TokenInfo, token1: TokenInfo, provider: ExchangeId) -> String {
    let pool = Pool::create(token0, token1, provider);
    pool.id
}

#[update]
pub fn delete_pool(id: String) -> bool {
    pools_repo::get_pool_by_id(id.clone())
        .map(|pool| {
            pool.delete();
            true
        })
        .unwrap_or(false)
}

#[update]
pub fn get_pools() -> Vec<Pool> {
    pools_repo::get_pools()
}

#[update]
pub fn get_pool_by_id(id: String) -> Option<Pool> {
    pools_repo::get_pool_by_id(id)
}

#[update]
pub fn get_pool_by_tokens(pool_by_tokens: PoolByTokens) -> Option<Pool> {
    pools_repo::get_pool_by_tokens(pool_by_tokens)
}

// Pool metrics

#[update]
pub fn get_pool_metrics(pool_ids: Vec<String>) -> HashMap<String, PoolMetrics> {
    pool_ids.into_iter()
        .filter_map(|pool_id| {
            pools_repo::get_pool_by_id(pool_id.clone())
                .map(|pool| (pool_id, pool_metrics_service::create_pool_metrics(pool)))
        })
        .collect()
}

#[update]
pub fn get_pools_snapshots(pool_ids: Vec<String>) -> HashMap<String, Vec<PoolSnapshot>> {
    pool_ids.into_iter()
        .filter_map(|pool_id| {
            pools_repo::get_pool_by_id(pool_id.clone())
                .map(|pool| (pool_id, pools_repo::get_pool_snapshots(pool.id).unwrap_or_default()))
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

// Vault management

#[ic_cdk::init]
async fn init() {
    snapshot_service::start_pool_snapshots_timer(SNAPSHOTS_FETCHING_INTERVAL);
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    pools_repo::stable_save();
    snapshot_service::stop_pool_snapshots_timer();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    pools_repo::stable_restore();
    snapshot_service::start_pool_snapshots_timer(SNAPSHOTS_FETCHING_INTERVAL);
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
