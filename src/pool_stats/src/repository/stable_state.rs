use candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_cdk::storage;
use std::collections::HashMap;

use crate::event_records::event_record::EventRecord;

use crate::pools::pool::Pool;
use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::repository::pools_repo::POOLS;
use crate::repository::pools_repo::POOLS_SNAPSHOTS;
use crate::repository::event_records_repo::EVENT_RECORDS;
use crate::repository::runtime_config_repo::{self, RuntimeConfig};

#[derive(Serialize, Deserialize, CandidType)]
pub struct StableState {
    pub runtime_config: RuntimeConfig,
    pub pools: HashMap<String, Pool>,
    pub pool_snapshots: HashMap<String, Vec<PoolSnapshot>>,
    pub event_records: Vec<EventRecord>,
}

pub fn stable_save() {
    let runtime_config = runtime_config_repo::get_runtime_config();

    let pools = POOLS.with(|pools| {
        pools.borrow().clone()
    });

    let pool_snapshots = POOLS_SNAPSHOTS.with(|snapshots| {
        snapshots.borrow().clone()
    });

    let event_records = EVENT_RECORDS.with(|records| {
        records.borrow().clone()
    });

    let state = StableState { runtime_config, pools, pool_snapshots, event_records };

    storage::stable_save((state,)).expect("failed to save stable state");
}

pub fn stable_restore() {
    let (state,): (StableState,) = storage::stable_restore().expect("failed to restore stable state");

    runtime_config_repo::set_runtime_config(state.runtime_config.clone());

    POOLS.with(|pools| {
        pools.borrow_mut();
        pools.replace(state.pools.clone())
    });

    POOLS_SNAPSHOTS.with(|snapshots| {
        snapshots.borrow_mut();
        snapshots.replace(state.pool_snapshots.clone())
    });

    EVENT_RECORDS.with(|event_records| {
        event_records.borrow_mut();
        event_records.replace(state.event_records)
    });
}
