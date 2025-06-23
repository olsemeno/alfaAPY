    fn calculate_apy<F>(&self, extract_value: F) -> f64
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

        let precision_multiplier: u64 = 100000;

        let growth_factor = nat_to_u128(&final_value) as f64 / nat_to_u128(&initial_value) as f64;
        
        let yield_value = if growth_factor >= 1.0 {
            // growth -> APY
            let apy = growth_factor.powf(365.0 / period_days) - 1.0;
            apy * 100.0
        } else {
            // decline -> APR
            let apr = (growth_factor - 1.0) * (365.0 / period_days);
            apr * 100.0
        };

        panic!("initial_value: {}, final_value: {}, period_days: {}, growth_factor: {}, yield: {}", initial_value, final_value, period_days, growth_factor, yield_value);

        yield_value
    } 