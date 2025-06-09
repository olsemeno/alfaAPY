use std::cell::RefCell;
use std::collections::HashMap;
use candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_cdk::storage;

use types::pool_stats::PoolByTokens;

use crate::pools::pool::Pool;
use crate::pools::pool_snapshot::PoolSnapshot;


#[derive(Serialize, Deserialize, CandidType)]
pub struct StableState {
    pub pools: HashMap<String, Pool>,
    pub pool_snapshots: HashMap<String, Vec<PoolSnapshot>>,
}

thread_local! {
    pub static POOLS: RefCell<HashMap<String, Pool>> = RefCell::new(HashMap::new());
    pub static POOLS_SNAPSHOTS: RefCell<HashMap<String, Vec<PoolSnapshot>>> = RefCell::new(HashMap::new());
}

// Pools

// TODO: test method, remove after testing
pub fn delete_all_pools_and_snapshots() {
    POOLS.with(|pools| pools.borrow_mut().clear());
    POOLS_SNAPSHOTS.with(|snapshots| snapshots.borrow_mut().clear());
}

pub fn save_pool(pool: Pool) {
    POOLS.with(|pools| pools.borrow_mut().insert(pool.id.clone(), pool));
}

pub fn delete_pool(pool_id: String) {
    POOLS.with(|pools| pools.borrow_mut().remove(&pool_id));
}

pub fn get_pools() -> Vec<Pool> {
    POOLS.with(|pools| pools.borrow().values().cloned().collect())
}

pub fn get_pool_by_tokens(pool_by_tokens: PoolByTokens) -> Option<Pool> {
    POOLS.with(|pools| pools.borrow().values().find(|pool| {
        let direct_match = pool.token0 == pool_by_tokens.token0
            && pool.token1 == pool_by_tokens.token1;
        let reverse_match = pool.token0 == pool_by_tokens.token1
            && pool.token1 == pool_by_tokens.token0;

        (direct_match || reverse_match) && pool.provider == pool_by_tokens.provider
    }).cloned())
}

pub fn get_pool_by_id(pool_id: String) -> Option<Pool> {
    POOLS.with(|pools| pools.borrow().get(&pool_id).cloned())
}

pub fn update_pool(pool_id: String, pool: Pool) {
    POOLS.with(|pools| {
        let mut pools = pools.borrow_mut();
        pools.insert(pool_id.to_string(), pool);
    });
}

pub fn get_pool_count() -> u64 {
    POOLS.with(|pools| pools.borrow().len() as u64)
}

// Pool Snapshots

pub fn get_pool_snapshots(pool_id: String) -> Option<Vec<PoolSnapshot>> {
    POOLS_SNAPSHOTS.with(|snapshots| snapshots.borrow().get(&pool_id).cloned())
}

pub fn get_pool_snapshots_count(pool_id: String) -> u32 {
    POOLS_SNAPSHOTS.with(|snapshots| {
        snapshots.borrow().get(&pool_id).map(|snapshots| snapshots.len() as u32).unwrap_or(0)
    })
}

pub fn save_pool_snapshot(snapshot: PoolSnapshot) {
    POOLS_SNAPSHOTS.with(|snapshots| {
        let mut snapshots = snapshots.borrow_mut();
        if let Some(entry) = snapshots.get_mut(&snapshot.pool_id) {
            let index = entry.iter().position(|s| s.id == snapshot.id);
            if let Some(index) = index {
                entry[index] = snapshot;
            } else {
                entry.push(snapshot);
            }
        } else {
            snapshots.insert(snapshot.pool_id.clone(), vec![snapshot]);
        }
    });
}

// TODO: remove test method
pub fn delete_pool_snapshots(pool_id: String) {
    POOLS_SNAPSHOTS.with(|snapshots| {
        snapshots.borrow_mut().remove(&pool_id);
    });
}

// TODO: remove test method
pub fn delete_pool_snapshot(pool_id: String, snapshot_id: String) {
    POOLS_SNAPSHOTS.with(|snapshots| {
        let mut snapshots = snapshots.borrow_mut();
        snapshots.get_mut(&pool_id)
            .map(|snapshots| snapshots.retain(|snapshot| snapshot.id != snapshot_id));
    });
}

// Stable storage

pub fn stable_save() {
    let pools = POOLS.with(|pools| pools.borrow().clone());
    let pool_snapshots = POOLS_SNAPSHOTS.with(|snapshots| snapshots.borrow().clone());
    let state = StableState { pools, pool_snapshots };

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
        snapshots.replace(state.pool_snapshots)
    });
}
