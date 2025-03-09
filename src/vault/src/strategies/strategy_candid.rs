use candid::{CandidType, Deserialize};
use serde::Serialize;
use crate::strategies::r#impl::ck_btc_strategy::ckBTCStrategy;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use crate::strategies::strategy::IStrategy;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum StrategyCandid {
    ckBTCStrategyV(ckBTCStrategy),
    ICPStrategyV(ICPStrategy),
}


pub trait Candid {
    fn to_strategy(&self) -> Box<dyn IStrategy>;
}

//TODO maybe move to/from candid object + builders
impl Candid for StrategyCandid {
     fn to_strategy(&self) -> Box<dyn IStrategy> {
        match self {
            StrategyCandid::ckBTCStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::ICPStrategyV(strategy) => Box::new(strategy.clone()),
        }
    }
}