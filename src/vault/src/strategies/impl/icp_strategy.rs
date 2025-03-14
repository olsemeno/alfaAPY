use crate::liquidity::liquidity_service::get_pools_data;
use crate::providers::kong::kong::{add_liquidity, remove_liquidity, user_balances};
use crate::providers::kong::kong::{add_liquidity_amounts, swap_amounts};
use crate::strategies::calculator::Calculator;
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
use crate::swap::token_swaps::nat_to_u128;
use async_trait::async_trait;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::{caller, trap};
use ic_ledger_types::Subaccount;
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use icrc_ledger_types::icrc1::account::Account;
use kongswap_canister::user_balances::{Response, UserBalancesReply};
use kongswap_canister::PoolReply;
use serde::Serialize;
use std::collections::HashMap;
use std::future::Future;
use std::ops::{Div, Mul};
use types::exchanges::TokenInfo;
use crate::repo::repo::save_strategy;
use crate::user::user_service::accept_deposit;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ICPStrategy {
    current_pool: Option<PoolReply>,
    total_balance: Nat,
    total_shares: Nat,
    user_shares: HashMap<Principal, Nat>,

}

impl ICPStrategy {
    pub fn new() -> Self {
        ICPStrategy {
            current_pool: None,
            total_balance: Nat::from(0u64),
            total_shares: Nat::from(0u64),
            user_shares: HashMap::new(),
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

    fn get_current_pool(&self) -> Option<PoolReply> {
        self.current_pool.clone()
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
                rolling_24h_apy: 10.0,
            }
        };
        let ICP_ckUSDT = {
            Pool {
                pool_symbol: "ICP_ckUSDT".to_string(),
                token0: "ICP".to_string(),
                token1: "ckUSDT".to_string(),
                rolling_24h_apy: 20.0,
            }
        };
        vec![ckUSDC_ICP, ICP_ckUSDT]
    }

    fn get_pool_tokens_info(&self, pool: PoolReply) -> TokensInfo {
        let token_info_0 = TokenInfo {
            ledger: Principal::from_text(&pool.address_0).unwrap(),
            symbol: pool.symbol_0.clone(),
        };

        let token_info_1 = TokenInfo {
            ledger: Principal::from_text(&pool.address_1).unwrap(),
            symbol: pool.symbol_1.clone(),
        };

        TokensInfo {
            token_0: token_info_0,
            token_1: token_info_1,
        }
    }

    fn to_candid(&self) -> StrategyCandid {
        StrategyCandid::ICPStrategyV(self.clone())
    }

    fn get_user_shares(&self) -> HashMap<Principal, Nat> {
        self.user_shares.clone()
    }

    fn get_total_shares(&self) -> Nat {
        self.total_shares.clone()
    }

    async fn rebalance(&mut self) -> RebalanceResponse {
        let pools_data = get_pools_data(Vec::from(self.get_pools())).await;

        // TODO: remove this (added to setting current pool)
        if self.current_pool.is_none() {
            self.current_pool = pools_data.iter().find(|&x| x.symbol == "ckUSDC_ICP").cloned();
        }

        let mut max_apy = 0.0;
        let mut max_apy_pool = None;

        // Find pool with highest APY
        for pool in pools_data {
            if pool.rolling_24h_apy > max_apy {
                max_apy = pool.rolling_24h_apy;
                max_apy_pool = Some(pool);
            }
        }

        let current_pool = self.get_current_pool();

        if let Some(max_pool) = max_apy_pool.clone() {
            // If current pool is the same as max APY pool, return
            if let Some(current_pool) = &current_pool {
                if current_pool.symbol == max_pool.symbol {
                    return RebalanceResponse {
                        pool: current_pool.clone(),
                    };
                }

                // Remove liquidity from current pool
                let withdraw_response = self.withdraw_from_pool(self.total_shares.clone(), current_pool.clone()).await;

                let token_0_amount = withdraw_response.token_0_amount;
                let token_1_amount = withdraw_response.token_1_amount;

                let tokens_info = self.get_pool_tokens_info(current_pool.clone());

                // Swap withdrawed token_1 to token_0 (to base token)
                let swap_response = swap_icrc2_kong(
                    tokens_info.token_1,
                    tokens_info.token_0,
                    nat_to_u128(token_1_amount)
                ).await;

                // Calculate total token_0 to send in new pool after swap
                let token_0_to_pool_amount = token_0_amount + swap_response.amount_out;

                // Add liquidity to new pool
                let add_liquidity_response = self.add_liquidity_to_pool(
                    token_0_to_pool_amount,
                    max_apy_pool.clone().unwrap()
                ).await;

                // Update current pool
                self.current_pool = Some(max_apy_pool.clone().unwrap());

                RebalanceResponse {
                    pool: self.current_pool.clone().unwrap(),
                }
            } else {
                trap("No current pool");
            }
        } else {
            RebalanceResponse {
                pool: self.current_pool.clone().unwrap(),
            }
        }
    }

