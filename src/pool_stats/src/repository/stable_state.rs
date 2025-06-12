use candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_cdk::storage;
use std::collections::HashMap;

use crate::event_logs::event_log::EventLog;

use crate::pools::pool::Pool;
use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::repository::pools_repo::POOLS;
use crate::repository::pools_repo::POOLS_SNAPSHOTS;
use crate::repository::event_logs_repo::EVENT_LOGS;

#[derive(Serialize, Deserialize, CandidType)]
pub struct StableState {
    pub pools: HashMap<String, Pool>,
    pub pool_snapshots: HashMap<String, Vec<PoolSnapshot>>,
    pub event_logs: Vec<EventLog>,
}

pub fn stable_save() {
    let pools = POOLS.with(|pools| pools.borrow().clone());
    let pool_snapshots = POOLS_SNAPSHOTS.with(|snapshots| snapshots.borrow().clone());
    let event_logs = EVENT_LOGS.with(|logs| logs.borrow().clone());
    let state = StableState { pools, pool_snapshots, event_logs };

    storage::stable_save((state,)).expect("failed to save stable state");
}

pub fn stable_restore() {
    let (state,): (StableState,) = storage::stable_restore().expect("failed to restore stable state");

    POOLS.with(|pools| {
        pools.borrow_mut();
        pools.replace(state.pools.clone())
    });

    POOLS_SNAPSHOTS.with(|snapshots| {
        snapshots.borrow_mut();
        snapshots.replace(state.pool_snapshots.clone())
    });

    EVENT_LOGS.with(|event_logs| {
        event_logs.borrow_mut();
        event_logs.replace(state.event_logs)
    });
}
