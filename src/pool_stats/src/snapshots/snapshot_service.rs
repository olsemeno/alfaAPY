use std::cell::RefCell;
use std::time::Duration;
use ic_cdk_timers::TimerId;

use crate::pools::pool::Pool;
use crate::pools::pool_snapshot::PoolSnapshot;
use crate::repository::pools_repo;
use crate::pools::pool_data_service::{get_pool_data, get_position_data};

thread_local! {
    static POOL_SNAPSHOT_TIMER_ID: RefCell<Option<TimerId>> = RefCell::new(None);
}

fn set_timer_interval(
    interval: Duration,
    func: impl FnMut() + 'static,
) -> TimerId {
    ic_cdk_timers::set_timer_interval(interval, func)
}

pub fn start_pool_snapshots_timer(interval: u64) {
    let timer_id = set_timer_interval(Duration::from_secs(interval), || {
        ic_cdk::spawn(async {
            take_pool_snapshots().await;
        });
    });

    POOL_SNAPSHOT_TIMER_ID.with(|cell| {
        cell.replace(Some(timer_id));
    });
}

pub fn stop_pool_snapshots_timer() {
    POOL_SNAPSHOT_TIMER_ID.with(|timer_id| {
        if let Some(timer_id) = timer_id.borrow_mut().take() {
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}

pub async fn take_pool_snapshots() {
    let pools = pools_repo::get_pools();
    // Iterate over pools with liquidity position
    for pool in pools.into_iter().filter(|p| p.position_id.is_some()) {
        take_pool_snapshot(&pool).await;
    }
}

pub async fn take_pool_snapshot(pool: &Pool) -> PoolSnapshot {
    let pool_data = get_pool_data(pool).await;
    let position_data = get_position_data(pool).await;
    let snapshot = PoolSnapshot::build(pool.id.clone(), position_data, pool_data);
    snapshot.save();
    snapshot
}
