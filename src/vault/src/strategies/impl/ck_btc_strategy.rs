use crate::providers::kong::kong::add_liquidity;
use crate::strategies::calculator::Calculator;
use crate::strategies::strategy::{DepositResponse, IStrategy, Pool, PoolSymbol, StrategyId, StrategyResponse, WithdrawResponse};
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
    allocations: HashMap<PoolSymbol, Nat>,
}

impl ckBTCStrategy {
    pub fn new() -> Self {
        ckBTCStrategy {
            current_pool: None,
            total_balance: Nat::from(0u64),
            total_shares: Nat::from(0u64),
            user_shares: HashMap::new(),
            allocations: HashMap::new(),
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

        trap("Not implemented yet");

        // let new_shares = Calculator::calculate_shares(amount.clone(), self.total_balance.clone(), self.total_shares.clone());
        //
        // self.total_balance += amount.clone();
        // self.total_shares += new_shares.clone();
        // self.user_shares.insert(investor, new_shares.clone());
        //
        // if let Some(ref pool_reply) = self.current_pool {
        //
        //     // Расчитываем сколько нужно для свапа и для пула
        //     let response   = Calculator::calculate_pool_liquidity_amounts(amount.clone(), Pool {
        //         token0: pool_reply.symbol_0.clone(),
        //         token1: pool_reply.symbol_1.clone(),
        //         pool_symbol: pool_reply.symbol.clone(),
        //     }).await;
        //
        //     let token_0_for_swap = response.token_0_for_swap;
        //     let token_0_for_pool = response.token_0_for_pool;
        //     let  token_1_for_pool = response.token_1_for_pool;
        //
        //     let token_info_0 = TokenInfo {
        //         ledger: Principal::from_text(pool_reply.address_0.clone()).unwrap(),
        //         symbol: pool_reply.symbol_0.clone(),
        //     };
        //
        //     let token_info_1 = TokenInfo {
        //         ledger: Principal::from_text(pool_reply.address_1.clone()).unwrap(),
        //         symbol: pool_reply.symbol_1.clone(),
        //     };
        //     // Свап
        //     swap_icrc2_kong(token_info_0, token_info_1, token_0_for_swap.0.trailing_ones() as u128).await;
        //
        //     // Добавляем ликвидность
        //     add_liquidity(pool_reply.symbol_0.clone(), token_0_for_pool, pool_reply.symbol_1.clone(), token_1_for_pool).await;
        //
        //     // Добавляем в allocations
        //     self.allocations.insert(pool_reply.symbol.clone(), amount.clone());
        // } else {
        //     // rebalance();
        // }
        //
        // DepositResponse {
        //     amount: amount,
        //     shares: new_shares,
        // }
    }


    fn withdraw(&self, investor: Principal, shares: Nat) -> WithdrawResponse {
        trap("Not implemented yet");
        // // Check if user has enough shares
        // if shares > self.user_shares[&investor] {
        //     return Err("Not sufficient shares".into());
        // }

        // // Fetch LP tokens amount in pool
        // let (lp_tokens_in_pool) = user_balances(investor.to_string()).await.unwrap();

        // // Calculate LP tokens to withdraw
        // let lp_tokens_to_withdraw = lp_tokens_in_pool * shares / self.total_shares;

        // // Remove liquidity from pool
        // let (amount_0, amount_1) = remove_liquidity(pool_id.token0, pool_id.token1, lp_tokens_to_withdraw).await.unwrap();

        // // Update user shares
        // self.user_shares.insert(investor.clone(), self.user_shares[&investor] - shares);

        // // Update total shares
        // self.total_shares -= shares;

        // // Swap token_1 to token_0 (base token)
        // let(after_swap_amount_0) = swap_icrc2_kong(pool_id.token1, pool_id.token0, amount_1).await.unwrap();

        // // Calculate total token_0 to send after swap
        // let amount_to_withdraw = amount_0 + after_swap_amount_0;

        // // Send token_0 to user
        // send_token_0_to_user(investor, amount_to_withdraw);

        // Ok(WithdrawResponse {
        //     amount: amount_to_withdraw
        // })
    }

    // fn rebalance(&self) -> PoolReply {
    //     // Находим пул, с наибольшим APY
    //     // Получаем колличество токенов после remove_liquidity - remove_liquidity_amounts()
    //     // Достаем ликвидность remove_liquidity()
    //     // Высчитываем сколько токенов нужно для свапа и для нового пула Calculator::calculate_pool_liquidity_amounts()
    //     // Свапаем на токены из пула в базовый токен swap_icrc2_kong()
    //     // Добавляем ликвидность в новый add_liquidity()
    //     // обновляем current_pool
    //     trap("Not implemented yet");
    // }
}

//common_amount

//shares (% - principal)

// fn invest(investor: Principal, amount: Nat) {
//     common_amount + shares
// }
