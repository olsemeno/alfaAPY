use crate::providers::kong::kong::{remove_liquidity, user_balances};
use crate::strategies::strategy::{
    DepositResponse,
    IStrategy,
    Pool,
    PoolSymbol,
    StrategyId,
    StrategyResponse,
    WithdrawResponse,
    WithdrawFromPoolResponse,
    AddLiquidityResponse,
    RebalanceResponse,
    TokensInfo,
};
use crate::strategies::strategy_candid::StrategyCandid;
use crate::swap::swap_service::swap_icrc2_kong;
use async_trait::async_trait;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::trap;
use ic_ledger_types::Subaccount;
use kongswap_canister::PoolReply;
use serde::Serialize;
use std::collections::HashMap;
use types::exchanges::TokenInfo;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ckBTCStrategy {
    current_pool: Option<PoolReply>,
    total_balance: Nat,
    total_shares: Nat,
    user_shares: HashMap<Principal, Nat>,
}

impl ckBTCStrategy {
    pub fn new() -> Self {
        ckBTCStrategy {
            current_pool: None,
            total_balance: Nat::from(0u64),
            total_shares: Nat::from(0u64),
            user_shares: HashMap::new(),
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
                rolling_24h_apy: 0.0,
            }
        };
        let ckBTC_ckUSDT = {
            Pool {
                pool_symbol: "ckBTC_ckUSDT".to_string(),
                token0: "ckBTC".to_string(),
                token1: "ckUSDT".to_string(),
                rolling_24h_apy: 0.0,
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

    fn get_pool_tokens_info(&self, pool: PoolReply) -> TokensInfo {
        trap("Not implemented yet");
    }

    fn to_candid(&self) -> StrategyCandid {
        StrategyCandid::ckBTCStrategyV(self.clone())
    }

    fn get_user_shares(&self) -> HashMap<Principal, Nat> {
        self.user_shares.clone()
    }

    fn get_total_shares(&self) -> Nat {
        self.total_shares.clone()
    }

    async fn rebalance(&mut self) -> RebalanceResponse {
        trap("Not implemented yet");
    }

    async fn deposit(&mut self, investor: Principal, amount: Nat) -> DepositResponse {
        trap("Not implemented yet");
    }

    async fn withdraw(&mut self, investor: Principal, shares: Nat) -> WithdrawResponse {
        trap("Not implemented yet");
    }

    async fn withdraw_from_pool(&mut self, shares: Nat, pool: PoolReply) -> WithdrawFromPoolResponse {
        trap("Not implemented yet");
    }

    async fn add_liquidity_to_pool(&mut self, amount: Nat, pool: PoolReply) -> AddLiquidityResponse {
        trap("Not implemented yet");
    }
}
