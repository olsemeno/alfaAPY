use std::cell::RefCell;
use std::collections::HashMap;

use types::pool_stats::PoolByTokens;

use crate::pools::pool::Pool;
use crate::pools::pool_snapshot::PoolSnapshot;


thread_local! {
    pub static POOLS: RefCell<HashMap<String, Pool>> = RefCell::new(HashMap::new());
    pub static POOLS_SNAPSHOTS: RefCell<HashMap<String, Vec<PoolSnapshot>>> = RefCell::new(HashMap::new());
}

// Pools

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
        let direct = pool.token0.symbol == pool_by_tokens.token0.symbol
            && pool.token1.symbol == pool_by_tokens.token1.symbol;
        let reverse = pool.token0.symbol == pool_by_tokens.token1.symbol
            && pool.token1.symbol == pool_by_tokens.token0.symbol;

        (direct || reverse) && pool.provider == pool_by_tokens.provider
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

// TODO: remove test method
pub fn save_pool_snapshot(snapshot: PoolSnapshot) {
    POOLS_SNAPSHOTS.with(|snapshots| {
        let mut snapshots = snapshots.borrow_mut();
        snapshots.insert(snapshot.pool_id.clone(), vec![snapshot]);
    });
}
