use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::types::types::Pool;


#[derive(Debug, Clone)]
pub struct StrategyInfo {
    pub name: String,
    pub description: String,
    pub pools: Vec<Pool>,
}


//TODO init from file
lazy_static! {
    pub static ref STRATEGY_MAP: HashMap<u16, StrategyInfo> = {
        let mut m = HashMap::new();
        m.insert(2, StrategyInfo {
            name: "ICP stable as possible".to_string(),
            description: "Half ICP, half stable coin".to_string(),
        pools: vec![
            Pool {
                pool_symbol: "ckUSDC_ICP".to_string(),
                token0: "ckUSDC".to_string(),
                token1: "ICP".to_string(),
                rolling_24h_apy: 10.0,
            },
            Pool {
                pool_symbol: "ICP_ckUSDT".to_string(),
                token0: "ICP".to_string(),
                token1: "ckUSDT".to_string(),
                rolling_24h_apy: 20.0,
            }
            ],});
        m.insert(1, StrategyInfo {
            name: "ckBTC to the moon".to_string(),
            description: "Half ckBTC, half something else".to_string(),
        pools: vec![
            Pool {
                pool_symbol: "ckBTC_ICP".to_string(),
                token0: "ckBTC".to_string(),
                token1: "ICP".to_string(),
                rolling_24h_apy: 0.0,
            },
            Pool {
                pool_symbol: "ckBTC_ckUSDT".to_string(),
                token0: "ckBTC".to_string(),
                token1: "ckUSDT".to_string(),
                rolling_24h_apy: 0.0,
            },
            ],});
        m
    };
}