    async fn deposit(&mut self, investor: Principal, amount: Nat) -> DepositResponse {
        // accept_deposit(investor, amount, self.get_subaccount());

        // TODO: remove this (added to setting current pool)
        let pools_data = get_pools_data(Vec::from(self.get_pools())).await;

        if self.current_pool.is_none() {
            self.current_pool = pools_data.iter().find(|&x| x.symbol == "ckUSDC_ICP").cloned();
        }

        // Calculate new shares for investor's deposit
        let new_shares = Calculator::calculate_shares(nat_to_f64(&amount), nat_to_f64(&self.total_balance), nat_to_f64(&self.total_shares.clone()));

        // Update total balance and total shares
        self.total_balance += amount.clone();
        self.total_shares += Nat::from(new_shares as u128);
        self.user_shares.insert(investor,  Nat::from(new_shares as u128));

        if let Some(ref pool_reply) = self.current_pool {
            let resp = self.add_liquidity_to_pool(amount.clone(), pool_reply.clone()).await;

            save_strategy(self.clone_self());

            DepositResponse {
                amount: amount,
                shares: Nat::from(new_shares as u128),
                tx_id: 0,
                request_id: resp.request_id,
            }
        } else {
            trap("Rebalance");
            // rebalance();
            //TODO fix
            DepositResponse {
                amount: amount,
                shares:  Nat::from((new_shares as u128)),
                tx_id: 0,
                request_id: 0,
            }
        }
    }

    async fn withdraw(&mut self, investor: Principal, shares: Nat) -> WithdrawResponse {
        // Check if user has enough shares
        if let Some(user_shares) = self.user_shares.get(&investor) {
            if shares > *user_shares {
                trap("Not sufficient shares".into());
            }
        } else {
            trap("No shares found for this investor".into());
        }

        let pool = self.get_current_pool();

        if let Some(pool) = pool {
            let tokens_info = self.get_pool_tokens_info(pool.clone());

            // Remove liquidity from pool
            let withdraw_response = self.withdraw_from_pool(shares.clone(), pool).await;

            // Swap token_1 to token_0 (to base token)
            let swap_response = swap_icrc2_kong(
                tokens_info.token_1,
                tokens_info.token_0.clone(),
                nat_to_f64(&withdraw_response.token_1_amount) as u128
            ).await;

            // Calculate total token_0 to send after swap
            let amount_to_withdraw = withdraw_response.token_0_amount + swap_response.amount_out;

            let transfer_result = icrc1_transfer(
                tokens_info.token_0.ledger,
                &TransferArg {
                    from_subaccount: None,
                    to: Account {
                        owner: caller(),
                        subaccount: None,
                    },
                    fee: None,
                    created_at_time: None,
                    memo: None,
                    amount: amount_to_withdraw.clone(),
                }
            ).await;

            let tr_id = match transfer_result {
                Ok(Ok(x)) => x,
                Err(x) => {
                    trap(format!("Transfer error 1: {:?}", x.1).as_str());
                }
                Ok(Err(x)) => {
                    trap(format!("Transfer error 2: {:?}", x).as_str());
                }
            };

            // Update user shares
            let current_shares = self.user_shares.get(&investor).cloned().unwrap_or(Nat::from(0u64));
            let new_shares = current_shares.min(shares.clone());
            self.user_shares.insert(investor.clone(), new_shares.clone());

            // Update total shares
            self.total_shares = self.total_shares.clone().min(shares);

            save_strategy(self.clone_self());

            WithdrawResponse {
                amount: amount_to_withdraw,
                current_shares: new_shares,
            }
        } else {
            trap("No current pool");
        }
    }

