use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::cell::RefCell;
use ic_cdk::{call, id, trap, update};
use ic_cdk::api::call::CallResult;

use crate::snapshots::snapshot_service::{start_pool_snapshots_timer, stop_pool_snapshots_timer};

pub mod pools;
pub mod liquidity;
pub mod service;
pub mod repository;
pub mod snapshots;

const SNAPSHOT_INTERVAL: u64 = 3600; // 1 hour

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

#[ic_cdk::init]
async fn init() {
    start_pool_snapshots_timer(SNAPSHOT_INTERVAL);
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    stop_pool_snapshots_timer();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    start_pool_snapshots_timer(SNAPSHOT_INTERVAL);
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
