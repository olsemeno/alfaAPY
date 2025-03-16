use candid::{CandidType, Deserialize};
use serde::Serialize;
use crate::strategies::r#impl::ck_btc_strategy::ckBTCStrategy;
use crate::strategies::r#impl::pasta_icp_stategy::PandaTestStrategy;
use crate::strategies::r#impl::icp_strategy::ICPStrategy;
use crate::strategies::r#impl::icp_usdt_kong_icpswap_strategy::IcpCkUSDTStrategy;
use crate::strategies::strategy::IStrategy;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum StrategyCandid {
    #[allow(non_camel_case_types)]
    ckBTCStrategyV(ckBTCStrategy),
    ICPStrategyV(ICPStrategy),
    #[allow(non_camel_case_types)]
    PandaTestStrategyV(PandaTestStrategy),
    IcpCkUSDTStrategyV(IcpCkUSDTStrategy),
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
            StrategyCandid::PandaTestStrategyV(strategy) => Box::new(strategy.clone()),
            StrategyCandid::IcpCkUSDTStrategyV(strategy) => Box::new(strategy.clone()),
        }
    }
}