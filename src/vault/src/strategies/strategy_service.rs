use crate::repo::repo::{add_or_update_strategy, STRATEGIES};
use crate::strategies::r#impl::ck_btc_strategy::ckBTCStrategy;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use crate::strategies::strategy_candid::StrategyCandid;

pub fn init_strategies() {
    let ckBTC = Box::new(ckBTCStrategy::new());
    let icp = Box::new(ICPStrategy::new());
    add_or_update_strategy(ckBTC);
    add_or_update_strategy(icp);
}

pub fn get_actual_strategies() -> Vec<StrategyCandid> {
    let strategies: Vec<StrategyCandid> = STRATEGIES.with(|trss| {
        let a = trss.borrow();
        a.iter()
            .map(|a| a.to_candid())
            .collect()
    });
    strategies
}