    async fn withdraw_from_pool(&mut self, shares: Nat, pool: PoolReply) -> WithdrawFromPoolResponse {
        // trap("Not implemented yet");
        let canister_id = ic_cdk::id();

        // Fetch LP tokens amount in pool
        let user_balances_response = match user_balances(canister_id.to_string()).await.0 {
            Ok(reply) => reply,
            Err(err) => {
                trap(format!("Error user_balances_response: {}", err).as_str());
            }
        };

        // Get user balance in pool
        let user_balance_reply = user_balances_response.into_iter()
            .filter_map(|reply| match reply {
                UserBalancesReply::LP(lp) => Some(lp),
                _ => None,
            })
            .find(|balance| balance.symbol == pool.symbol)
            .unwrap_or_else(|| trap("Expected LP balance"));

        //LPReply { symbol: \"ICP_ckUSDT\", name: \"ICP_ckUSDT LP Token\", balance: 0.06884369, usd_balance: 0.336404,
        // chain_0: \"IC\", symbol_0: \"ICP\", address_0: \"ryjl3-tyaaa-aaaaa-aaaba-cai\", amount_0: 0.03121415, usd_amount_0: 0.168202, chain_1: \"IC\", symbol_1: \"ckUSDT\", address_1: \"cngnf-vqaaa-aaaar-qag4q-cai\",
        // amount_1: 0.168202, usd_amount_1: 0.168202, ts: 1741767423329139829 }
        let balance = user_balance_reply.balance;

        // Calculate how much LP tokens to withdraw
        // balance * shares / total_shares * 100000000
        let lp_tokens_to_withdraw: f64 = balance.mul(nat_to_f64(&shares)).div(nat_to_f64(&self.total_shares.clone())).mul( 100000000.0 );

        // trap(format!("balance: {}, shares: {}, total_shares: {}, lp_tokens_to_withdraw: {}", balance, shares, self.total_shares, lp_tokens_to_withdraw).as_str());

        // trap(format!("lp_tokens_to_withdraw: {}", lp_tokens_to_withdraw).as_str());
        // Remove liquidity from pool
        let remove_liquidity_response = match remove_liquidity(
            pool.symbol_0.clone(),
            pool.symbol_1.clone(),
            Nat::from(lp_tokens_to_withdraw.round() as u128),
        ).await {
            Ok(r) => {r}
            Err(e) => {
                trap(format!("Error: {} with balance {} and lp_tokens_to_withdraw {}", e, balance, Nat::from(lp_tokens_to_withdraw.round() as u128) ).as_str());
            }
        };

        // trap(format!("remove_liquidity_response: {:?}, {}", remove_liquidity_response,  Nat::from(lp_tokens_to_withdraw.round() as u128)).as_str(), );

        WithdrawFromPoolResponse {
            token_0_amount: remove_liquidity_response.amount_0,
            token_1_amount: remove_liquidity_response.amount_1,
        }
    }

