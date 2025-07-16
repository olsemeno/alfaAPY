use std::cell::RefCell;
use std::collections::HashMap;

use crate::pools::pool::Pool;
use crate::pool_snapshots::pool_snapshot::PoolSnapshot;

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

pub fn get_pool_by_id(pool_id: String) -> Option<Pool> {
    POOLS.with(|pools| pools.borrow().get(&pool_id).cloned())
}

pub fn update_pool(pool_id: String, pool: Pool) {
    POOLS.with(|pools| {
        let mut pools = pools.borrow_mut();
        pools.insert(pool_id.to_string(), pool);
    });
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

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;
    use types::exchange_id::ExchangeId;

    use crate::pools::pool::Pool;
    use crate::pool_snapshots::pool_snapshot::PoolSnapshot;

    fn dummy_pool(id: &str) -> Pool {
        Pool {
            id: id.to_string(),
            token0: Principal::from_text("aaaaa-aa").unwrap(),
            token1: Principal::from_text("aaaaa-aa").unwrap(),
            provider: ExchangeId::ICPSwap,
            position_id: None,
        }
    }

    fn dummy_snapshot(pool_id: &str, snapshot_id: &str) -> PoolSnapshot {
        PoolSnapshot {
            id: snapshot_id.to_string(),
            pool_id: pool_id.to_string(),
            timestamp: 123,
            position_data: None,
            pool_data: None,
        }
    }

    mod save_pool {
        use super::*;

        #[test]
        fn inserts_pool_into_store() {
            let pool = dummy_pool("test-pool");
            save_pool(pool.clone());

            let result = get_pool_by_id("test-pool".to_string());
            assert_eq!(result, Some(pool));
        }
    }

    mod delete_pool {
        use super::*;

        #[test]
        fn removes_pool_by_id() {
            let pool = dummy_pool("delete-pool");
            save_pool(pool.clone());

            delete_pool("delete-pool".to_string());
            assert_eq!(get_pool_by_id("delete-pool".to_string()), None);
        }
    }

    mod get_pools {
        use super::*;

        #[test]
        fn returns_all_pools() {
            delete_all_pools_and_snapshots();

            save_pool(dummy_pool("pool-1"));
            save_pool(dummy_pool("pool-2"));

            let pools = get_pools();
            assert_eq!(pools.len(), 2);
        }
    }

    mod get_pool_by_id {
        use super::*;

        #[test]
        fn returns_correct_pool_if_exists() {
            let pool = dummy_pool("find-me");
            save_pool(pool.clone());

            let found_pool = get_pool_by_id("find-me".to_string());
            assert_eq!(found_pool, Some(pool));
        }

        #[test]
        fn returns_none_if_pool_not_found() {
            assert_eq!(get_pool_by_id("non-existent".to_string()), None);
        }
    }

    mod update_pool {
        use super::*;

        #[test]
        fn replaces_existing_pool() {
            let mut pool = dummy_pool("update-me");
            save_pool(pool.clone());

            pool.position_id = Some(42);
            update_pool("update-me".to_string(), pool.clone());

            let updated_pool = get_pool_by_id("update-me".to_string());
            assert_eq!(updated_pool, Some(pool));
        }
    }

    mod save_pool_snapshot {
        use super::*;

        #[test]
        fn inserts_new_snapshot() {
            let snapshot = dummy_snapshot("snap-pool", "1");
            save_pool_snapshot(snapshot.clone());

            let snapshots = get_pool_snapshots("snap-pool".to_string()).unwrap();
            assert_eq!(snapshots, vec![snapshot.clone()]);
        }

        #[test]
        fn replaces_existing_snapshot_with_same_id() {
            let mut snapshot = dummy_snapshot("same-pool", "1");
            save_pool_snapshot(snapshot.clone());

            snapshot.timestamp = 999;
            save_pool_snapshot(snapshot.clone());

            let snapshots = get_pool_snapshots("same-pool".to_string()).unwrap();
            assert_eq!(snapshots.len(), 1);
            assert_eq!(snapshots[0].timestamp, 999);
        }
    }

    mod get_pool_snapshots {
        use super::*;

        #[test]
        fn returns_snapshots_for_pool() {
            delete_pool_snapshots("pool-x".to_string());

            save_pool_snapshot(dummy_snapshot("pool-x", "1"));
            save_pool_snapshot(dummy_snapshot("pool-x", "2"));

            let snapshots = get_pool_snapshots("pool-x".to_string()).unwrap();
            assert_eq!(snapshots.len(), 2);
        }

        #[test]
        fn returns_none_if_no_snapshots() {
            delete_pool_snapshots("no-snap".to_string());
            assert_eq!(get_pool_snapshots("no-snap".to_string()), None);
        }
    }

    mod get_pool_snapshots_count {
        use super::*;

        #[test]
        fn counts_snapshots_correctly() {
            delete_pool_snapshots("count-me".to_string());

            save_pool_snapshot(dummy_snapshot("count-me", "1"));
            save_pool_snapshot(dummy_snapshot("count-me", "2"));

            let count = get_pool_snapshots_count("count-me".to_string());
            assert_eq!(count, 2);
        }
    }

    mod delete_pool_snapshots {
        use super::*;

        #[test]
        fn removes_all_snapshots_for_pool() {
            save_pool_snapshot(dummy_snapshot("bulk-delete", "1"));
            save_pool_snapshot(dummy_snapshot("bulk-delete", "2"));

            delete_pool_snapshots("bulk-delete".to_string());
            assert_eq!(get_pool_snapshots("bulk-delete".to_string()), None);
        }
    }

    mod delete_pool_snapshot {
        use super::*;

        #[test]
        fn removes_single_snapshot_by_id() {
            delete_pool_snapshots("partial-delete".to_string());

            save_pool_snapshot(dummy_snapshot("partial-delete", "1"));
            save_pool_snapshot(dummy_snapshot("partial-delete", "2"));

            delete_pool_snapshot("partial-delete".to_string(), "1".to_string());

            let remaining = get_pool_snapshots("partial-delete".to_string()).unwrap();
            assert_eq!(remaining.len(), 1);
            assert_eq!(remaining[0].id, "2");
        }
    }

    mod delete_all_pools_and_snapshots {
        use super::*;

        #[test]
        fn clears_both_pools_and_snapshots() {
            save_pool(dummy_pool("wipe-me"));
            save_pool_snapshot(dummy_snapshot("wipe-me", "1"));

            delete_all_pools_and_snapshots();

            assert_eq!(get_pools().len(), 0);
            assert_eq!(get_pool_snapshots("wipe-me".to_string()), None);
        }
    }
}
