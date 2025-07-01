use candid::{CandidType, Deserialize, Principal, Nat};
use serde::Serialize;
use std::cell::RefCell;
use ic_cdk::{call, id, trap, update, caller};
use ic_cdk::api::call::CallResult;
use candid::export_service;

use ::types::exchange_id::ExchangeId;
use ::types::context::Context;
use ::types::CanisterId;
use ::types::pool::PoolTrait;
use errors::response_error::error::ResponseError;
use errors::internal_error::error::InternalError;
use errors::internal_error::error::build_error_code;

use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::pool_snapshots::pool_snapshot_service;
use crate::pools::pool::Pool;
use crate::repository::pools_repo;
use crate::repository::stable_state;
use crate::pools::pool_service;
use crate::types::types::{
    AddLiquidityResult,
    WithdrawLiquidityResult,
    AddPoolResult,
    DeletePoolResult,
    GetPoolsResult,
    GetPoolByIdResult,
    GetPoolMetricsResult,
    GetPoolsSnapshotsResult,
    GetEventRecordsResult,
};

pub mod pools;
pub mod liquidity;
pub mod repository;
pub mod pool_snapshots;
pub mod pool_metrics;
pub mod event_records;
pub mod types;
pub mod service;

// const SNAPSHOTS_FETCHING_INTERVAL: u64 = 3600; // 1 hour
const SNAPSHOTS_FETCHING_INTERVAL: u64 = 604_800; // 1 week

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
use crate::pool_snapshots::position_data::position_data::PositionData;
use crate::pool_snapshots::pool_data::pool_data::PoolData;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolSnapshotArgs {
    pub pool_id: String,
    pub timestamp: u64,
    pub position_data: Option<PositionData>,
    pub pool_data: Option<PoolData>,
}

// TODO: test method, remove after testing
#[update]
pub fn test_add_pool_snapshot(args: PoolSnapshotArgs) {
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
pub fn test_delete_pool_snapshots(pool_id: String) {
    pools_repo::delete_pool_snapshots(pool_id);
}

// TODO: test method, remove after testing
#[update]
pub fn test_delete_pool_snapshot(pool_id: String, snapshot_id: String) {
    pools_repo::delete_pool_snapshot(pool_id, snapshot_id);
}

// TODO: test method, remove after testing
#[update]
pub fn test_update_pool_ids() {
    let pools = pools_repo::get_pools();
    for mut pool in pools {
        let new_id = Pool::generate_pool_id(&pool.token0, &pool.token1, &pool.provider);
        pool.id = new_id;
        pools_repo::save_pool(pool);
    }
}

// TODO: test method, remove after testing
#[update]
pub fn test_delete_all_pools_and_snapshots() {
    pools_repo::delete_all_pools_and_snapshots()
}

// TODO: test method, remove after testing
#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct TestCreatePoolSnapshotResult(pub Result<PoolSnapshot, ResponseError>);

#[update]
pub async fn test_create_pool_snapshot(pool_id: String) -> TestCreatePoolSnapshotResult {
    let context = Context::generate(None);

    let pool = pools_repo::get_pool_by_id(pool_id.clone());

    if let Some(pool) = pool {
        let result = pool_snapshot_service::create_pool_snapshot(
            context,
            &pool
        ).await
            .map_err(|error| ResponseError::from_internal_error(error));

        TestCreatePoolSnapshotResult(result)
    } else {
        let error = InternalError::not_found(
            build_error_code(0000, 0, 0), // 0000 00 00
            "pool_stats::create_pool_snapshot".to_string(),
            format!("Pool not found: {pool_id}"),
            None
        );

        TestCreatePoolSnapshotResult(Err(ResponseError::from_internal_error(error)))
    }
}

// ========================== End of test method ==========================





// ========================== Pools management ==========================

#[update]
pub fn add_pool(token0: CanisterId, token1: CanisterId, provider: ExchangeId) -> AddPoolResult {
    let result = service::add_pool(token0, token1, provider)
        .map_err(|error| ResponseError::from_internal_error(error));

    AddPoolResult(result)
}

#[update]
pub fn delete_pool(id: String) -> DeletePoolResult {
    let result = service::delete_pool(id)
        .map_err(|error| ResponseError::from_internal_error(error));

    DeletePoolResult(result)
}

#[update]
pub fn get_pools() -> GetPoolsResult {
    let result = service::get_pools()
        .map_err(|error| ResponseError::from_internal_error(error));

    GetPoolsResult(result)
}

#[update]
pub fn get_pool_by_id(id: String) -> GetPoolByIdResult {
    let result = service::get_pool_by_id(id)
        .map_err(|error| ResponseError::from_internal_error(error));

    GetPoolByIdResult(result)
}

// ========================== Pool metrics ==========================

#[update]
pub fn get_pool_metrics(pool_ids: Vec<String>) -> GetPoolMetricsResult {
    let result = service::get_pool_metrics(pool_ids);

    GetPoolMetricsResult(result)
}

#[update]
pub fn get_pools_snapshots(pool_ids: Vec<String>) -> GetPoolsSnapshotsResult {
    let result = service::get_pools_snapshots(pool_ids);

    GetPoolsSnapshotsResult(result)
}

// ========================== Liquidity management ==========================

#[update]
pub async fn add_liquidity_to_pool(
    ledger: CanisterId,
    pool_id: String,
    amount: Nat
) -> AddLiquidityResult {
    let context = generate_context();

    let result = service::add_liquidity_to_pool(
        context,
        ledger,
        pool_id,
        amount
    ).await
        .map_err(|error| ResponseError::from_internal_error(error));

    AddLiquidityResult(result)
}

#[update]
pub async fn withdraw_liquidity_from_pool(pool_id: String) -> WithdrawLiquidityResult {
    let context = generate_context();

    let result = service::withdraw_liquidity_from_pool(
        context,
        pool_id
    ).await
        .map_err(|error| ResponseError::from_internal_error(error));

    WithdrawLiquidityResult(result)
}

fn generate_context() -> Context {
    Context::generate(Some(caller()))
}

// ========================== Event records ==========================

#[update]
pub fn get_event_records(offset: u64, limit: u64) -> GetEventRecordsResult {
    let result = service::get_event_records(offset, limit)
        .map_err(|error| ResponseError::from_internal_error(error));

    GetEventRecordsResult(result)
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
        .expect(
            "Get controllers function exited unexpectedly:\n\
            inter-canister call to management canister for canister_status returned an empty result."
        )
        .0.settings.controllers
}

export_service!();

#[ic_cdk_macros::query(name = "export_candid")]
fn export_candid() -> String {
    __export_service()
}
