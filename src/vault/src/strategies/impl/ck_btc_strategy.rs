use candid::{CandidType, Deserialize, Nat, Principal};
use std::collections::HashMap;
use crate::strategies::calculator::Calculator;
use crate::strategies::strategy::{IStrategy, Pool, PoolSymbol, StrategyId, StrategyResponse, DepositResponse};
use crate::user::user_service::{accept_deposit};
use async_trait::async_trait;
use ic_cdk::trap;
use ic_ledger_types::Subaccount;
use kongswap_canister::PoolReply;
use serde::Serialize;
use crate::strategies::strategy_candid::StrategyCandid;

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

    fn deposit(&self, investor: Principal, amount: Nat) -> DepositResponse {
        // accept_deposit(investor, amount, self.get_subaccount());

        let new_shares = Calculator::calculate_shares(amount, self.total_balance, self.total_shares);

        self.total_balance += amount;
        self.total_shares += new_shares;
        self.user_shares.insert(investor, new_shares);

        if let Some(ref pool_id) = self.current_pool {

            // Расчитываем сколько нужно для свапа и для пула
            let(token_0_for_swap, token_0_for_pool, token_1_for_pool) = Calculator::calculate_pool_liquidity_amounts(amount, pool_id);

            // Свап
            swap_icrc2_kong(pool_id.token0, pool_id.token1, token_0_for_swap);

            // Добавляем ликвидность
            add_liquidity(pool_id.token0, token_0_for_pool, pool_id.token1, token_1_for_pool);

            // Добавляем в allocations
            self.allocations.insert(pool_id.pool_symbol.clone(), amount);

        } else {
            // rebalance();
        }

        DepositResponse {
            amount: amount,
            shares: new_shares,
        }
    }



    fn withdraw(&self, investor: Principal, shares: Nat) -> WithdrawResponse {
        // Проверяем, есть ли у пользователя достаточное количество долей
        let user_account = self.user_accounts.get_mut(&user).ok_or("Пользователь не найден")?;
        if shares > user_account.shares {
            return Err("У пользователя недостаточно долей для вывода".into());
        }

        // Получаем колличество токенов после remove_liquidity - remove_liquidity_amounts()
        // Достаем ликвидность remove_liquidity()
        // Высчитываем сколько токенов нужно для свапа
        // Свапаем на токены из пула в базовый токен swap_icrc2_kong()
        // Переводим токены на адрес пользователя
        // обновляем current_pool
        
    }

    fn remove_liquidity(&self, investor: Principal, shares: Nat) -> RemoveLiquidityResponse {
        // Находим пул, с наибольшим APY
        // Получаем колличество токенов после remove_liquidity - remove_liquidity_amounts()
        // Достаем ликвидность remove_liquidity()
        // Высчитываем сколько токенов нужно для свапа и для нового пула Calculator::calculate_pool_liquidity_amounts()
        // Свапаем на токены из пула в базовый токен swap_icrc2_kong()
        // Добавляем ликвидность в новый add_liquidity()
        // обновляем current_pool
    }
    }


    //common_amount

    //shares (% - principal)

    // fn invest(investor: Principal, amount: Nat) {
    //     common_amount + shares
    // }
}