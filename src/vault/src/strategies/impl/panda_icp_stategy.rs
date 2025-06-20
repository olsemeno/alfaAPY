use crate::impl_strategy_methods;
use crate::strategies::basic_strategy::BasicStrategy;
use crate::strategies::r#impl::description::STRATEGY_MAP;
use crate::strategies::strategy::IStrategy;
use crate::strategies::strategy_candid::StrategyCandid;
use crate::types::types::StrategyId;
use crate::pools::pool::Pool;
use async_trait::async_trait;
use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use std::collections::HashMap;

impl_strategy_methods!(PandaTestStrategy);
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct PandaTestStrategy {
    id: StrategyId,
    current_pool: Option<Pool>,
    total_balance: Nat,
    total_shares: Nat,
    user_shares: HashMap<Principal, Nat>,
    initial_deposit: HashMap<Principal, Nat>,
}

impl PandaTestStrategy {
    pub fn new() -> Self {
        //TODO move to config
        PandaTestStrategy {
            current_pool: None,
            total_balance: Nat::from(0u64),
            total_shares: Nat::from(0u64),
            user_shares: HashMap::new(),
            initial_deposit: HashMap::new(),
            id: 4,
        }
    }
}

#[async_trait]
impl IStrategy for PandaTestStrategy {
    fn to_candid(&self) -> StrategyCandid {
        StrategyCandid::PandaTestStrategyV(self.clone())
    }

    fn clone_self(&self) -> Box<dyn IStrategy> {
        Box::new(self.clone())
    }
}
