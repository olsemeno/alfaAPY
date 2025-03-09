use crate::strategies::strategy::{IStrategy, Pool, StrategyId};
use async_trait::async_trait;
use candid::{CandidType, Deserialize};
use ic_cdk::trap;
use ic_ledger_types::Subaccount;
use kongswap_canister::PoolReply;
use serde::Serialize;
use crate::strategies::strategy_candid::StrategyCandid;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ICPStrategy {
    current_pool: Option<PoolReply>,
}

impl ICPStrategy {
    pub fn new() -> Self {
        ICPStrategy {
            current_pool: None
        }
    }
}


#[async_trait]
impl IStrategy for ICPStrategy {
    fn get_name(&self) -> String {
        "ICP stable as possible".to_string()
    }

    fn get_id(&self) -> StrategyId {
        2
    }

    fn get_description(&self) -> String {
        "Half ICP, half stable coin".to_string()
    }

    fn get_subaccount(&self) -> Subaccount {
        Subaccount([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2])
    }

    fn get_current_pool(&self) -> PoolReply {
        match self.current_pool.clone() {
            Some(pool) => pool,
            None => trap("No current pool"),
        }
    }

    fn clone_self(&self) -> Box<dyn IStrategy> {
        Box::new(self.clone())
    }

    fn get_pools(&self) -> Vec<Pool> {
        let ckUSDC_ICP = {
            Pool {
                pool_symbol: "ckUSDC_ICP".to_string(),
                token0: "ckUSDC".to_string(),
                token1: "ICP".to_string(),
            }
        };
        let ICP_ckUSDT = {
            Pool {
                pool_symbol: "ICP_ckUSDT".to_string(),
                token0: "ICP".to_string(),
                token1: "ckUSDT".to_string(),
            }
        };
        vec![ckUSDC_ICP, ICP_ckUSDT]
    }

    async fn rebalance(&self) -> PoolReply {
        trap("Not implemented yet");
    }

    fn to_candid(&self) -> StrategyCandid {
        StrategyCandid::ICPStrategyV(self.clone())
    }
}