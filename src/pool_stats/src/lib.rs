use candid::{CandidType, Deserialize, Principal, Nat};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::{call, id, trap, update, caller};
use ic_cdk::api::call::CallResult;
use candid::export_service;

use types::exchange_id::ExchangeId;
use types::liquidity::{AddLiquidityResponse, WithdrawFromPoolResponse};
use types::context::Context;
use types::CanisterId;
use types::pool::PoolTrait;
use errors::response_error::error::ResponseError;
use utils::token_transfer::icrc2_transfer_from_user;

use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::pool_snapshots::pool_snapshot_service;
use crate::pools::pool::Pool;
use crate::pool_metrics::pool_metrics::PoolMetrics;
use crate::pool_metrics::pool_metrics_service;
use crate::repository::pools_repo;
use crate::repository::stable_state;
use crate::liquidity::liquidity_service;
use crate::pools::pool_service;

pub mod pools;
pub mod liquidity;
pub mod repository;
pub mod pool_snapshots;
pub mod pool_metrics;
pub mod event_logs;

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

// ========================== Test methods ==========================

// TODO: test method, remove after testing
use crate::pool_snapshots::pool_snapshot::{PositionData, PoolData};

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
        let new_id = Pool::generate_pool_id(&pool.token0, &pool.token1, &pool.provider);
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

// TODO: test method, remove after testing
#[update]
pub async fn create_pool_snapshot(pool_id: String) -> PoolSnapshot {
    let context = Context::generate(None);

    let pool = pools_repo::get_pool_by_id(pool_id.clone()).unwrap();
    pool_snapshot_service::create_pool_snapshot(context, &pool).await
}

// ========================== End of test method ==========================

// ========================== Pools management ==========================

#[update]
pub fn add_pool(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> String {
    let pool = Pool::build(token0, token1, provider);
    pool.save();
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

// ========================== Pool metrics ==========================

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

// ========================== Liquidity management ==========================

#[update]
pub async fn add_liquidity_to_pool(
    ledger: CanisterId,
    pool_id: String,
    amount: Nat
) -> Result<AddLiquidityResponse, ResponseError> {
    let context = Context::generate(Some(caller()));

    match icrc2_transfer_from_user(context.clone(), caller(), ledger, amount.clone()).await {
        Ok(_) => {
            match liquidity_service::add_liquidity_to_pool(context, pool_id, amount).await {
                Ok(response) => Ok(response),
                Err(error) => ResponseError::from_internal_error(error),
            }
        },
        Err(error) => ResponseError::from_internal_error(error),
    }
}

#[update]
pub async fn remove_liquidity_from_pool(pool_id: String) -> Result<WithdrawFromPoolResponse, ResponseError> {
    let context = Context::generate(Some(caller()));

    match liquidity_service::remove_liquidity_from_pool(context, pool_id).await {
        Ok(response) => Ok(response),
        Err(error) => ResponseError::from_internal_error(error)
    }
}

// ========================== Vault management ==========================

#[ic_cdk::init]
async fn init() {
    pool_service::init_pools();
    pool_snapshot_service::start_pool_snapshots_timer(SNAPSHOTS_FETCHING_INTERVAL);
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    stable_state::stable_save();
    pool_snapshot_service::stop_pool_snapshots_timer();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    stable_state::stable_restore();
    pool_snapshot_service::start_pool_snapshots_timer(SNAPSHOTS_FETCHING_INTERVAL);
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
