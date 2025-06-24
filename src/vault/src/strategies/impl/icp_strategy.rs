use crate::impl_strategy_methods;
use crate::strategies::basic_strategy::BasicStrategy;
use crate::strategies::strategy::IStrategy;
use crate::strategies::strategy_candid::StrategyCandid;
use crate::types::types::StrategyId;
use crate::pools::pool::Pool;
use async_trait::async_trait;
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use std::collections::HashMap;
use crate::strategies::r#impl::description::STRATEGY_MAP;

impl_strategy_methods!(ICPStrategy);
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ICPStrategy {
    id: StrategyId,
    current_pool: Option<Pool>,
    position_id: Option<u64>,
    total_balance: Nat,
    total_shares: Nat,
    user_shares: HashMap<Principal, Nat>,
    initial_deposit: HashMap<Principal, Nat>,
    current_liquidity: Option<Nat>,
    current_liquidity_updated_at: Option<u64>,
}

impl ICPStrategy {
    pub fn new() -> Self {
        //TODO move to config
        ICPStrategy {
            id: 2,
            current_pool: None,
            position_id: None,
            total_balance: Nat::from(0u64),
            total_shares: Nat::from(0u64),
            user_shares: HashMap::new(),
            initial_deposit: HashMap::new(),
            current_liquidity: None,
            current_liquidity_updated_at: None,
        }
    }
}

#[async_trait]
impl IStrategy for ICPStrategy {
    fn to_candid(&self) -> StrategyCandid {
        StrategyCandid::ICPStrategyV(self.clone())
    }


    fn clone_self(&self) -> Box<dyn IStrategy> {
        Box::new(self.clone())
    }
}