    async fn add_liquidity_to_pool(&mut self, amount: Nat, pool: PoolReply) -> AddLiquidityResponse {
        let token_0 = pool.symbol_0.clone();
        let token_1 = pool.symbol_1.clone();
        let pool_symbol = pool.symbol.clone();
        let address_0 = pool.address_0.clone();
        let address_1 = pool.address_1.clone();
        let tokens_info = self.get_pool_tokens_info(pool);

        // Get amounts of token_0 and token1 to add to pool
        let add_liq_amounts_resp = match add_liquidity_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
            (Ok(x), ) => x,
            (Err(e), ) => trap(format!("Error for {} and {} and {}: {}", token_0, token_1, amount, e).as_str()),
        };

        //  AddLiquidityAmountsReply { symbol: "ICP_ckUSDT", chain_0: "IC", address_0: "ryjl3-tyaaa-aaaaa-aaaba-cai", symbol_0: "ICP",
        // amount_0: Nat(10000), fee_0: Nat(10000), chain_1: "IC", address_1: "cngnf-vqaaa-aaaar-qag4q-cai",
        // symbol_1: "ckUSDT", amount_1: Nat(537), fee_1: Nat(10000), add_lp_token_amount: Nat(22038) }

        // Get amounts of token_0 and token1 to swap
        let swap_amounts_resp = match swap_amounts(token_0.clone(), amount.clone(), token_1.clone()).await {
            (Ok(x), ) => x,
            (Err(e), ) => trap(format!("Error for {} and {} and {}: {}", token_0, token_1, amount, e).as_str()),
        };

        //  SwapAmountsReply { pay_chain: "IC", pay_symbol: "ICP", pay_address: "ryjl3-tyaaa-aaaaa-aaaba-cai",
        // pay_amount: Nat(1000000), receive_chain: "IC", receive_symbol: "ckUSDT", receive_address: "cngnf-vqaaa-aaaar-qag4q-cai",
        // receive_amount: Nat(43557), price: 4.3557, mid_price: 5.37189568, slippage: 18.92,
        // txs: [SwapAmountsTxReply { pool_symbol: "ICP_ckUSDT", pay_chain: "IC", pay_symbol: "ICP", pay_amount: Nat(1000000),
        // pay_address: "ryjl3-tyaaa-aaaaa-aaaba-cai", receive_chain: "IC", receive_symbol: "ckUSDT",
        // receive_address: "cngnf-vqaaa-aaaar-qag4q-cai", receive_amount: Nat(43557), price: 4.3557, lp_fee: Nat(161), gas_fee: Nat(10000) }]

        // trap(format!("swap_amounts_resp: {:?}", swap_amounts_resp).as_str());

        // Calculate pool ratio and swap price
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

        // Swap token0 for token1 to get token1 for pool
        // let res =  swap_icrc2_kong(token_info_0, token_info_1, token_0_for_swap as u128, swap_amounts_resp2.receive_amount).await;
        let swap_response = swap_icrc2_kong(
            tokens_info.token_0,
            tokens_info.token_1,
            token_0_for_swap as u128,
        ).await;

        // Add liquidity to pool with token0 and token1
        let response = add_liquidity(
            token_0,
            Nat::from(token_0_for_pool as u128),
            token_1,
            Nat::from(token_1_for_pool as u128),
            Principal::from_text(address_0).unwrap(),
            Principal::from_text(address_1).unwrap(),
        ).await;

        match response {
            Ok(r) => {
                AddLiquidityResponse {
                    token_0_amount: Nat::from(token_0_for_pool as u128),
                    token_1_amount: Nat::from(token_1_for_pool as u128),
                    request_id: r.request_id,

                }
            }
            Err(e) => {
                trap(format!("Error: {}", e).as_str());
            }
        }
    }
}

pub fn nat_to_f64(n: &Nat) -> f64 {
    let nat_str = n.0.to_str_radix(10); // Convert to string
    nat_str.parse::<f64>().unwrap_or(0.0) // Parse as f64
}
