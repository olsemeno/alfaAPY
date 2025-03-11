use std::collections::HashMap;
use candid::{CandidType, Deserialize, Nat, Principal};
use crate::strategies::strategy::{DepositResponse, IStrategy, Pool, PoolSymbol, StrategyId, StrategyResponse, WithdrawResponse};
use crate::providers::kong::kong::{add_liquidity_amounts, swap_amounts};
use async_trait::async_trait;
use ic_cdk::trap;
use ic_ledger_types::Subaccount;
use kongswap_canister::PoolReply;
use serde::Serialize;
use kongswap_canister::add_liquidity::Response;
use types::exchanges::TokenInfo;
use types::swap_tokens::SuccessResult;
use crate::liquidity::liquidity_service::get_pools_data;
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

        // Calculate new shares for investor's deposit
        let new_shares = Calculator::calculate_shares(amount.clone(), self.total_balance.clone(), self.total_shares.clone());

        // Update total balance and total shares
        self.total_balance += amount.clone();
        self.total_shares += new_shares.clone();
        self.user_shares.insert(investor, new_shares.clone());

        let pools_data = get_pools_data(Vec::from(self.get_pools())).await;
        self.current_pool = pools_data.iter().find(|&x| x.symbol == "ICP_ckUSDT").cloned();

        if let Some(ref pool_reply) = self.current_pool {
            let token_0 = pool_reply.symbol_0.clone();
            let token_1 =  pool_reply.symbol_1.clone();

            // Get amounts of token_0 and token1 to add to pool
            let add_liq_amounts_resp = match add_liquidity_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
                Ok(x) => {
                    x
                }
                Err(e) => {
                    trap( format!("Error for {} and {} and {}", token_1, token_1, amount).as_str())
                }
            };

            // Get amounts of token_0 and token1 to swap
            let swap_amounts_resp = match swap_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
                Ok(x) => {
                    x
                }
                Err(e) => {
                    trap(e.as_str())
                }
            };

            let pool_ratio = add_liq_amounts_resp.amount_1 / add_liq_amounts_resp.amount_0;
            let swap_price = swap_amounts_resp.price;

            // Calculate how much token_0 and token_1 to swap and add to pool
            let response = Calculator::calculate_pool_liquidity_amounts(
                amount.clone(),
                pool_ratio.clone(),
                swap_price.clone()
            );

            let token_0_for_swap = response.token_0_for_swap;
            let token_0_for_pool = response.token_0_for_pool;
            let token_1_for_pool = response.token_1_for_pool;

            let token_info_0 = TokenInfo {
                ledger: Principal::from_text(pool_reply.address_0.clone()).unwrap(),
                symbol: pool_reply.symbol_0.clone(),
            };

            let token_info_1 = TokenInfo {
                ledger: Principal::from_text(pool_reply.address_1.clone()).unwrap(),
                symbol: pool_reply.symbol_1.clone(),
            };

            // Swap token0 for token1 to get token1 for pool
            swap_icrc2_kong(token_info_0, token_info_1, token_0_for_swap.0.trailing_ones() as u128).await;

            // Add liquidity to pool with token0 and token1
            let response = add_liquidity(
                pool_reply.symbol_0.clone(),
                token_0_for_pool,
                pool_reply.symbol_1.clone(),
                token_1_for_pool
            ).await;

            match response {
                Ok(r) => {
                    //TODO save response
                    self.allocations.insert(pool_reply.symbol.clone(), amount.clone());

                    DepositResponse {
                        amount: amount.clone(),
                        shares: new_shares,
                        tx_id: r.tx_id,
                        request_id: r.request_id,
                    }
                }
                Err(e) => {
                    trap(format!("Error: {}", e).as_str());
                }
            }
        } else {
            // rebalance();
            //TODO fix
            DepositResponse {
                amount: amount,
                shares: new_shares,
                tx_id: 0,
                request_id: 0,
            }

        }


    }

    fn withdraw(&self, investor: Principal, shares: Nat) -> WithdrawResponse {
        trap("Not implemented yet");
    }
}