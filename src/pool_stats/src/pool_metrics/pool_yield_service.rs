use candid::Nat;

use utils::util::nat_to_u128;

use crate::pool_snapshots::pool_snapshot::PoolSnapshot;
use crate::pool_metrics::pool_metrics::ApyValue;

const SECONDS_PER_DAY: u64 = 86_400;      // 60 * 60 * 24
const SECONDS_PER_WEEK: u64 = 604_800;    // 86_400 * 7
const SECONDS_PER_MONTH: u64 = 2_592_000; // 86_400 * 30
const SECONDS_PER_YEAR: u64 = 31_536_000; // 86_400 * 365

struct YieldCalculator<'a> {
    snapshots: &'a [&'a PoolSnapshot],
}

impl<'a> YieldCalculator<'a> {
    fn new(snapshots: &'a [&'a PoolSnapshot]) -> Self {
        Self { snapshots }
    }

    fn calculate_yield<F>(&self, extract_value: F) -> f64
    where
        F: Fn(&PoolSnapshot) -> Nat
    {
        if self.snapshots.len() < 2 {
            return 0.0;
        }

        let first_snapshot = self.snapshots.first().unwrap();
        let last_snapshot = self.snapshots.last().unwrap();

        let initial_value = extract_value(first_snapshot);
        let final_value = extract_value(last_snapshot);
        let period_days = (last_snapshot.timestamp - first_snapshot.timestamp) as f64 / SECONDS_PER_DAY as f64;

        if initial_value <= Nat::from(0u64) || period_days <= 0.0 {
            return 0.0;
        }

        let growth_factor = nat_to_u128(&final_value) as f64 / nat_to_u128(&initial_value) as f64;

        if growth_factor >= 1.0 {
            // growth -> APY
            let apy = growth_factor.powf(365.0 / period_days) - 1.0;
            return apy * 100.0;
        } else {
            // fall -> percent loss
            let percent_loss = (growth_factor - 1.0) * 100.0;
            return percent_loss;
        };

        //panic!("initial_value: {}, final_value: {}, period_days: {}, growth_factor: {}, result: {}", initial_value, final_value, period_days, growth_factor, result);

    }

    fn calculate_tokens_yield(&self) -> f64 {
        let apy_token0 = self.calculate_yield(
            |snapshot: &PoolSnapshot| {
                snapshot.position_data.as_ref().unwrap().amount0.clone()
            }
        );
        let apy_token1 = self.calculate_yield(
            |snapshot: &PoolSnapshot| {
                snapshot.position_data.as_ref().unwrap().amount1.clone()
            }
        );

        match (apy_token0 > 0.0, apy_token1 > 0.0) {
            (true, true) => (apy_token0 + apy_token1) / 2.0,  // average if both tokens are present
            (true, false) => apy_token0,                      // only first token
            (false, true) => apy_token1,                      // only second token
            (false, false) => 0.0,                            // no tokens
        }
    }

    fn calculate_usd_yield(&self) -> f64 {
        let extract_usd_value = |snapshot: &PoolSnapshot| {
            let position = snapshot.position_data.as_ref().unwrap();
            position.usd_amount0.clone() + position.usd_amount1.clone() // Value of tokens in pool in USD
        };

        self.calculate_yield(extract_usd_value)
    }
}

pub fn calculate_pool_yield(snapshots: &[PoolSnapshot], now: u64) -> ApyValue {
    // Get snapshots for different periods
    let get_period_snapshots = |from: u64| {
        snapshots.iter()
            .filter(|s| s.timestamp >= from && s.timestamp <= now)
            .collect::<Vec<_>>()
    };

    let full_range_snapshots = get_period_snapshots(0);

    // Use this when we have need APY for different periods
    // let year_snapshots = get_period_snapshots(now.saturating_sub(SECONDS_PER_YEAR));
    // let month_snapshots = get_period_snapshots(now.saturating_sub(SECONDS_PER_MONTH));
    // let week_snapshots = get_period_snapshots(now.saturating_sub(SECONDS_PER_WEEK));

    // Calculate APY for each period
    let calculate_apy_for_period = |snapshots: Vec<&PoolSnapshot>| {
        let calculator = YieldCalculator::new(&snapshots);

        calculator.calculate_usd_yield();

        ApyValue {
            tokens_apy: calculator.calculate_tokens_yield(),
            usd_apy: calculator.calculate_usd_yield(),
        }
    };

    calculate_apy_for_period(full_range_snapshots)
}
