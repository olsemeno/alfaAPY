use crate::pools::pool_snapshot::PoolSnapshot;
use crate::pools::pool_metrics::{PoolApy, ApyValue};
use utils::util::nat_to_u128;

const DAYS_PER_WEEK: u64 = 7;
const DAYS_PER_MONTH: u64 = 30;
const DAYS_PER_YEAR: u64 = 365;
const MILLISECONDS_PER_DAY: u64 = 1000 * 60 * 60 * 24;
const MILLISECONDS_PER_WEEK: u64 = MILLISECONDS_PER_DAY * DAYS_PER_WEEK;
const MILLISECONDS_PER_MONTH: u64 = MILLISECONDS_PER_DAY * DAYS_PER_MONTH;
const MILLISECONDS_PER_YEAR: u64 = MILLISECONDS_PER_DAY * DAYS_PER_YEAR;

fn calculate_apy_for_period<F>(snapshots: &[&PoolSnapshot], extract_metric: F) -> u128
where
    F: Fn(&PoolSnapshot) -> u128,
{
    if snapshots.len() < 2 { return 0; }

    let mut sorted_snapshots = snapshots.to_vec();
    sorted_snapshots.sort_by_key(|s| s.timestamp);

    let first_snapshot = sorted_snapshots.first().unwrap();
    let last_snapshot = sorted_snapshots.last().unwrap();
    let initial_value = extract_metric(first_snapshot);
    let final_value = extract_metric(last_snapshot);

    if initial_value == 0 { return 0; }

    let gain = final_value / initial_value - 1;
    let period_days = (last_snapshot.timestamp - first_snapshot.timestamp) as u128 / MILLISECONDS_PER_DAY as u128;

    if period_days <= 0 { return 0; }

    // Converts the period yield to annual percentage yield (APY) with compounding:
    // (1 + gain) ^ (365 / period_days) - 1 extrapolates the gain to a year, as if it repeated every such period
    // * 10000 converts the result to basis points (100% = 10000 basis points)
    ((1 + gain).pow(DAYS_PER_YEAR as u32 / period_days as u32) - 1) * 10000
}

fn calculate_tokens_apy(snapshots: &[&PoolSnapshot]) -> u128 {
    let apy0 = calculate_apy_for_period(
        snapshots,
        |snap| nat_to_u128(&snap.position_data.as_ref().unwrap().amount0)
    );
    let apy1 = calculate_apy_for_period(
        snapshots,
        |snap| nat_to_u128(&snap.position_data.as_ref().unwrap().amount1)
    );

    // If both tokens are present, take the average APY, if only one is present, take its value
    match (apy0 > 0, apy1 > 0) {
        (true, true) => (apy0 + apy1) / 2,
        (true, false) => apy0,
        (false, true) => apy1,
        (false, false) => 0,
    }
}

pub fn calculate_pool_apy(snapshots: &[PoolSnapshot], now: u64) -> PoolApy {
    let week_ago = now.saturating_sub(MILLISECONDS_PER_WEEK);
    let month_ago = now.saturating_sub(MILLISECONDS_PER_MONTH);
    let year_ago = now.saturating_sub(MILLISECONDS_PER_YEAR);

    let year_snapshots: Vec<&PoolSnapshot> = snapshots
        .iter()
        .filter(|s| s.timestamp >= year_ago && s.timestamp <= now)
        .collect();

    let month_snapshots: Vec<&PoolSnapshot> = year_snapshots
        .iter()
        .copied()
        .filter(|s| s.timestamp >= month_ago)
        .collect();

    let week_snapshots: Vec<&PoolSnapshot> = year_snapshots
        .iter()
        .copied()
        .filter(|s| s.timestamp >= week_ago)
        .collect();

    // APY по USD
    let usd_value = |snapshot: &PoolSnapshot| {
        nat_to_u128(&snapshot.position_data.as_ref().unwrap().usd_amount0) + nat_to_u128(&snapshot.position_data.as_ref().unwrap().usd_amount1)
    };

    PoolApy {
        week: ApyValue {
            tokens_apy: calculate_tokens_apy(&week_snapshots),
            usd_apy: calculate_apy_for_period(&week_snapshots, &usd_value),
        },
        month: ApyValue {
            tokens_apy: calculate_tokens_apy(&month_snapshots),
            usd_apy: calculate_apy_for_period(&month_snapshots, &usd_value),
        },
        year: ApyValue {
            tokens_apy: calculate_tokens_apy(&year_snapshots),
            usd_apy: calculate_apy_for_period(&year_snapshots, &usd_value),
        },
    }
}
