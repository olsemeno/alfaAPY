use crate::types::types::Pool;
use lazy_static::lazy_static;
use std::collections::HashMap;

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
            name: "ICP Stability Strategy".to_string(),
            description: "A balanced strategy utilizing Kongswap with 50% ICP and 50% stable coin, featuring pool pairs like ckUSDC/ICP and ICP/ckUSDT.".to_string(),
            pools: vec![
                Pool {
                    pool_symbol: "ICP_ckUSDT".to_string(),
                    token0: "ICP".to_string(),
                    token1: "ckUSDT".to_string(),
                    rolling_24h_apy: 20.0,
                },
                Pool {
                    pool_symbol: "ckUSDC_ICP".to_string(),
                    token0: "ckUSDC".to_string(),
                    token1: "ICP".to_string(),
                    rolling_24h_apy: 10.0,
                }
            ],
        });
        m.insert(1, StrategyInfo {
            name: "ckBTC Growth Strategy".to_string(),
            description: "An aggressive strategy leveraging Kongswap with 50% ckBTC and 50% other assets, including pool pairs like ckBTC/ICP and ckBTC/ckUSDT.".to_string(),
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
                }
            ],
        });
        m.insert(3, StrategyInfo {
            name: "ICP-ckBTC Dynamic Strategy".to_string(),
            description: "A dynamic strategy that moves the ICP-ckBTC pool between Kongswap and ICPSwap to optimize returns.".to_string(),
            pools: vec![
                Pool {
                    pool_symbol: "ICP_ckBTC".to_string(),
                    token0: "ICP".to_string(),
                    token1: "ckBTC".to_string(),
                    rolling_24h_apy: 0.0,
                },
            ],
        });
        m.insert(4, StrategyInfo {
            name: "Panda-ICP Balanced Strategy".to_string(),
            description: "Cheap test strategy".to_string(),
            pools: vec![
                Pool {
                    pool_symbol: "PANDA_ICP".to_string(),
                    token0: "PANDA".to_string(),
                    token1: "ICP".to_string(),
                    rolling_24h_apy: 0.0,
                },
            ],
        });
        m
    };
}