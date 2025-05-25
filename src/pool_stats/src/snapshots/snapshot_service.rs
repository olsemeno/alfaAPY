use std::cell::RefCell;

use crate::pools::pool_snapshot::PoolSnapshot;
use crate::repository::pools_repo;

thread_local! {
    pub static HEARTBEAT: RefCell<u64> = RefCell::new(0);
}

#[ic_cdk::heartbeat]
async fn heartbeat() {
    HEARTBEAT.with(|heartbeat| {
        let count = heartbeat.borrow_mut().clone();
        // 1 hour
        if count % 3600 == 0 {
            take_snapshots_for_pools();
        }
        heartbeat.replace(count + 1)
    });
}

pub fn take_snapshots_for_pools() {
    let pools = pools_repo::get_pools();
    for pool in pools {
        let pool_current_data = pool.get_current_data();
        let current_lp_position = pool.get_current_lp_position().unwrap();

        // TODO: implement apy calculation
        let apy = 0.0;

        let snapshot = PoolSnapshot::new(
            pool.id, 
            ic_cdk::api::time(), 
            current_lp_position, 
            pool_current_data, 
            apy
        );
        pools_repo::save_pool_snapshot(snapshot);
    }
}
