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

impl_strategy_methods!(IcsStrategy);
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct IcsStrategy {
    id: StrategyId,
    current_pool: Option<Pool>,
    position_id: Option<u64>,
    total_balance: Nat,
    total_shares: Nat,
    user_shares: HashMap<Principal, Nat>,
    initial_deposit: HashMap<Principal, Nat>,
}

impl IcsStrategy {
    pub fn new() -> Self {
        //TODO move to config
        IcsStrategy {
            current_pool: None,
            position_id: None,
            total_balance: Nat::from(0u64),
            total_shares: Nat::from(0u64),
            user_shares: HashMap::new(),
            initial_deposit: HashMap::new(),
            id: 5,
        }
    }
}

#[async_trait]
impl IStrategy for IcsStrategy {
    fn to_candid(&self) -> StrategyCandid {
        StrategyCandid::IcsStrategyV(self.clone())
    }

    fn clone_self(&self) -> Box<dyn IStrategy> {
        Box::new(self.clone())
    }
}
