use candid::Principal;

use crate::pools::pool::Pool;
use lazy_static::lazy_static;
use std::collections::HashMap;
use types::exchanges::TokenInfo;
use types::exchange_id::ExchangeId;

#[derive(Debug, Clone)]
pub struct StrategyInfo {
    pub name: String,
    pub description: String,
    pub pools: Vec<Pool>,
}

const ICP_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const CKBTC_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";
const CKUSDT_CANISTER_ID: &str = "cngnf-vqaaa-aaaar-qag4q-cai";
const PANDA_CANISTER_ID: &str = "druyg-tyaaa-aaaaq-aactq-cai";

//TODO init from file
lazy_static! {
    pub static ref STRATEGY_MAP: HashMap<u16, StrategyInfo> = {
        let mut m = HashMap::new();
        m.insert(2, StrategyInfo {
            name: "ICP Stability Strategy".to_string(),
            description: "A balanced strategy utilizing Kongswap with 50% ICP and 50% stable coin, featuring pool pairs like ckUSDC/ICP and ICP/ckUSDT.".to_string(),
            pools: vec![
                Pool {
                    token0: TokenInfo {
                        ledger: Principal::from_text(ICP_CANISTER_ID).unwrap(),
                        symbol: "ICP".to_string(),
                    },
                    token1: TokenInfo {
                        ledger: Principal::from_text(CKUSDT_CANISTER_ID).unwrap(),
                        symbol: "ckUSDT".to_string(),
                    },
                    provider: ExchangeId::KongSwap,
                },
                Pool {
                    token0: TokenInfo {
                        ledger: Principal::from_text(CKUSDT_CANISTER_ID).unwrap(),
                        symbol: "ckUSDT".to_string(),
                    },
                    token1: TokenInfo {
                        ledger: Principal::from_text(ICP_CANISTER_ID).unwrap(),
                        symbol: "ICP".to_string(),
                    },
                    provider: ExchangeId::KongSwap,
                }
            ],
        });
        m.insert(1, StrategyInfo {
            name: "ckBTC Growth Strategy".to_string(),
            description: "An aggressive strategy leveraging Kongswap with 50% ckBTC and 50% other assets, including pool pairs like ckBTC/ICP and ckBTC/ckUSDT.".to_string(),
            pools: vec![
                Pool {
                    token0: TokenInfo {
                        ledger: Principal::from_text(CKBTC_CANISTER_ID).unwrap(),
                        symbol: "ckBTC".to_string(),
                    },
                    token1: TokenInfo {
                        ledger: Principal::from_text(ICP_CANISTER_ID).unwrap(),
                        symbol: "ICP".to_string(),
                    },
                    provider: ExchangeId::KongSwap,
                },
                Pool {
                    token0: TokenInfo {
                        ledger: Principal::from_text(CKBTC_CANISTER_ID).unwrap(),
                        symbol: "ckBTC".to_string(),
                    },
                    token1: TokenInfo {
                        ledger: Principal::from_text(CKUSDT_CANISTER_ID).unwrap(),
                        symbol: "ckUSDT".to_string(),
                    },
                    provider: ExchangeId::KongSwap,
                }
            ],
        });
        m.insert(3, StrategyInfo {
            name: "ICP-ckBTC Dynamic Strategy".to_string(),
            description: "A dynamic strategy that moves the ICP-ckBTC pool between Kongswap and ICPSwap to optimize returns.".to_string(),
            pools: vec![
                Pool {
                    token0: TokenInfo {
                        ledger: Principal::from_text(ICP_CANISTER_ID).unwrap(),
                        symbol: "ICP".to_string(),
                    },
                    token1: TokenInfo {
                        ledger: Principal::from_text(CKBTC_CANISTER_ID).unwrap(),
                        symbol: "ckBTC".to_string(),
                    },
                    provider: ExchangeId::KongSwap,
                },
            ],
        });
        m.insert(4, StrategyInfo {
            name: "Panda-ICP Balanced Strategy".to_string(),
            description: "Cheap test strategy".to_string(),
            pools: vec![
                Pool {
                    token0: TokenInfo {
                        ledger: Principal::from_text(PANDA_CANISTER_ID).unwrap(),
                        symbol: "PANDA".to_string(),
                    },
                    token1: TokenInfo {
                        ledger: Principal::from_text(ICP_CANISTER_ID).unwrap(),
                        symbol: "ICP".to_string(),
                    },
                    provider: ExchangeId::KongSwap,
                },
            ],
        });
        m
    };
}