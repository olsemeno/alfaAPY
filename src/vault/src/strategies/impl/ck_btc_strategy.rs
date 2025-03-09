use crate::strategies::strategy::{IStrategy, Pool, StrategyId};
use async_trait::async_trait;
use candid::{CandidType, Deserialize};
use ic_cdk::trap;
use ic_ledger_types::Subaccount;
use kongswap_canister::PoolReply;
use serde::Serialize;
use crate::strategies::strategy_candid::StrategyCandid;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ckBTCStrategy {
    current_pool: Option<PoolReply>,
}

impl ckBTCStrategy {
    pub fn new() -> Self {
        ckBTCStrategy {
            current_pool: None
        }
    }
}

#[async_trait]
impl IStrategy for ckBTCStrategy {
    fn get_name(&self) -> String {
        "ckBTC to the moon".to_string()
    }

    fn get_id(&self) -> StrategyId {
        1
    }

    fn get_description(&self) -> String {
        "Half ckBTC, half something else".to_string()
    }

    fn get_pools(&self) -> Vec<Pool> {
        let ckBTC_ICP = {
            Pool {
                pool_symbol: "ckBTC_ICP".to_string(),
                token0: "ckBTC".to_string(),
                token1: "ICP".to_string(),
            }
        };
        let ckBTC_ckUSDT = {
            Pool {
                pool_symbol: "ckBTC_ckUSDT".to_string(),
                token0: "ckBTC".to_string(),
                token1: "ckUSDT".to_string(),
            }
        };
        vec![ckBTC_ICP, ckBTC_ckUSDT]
    }

    fn get_subaccount(&self) -> Subaccount {
        Subaccount([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
    }

    fn get_current_pool(&self) -> PoolReply {
        match self.current_pool.clone() {
            None => {
                trap("No current pool set");
            }
            Some(x) => {
                x.clone()
            }
        }
    }

    fn clone_self(&self) -> Box<dyn IStrategy> {
        Box::new(self.clone())
    }

    async fn rebalance(&self) -> PoolReply {
        trap("Not implemented yet");
    }

    fn to_candid(&self) -> StrategyCandid {
        StrategyCandid::ckBTCStrategyV(self.clone())
    }
}