use lazy_static::lazy_static;
use std::collections::HashMap;

use types::exchange_id::ExchangeId;
use types::pool::PoolTrait;
use utils::constants::{
    ICP_TOKEN_CANISTER_ID,
    CKUSDT_TOKEN_CANISTER_ID,
    CKBTC_TOKEN_CANISTER_ID,
    PANDA_TOKEN_CANISTER_ID,
    ICS_TOKEN_CANISTER_ID,
};

use crate::pools::pool::Pool;

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
                Pool::build(
                    *ICP_TOKEN_CANISTER_ID,
                    *CKUSDT_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *CKUSDT_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
            ],
        });
        m.insert(1, StrategyInfo {
            name: "ckBTC Growth Strategy".to_string(),
            description: "An aggressive strategy leveraging Kongswap with 50% ckBTC and 50% other assets, including pool pairs like ckBTC/ICP and ckBTC/ckUSDT.".to_string(),
            pools: vec![
                Pool::build(
                    *CKBTC_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *CKBTC_TOKEN_CANISTER_ID,
                    *CKUSDT_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
            ],
        });
        m.insert(3, StrategyInfo {
            name: "ICP-ckBTC Dynamic Strategy".to_string(),
            description: "A dynamic strategy that moves the ICP-ckBTC pool between Kongswap and ICPSwap to optimize returns.".to_string(),
            pools: vec![
                Pool::build(
                    *ICP_TOKEN_CANISTER_ID,
                    *CKBTC_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *CKBTC_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
            ],
        });
        m.insert(4, StrategyInfo {
            name: "Panda-ICP Balanced Strategy".to_string(),
            description: "Cheap test strategy".to_string(),
            pools: vec![
                Pool::build(
                    *PANDA_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *PANDA_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });
        m.insert(5, StrategyInfo {
            name: "ICS-ICP Balanced Strategy".to_string(),
            description: "Cheap test strategy".to_string(),
            pools: vec![
                Pool::build(
                    *ICS_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::KongSwap,
                ),
                Pool::build(
                    *ICS_TOKEN_CANISTER_ID,
                    *ICP_TOKEN_CANISTER_ID,
                    ExchangeId::ICPSwap,
                ),
            ],
        });
        m
    };
}
