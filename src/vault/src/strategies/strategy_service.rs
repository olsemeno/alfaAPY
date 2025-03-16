use crate::repo::repo::{add_or_update_strategy, STRATEGIES};
use crate::strategies::r#impl::ck_btc_strategy::ckBTCStrategy;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use crate::types::types::StrategyResponse;

pub fn init_strategies() {

    let ck_btc = Box::new(ckBTCStrategy::new());
    let icp = Box::new(ICPStrategy::new());
    add_or_update_strategy(ck_btc);
    add_or_update_strategy(icp);
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

// pub async fn withdraw(strategy_id: u16, amount: Nat) -> WithdrawResponse {
//     STRATEGIES.with(|strategies| async {
//         let mut strategies = strategies.borrow_mut();
//         let index = strategies.iter().position(|s| s.get_id() == strategy_id);
//         if let Some(index) = index {
//             strategies[index]
//                 .withdraw(amount).await
//         } else { panic!("Strategy not found") }
//     }).await
// }