use std::collections::HashMap;
use crate::strategies::strategy::{DepositResponse, IStrategy, Pool, PoolSymbol, StrategyId, StrategyResponse, WithdrawResponse};
use async_trait::async_trait;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::trap;
use ic_ledger_types::Subaccount;
use kongswap_canister::PoolReply;
use serde::Serialize;
use types::exchanges::TokenInfo;
use types::swap_tokens::SuccessResult;
use crate::providers::kong::kong::add_liquidity;
use crate::strategies::calculator::Calculator;
use crate::strategies::strategy_candid::StrategyCandid;
use crate::swap::swap_service::swap_icrc2_kong;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ICPStrategy {
    current_pool: Option<PoolReply>,
    total_balance: Nat,
    total_shares: Nat,
    user_shares: HashMap<Principal, Nat>,
    allocations: HashMap<PoolSymbol, Nat>,
}

impl ICPStrategy {
    pub fn new() -> Self {
        ICPStrategy {
            current_pool: None,
            total_balance: Nat::from(0u64),
            total_shares: Nat::from(0u64),
            user_shares: HashMap::new(),
            allocations: HashMap::new(),
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

    fn to_response(&self) -> StrategyResponse {
        StrategyResponse {
            name: self.get_name(),
            id: self.get_id(),
            description: self.get_description(),
            pools: self.get_pools().iter().map(|x| x.pool_symbol.clone()).collect(),
        }
    }


    async fn deposit(&mut self, investor: Principal, amount: Nat) -> DepositResponse {
        // accept_deposit(investor, amount, self.get_subaccount());

        let new_shares = Calculator::calculate_shares(amount.clone(), self.total_balance.clone(), self.total_shares.clone());

        self.total_balance += amount.clone();
        self.total_shares += new_shares.clone();
        self.user_shares.insert(investor, new_shares.clone());

        if let Some(ref pool_reply) = self.current_pool {

            // Расчитываем сколько нужно для свапа и для пула
            let response   = Calculator::calculate_pool_liquidity_amounts(amount.clone(), Pool {
                token0: pool_reply.symbol_0.clone(),
                token1: pool_reply.symbol_1.clone(),
                pool_symbol: pool_reply.symbol.clone(),
            }).await;

            let token_0_for_swap = response.token_0_for_swap;
            let token_0_for_pool = response.token_0_for_pool;
            let  token_1_for_pool = response.token_1_for_pool;

            let token_info_0 = TokenInfo {
                ledger: Principal::from_text(pool_reply.address_0.clone()).unwrap(),
                symbol: pool_reply.symbol_0.clone(),
            };

            let token_info_1 = TokenInfo {
                ledger: Principal::from_text(pool_reply.address_1.clone()).unwrap(),
                symbol: pool_reply.symbol_1.clone(),
            };
            // Свап
           swap_icrc2_kong(token_info_0, token_info_1, token_0_for_swap.0.trailing_ones() as u128).await;

            // Добавляем ликвидность
             add_liquidity(pool_reply.symbol_0.clone(), token_0_for_pool, pool_reply.symbol_1.clone(), token_1_for_pool).await;


            // Добавляем в allocations
            self.allocations.insert(pool_reply.symbol.clone(), amount.clone());
        } else {
            // rebalance();
        }

        DepositResponse {
            amount: amount,
            shares: new_shares,
        }
    }

    fn withdraw(&self, investor: Principal, shares: Nat) -> WithdrawResponse {
        trap("Not implemented yet");
    }
}