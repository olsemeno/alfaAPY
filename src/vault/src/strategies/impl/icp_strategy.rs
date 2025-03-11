use crate::liquidity::liquidity_service::get_pools_data;
use crate::providers::kong::kong::add_liquidity;
use crate::providers::kong::kong::{add_liquidity_amounts, swap_amounts};
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
        let new_shares = Calculator::calculate_shares(nat_to_f64(&amount), nat_to_f64(&self.total_balance), nat_to_f64(&self.total_shares.clone()));

        // Update total balance and total shares
        self.total_balance += amount.clone();
        self.total_shares +=  f64_to_nat(&new_shares);
        self.user_shares.insert(investor, f64_to_nat(&new_shares));

        let pools_data = get_pools_data(Vec::from(self.get_pools())).await;
        self.current_pool = pools_data.iter().find(|&x| x.symbol == "ICP_ckUSDT").cloned();

        if let Some(ref pool_reply) = self.current_pool {
            let token_0 = pool_reply.symbol_0.clone();
            let token_1 = pool_reply.symbol_1.clone();

            // Get amounts of token_0 and token1 to add to pool
            let add_liq_amounts_resp = match add_liquidity_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
                (Ok(x),) => x,
                (Err(e),) => trap(format!("Error for {} and {} and {}: {}", token_0, token_1, amount, e).as_str()),
            };
//  AddLiquidityAmountsReply { symbol: "ICP_ckUSDT", chain_0: "IC", address_0: "ryjl3-tyaaa-aaaaa-aaaba-cai", symbol_0: "ICP",
            // amount_0: Nat(10000), fee_0: Nat(10000), chain_1: "IC", address_1: "cngnf-vqaaa-aaaar-qag4q-cai",
            // symbol_1: "ckUSDT", amount_1: Nat(537), fee_1: Nat(10000), add_lp_token_amount: Nat(22038) }
            // Get amounts of token_0 and token1 to swap

            let swap_amounts_resp = match swap_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
                (Ok(x),) => x,
                (Err(e),) => trap(format!("Error for {} and {} and {}: {}", token_0, token_1, amount, e).as_str()),
            };

            //  SwapAmountsReply { pay_chain: "IC", pay_symbol: "ICP", pay_address: "ryjl3-tyaaa-aaaaa-aaaba-cai",
            // pay_amount: Nat(1000000), receive_chain: "IC", receive_symbol: "ckUSDT", receive_address: "cngnf-vqaaa-aaaar-qag4q-cai",
            // receive_amount: Nat(43557), price: 4.3557, mid_price: 5.37189568, slippage: 18.92,
            // txs: [SwapAmountsTxReply { pool_symbol: "ICP_ckUSDT", pay_chain: "IC", pay_symbol: "ICP", pay_amount: Nat(1000000),
            // pay_address: "ryjl3-tyaaa-aaaaa-aaaba-cai", receive_chain: "IC", receive_symbol: "ckUSDT",
            // receive_address: "cngnf-vqaaa-aaaar-qag4q-cai", receive_amount: Nat(43557), price: 4.3557, lp_fee: Nat(161), gas_fee: Nat(10000) }]

            // trap(format!("swap_amounts_resp: {:?}", swap_amounts_resp).as_str());


            let pool_ratio = nat_to_f64(&add_liq_amounts_resp.amount_1) / nat_to_f64(&add_liq_amounts_resp.amount_0);
            let swap_price = nat_to_f64(&swap_amounts_resp.receive_amount) / nat_to_f64(&swap_amounts_resp.pay_amount);
           //  100 ,0 ,0

            // trap(format!("pool_ratio: {}, swap_price: {}, ampunt: {}", pool_ratio, swap_price, nat_to_f64(&amount)).as_str());
            // Calculate how much token_0 and token_1 to swap and add to pool
            let calculator_response = Calculator::calculate_pool_liquidity_amounts(
                nat_to_f64(&amount),
                pool_ratio.clone(),
                swap_price.clone(),
            );

            let token_0_for_swap = calculator_response.token_0_for_swap;
            let token_0_for_pool = calculator_response.token_0_for_pool;
            let token_1_for_pool = calculator_response.token_1_for_pool;

            // trap(format!("token_0_for_swap: {}, token_0_for_pool: {}, token_1_for_pool: {}, amount: {}", token_0_for_swap, token_0_for_pool, token_1_for_pool, amount).as_str());

            let token_info_0 = TokenInfo {
                ledger: Principal::from_text(pool_reply.address_0.clone()).unwrap(),
                symbol: pool_reply.symbol_0.clone(),
            };

            let token_info_1 = TokenInfo {
                ledger: Principal::from_text(pool_reply.address_1.clone()).unwrap(),
                symbol: pool_reply.symbol_1.clone(),
            };

            // Swap token0 for token1 to get token1 for pool
           // let res =  swap_icrc2_kong(token_info_0, token_info_1, token_0_for_swap as u128, swap_amounts_resp2.receive_amount).await;

            // Add liquidity to pool with token0 and token1
            let response = add_liquidity(
                pool_reply.symbol_0.clone(),
                Nat::from(token_0_for_pool as u128),
                pool_reply.symbol_1.clone(),
                Nat::from(token_1_for_pool as u128),
                Principal::from_text(pool_reply.address_0.clone()).unwrap(),
                Principal::from_text(pool_reply.address_1.clone()).unwrap()
            ).await;

            match response {
                Ok(r) => {
                    //TODO save response
                    self.allocations.insert(pool_reply.symbol.clone(), amount.clone());

                    DepositResponse {
                        amount: amount.clone(),
                        shares: f64_to_nat(&new_shares),
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
                shares: f64_to_nat(&new_shares),
                tx_id: 0,
                request_id: 0,
            }
        }
    }

    fn withdraw(&self, investor: Principal, shares: Nat) -> WithdrawResponse {
        trap("Not implemented yet");
    }


}

pub fn nat_to_f64(n: &Nat) -> f64 {
    let nat_str = n.0.to_str_radix(10); // Convert to string
    nat_str.parse::<f64>().unwrap_or(0.0) // Parse as f64
}

pub fn f64_to_nat(f: &f64) -> Nat {
    Nat::from(f.to_bits())
}