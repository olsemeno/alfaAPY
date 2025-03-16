use crate::repo::repo::{add_if_not_exists, add_or_update_strategy, save_strategy, STRATEGIES};
use crate::strategies::r#impl::ck_btc_strategy::ckBTCStrategy;
use crate::strategies::r#impl::pasta_icp_stategy::PandaTestStrategy;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use crate::strategies::r#impl::icp_usdt_kong_icpswap_strategy::IcpCkUSDTStrategy;
use crate::types::types::StrategyResponse;

pub fn init_strategies() {

    let ck_btc = Box::new(ckBTCStrategy::new());
    let icp = Box::new(ICPStrategy::new());
    add_if_not_exists(ck_btc);
    add_if_not_exists(icp);
    add_or_update_strategy(Box::new(PandaTestStrategy::new()));
    add_or_update_strategy(Box::new(IcpCkUSDTStrategy::new()));
}
pub fn get_actual_strategies() -> Vec<StrategyResponse> {
    let strategies: Vec<StrategyResponse> = STRATEGIES.with(|trss| {
        let a = trss.borrow();
        a.iter()
            .map(|a| a.to_response())
            .collect()
    });
    strategies
}