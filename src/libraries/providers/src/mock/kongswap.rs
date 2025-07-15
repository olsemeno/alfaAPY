use std::collections::HashMap;
use candid::{Nat, Principal};
use types::CanisterId;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use candid::CandidType;

use kongswap_canister::add_liquidity::AddLiquidityReply;
use kongswap_canister::remove_liquidity::RemoveLiquidityReply;
use kongswap_canister::remove_liquidity_amounts::RemoveLiquidityAmountsReply;
use kongswap_canister::queries::pools::PoolReply;
use kongswap_canister::queries::add_liquidity_amounts::AddLiquidityAmountsReply;
use kongswap_canister::swap_amounts::SwapAmountsReply;
use kongswap_canister::user_balances::UserBalancesReply;
use kongswap_canister::swap::SwapReply;
use errors::internal_error::error::{InternalError, build_error_code};

use crate::kongswap::KongSwapProvider;

// Converts Option<f64> to String for use as a HashMap key since f64 doesn't implement Hash and Eq traits.
// We need this because floating point numbers can't be used directly as HashMap keys.
// The precision is fixed at 8 decimal places to ensure consistent string representation.
// Returns "none" for None values to maintain a unique string representation.
fn slippage_to_string(slippage: Option<f64>) -> String {
    slippage.map_or("none".to_string(), |v| format!("{:.8}", v))
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct MockKongSwapProvider {
    pub pools_response: Result<Vec<PoolReply>, InternalError>,
    pub swap_amounts_responses: HashMap<(String, String, String), Result<SwapAmountsReply, InternalError>>,
    pub swap_responses: HashMap<(String, String, String, String), Result<SwapReply, InternalError>>,
    pub add_liquidity_amounts_responses: HashMap<(String, String, String), Result<AddLiquidityAmountsReply, InternalError>>,
    pub add_liquidity_responses: HashMap<(String, String, String, String, String, String), Result<AddLiquidityReply, InternalError>>,
    pub user_balances_responses: HashMap<String, Result<Vec<UserBalancesReply>, InternalError>>,
    pub remove_liquidity_amounts_responses: HashMap<(String, String, String), Result<RemoveLiquidityAmountsReply, InternalError>>,
    pub remove_liquidity_responses: HashMap<(String, String, String), Result<RemoveLiquidityReply, InternalError>>,
}

impl Default for MockKongSwapProvider {
    fn default() -> Self {
        Self {
            pools_response: Err(InternalError::not_found(
                build_error_code(0000, 0, 0),
                "mock_error".to_string(),
                "Mock response not set for pools".to_string(),
                None
            )),
            swap_amounts_responses: HashMap::new(),
            swap_responses: HashMap::new(),
            add_liquidity_amounts_responses: HashMap::new(),
            add_liquidity_responses: HashMap::new(),
            user_balances_responses: HashMap::new(),
            remove_liquidity_amounts_responses: HashMap::new(),
            remove_liquidity_responses: HashMap::new(),
        }
    }
}

impl MockKongSwapProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mock_pools(&mut self, response: Result<Vec<PoolReply>, InternalError>) {
        self.pools_response = response;
    }

    pub fn mock_swap_amounts(
        &mut self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
        response: Result<SwapAmountsReply, InternalError>,
    ) {
        self.swap_amounts_responses.insert(
            (token_in.to_text(), amount.to_string(), token_out.to_text()),
            response
        );
    }

    pub fn mock_swap(
        &mut self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
        max_slippage: Option<f64>,
        response: Result<SwapReply, InternalError>,
    ) {
        self.swap_responses.insert(
            (
                token_in.to_text(),
                amount.to_string(),
                token_out.to_text(),
                slippage_to_string(max_slippage)
            ),
            response
        );
    }

    pub fn mock_add_liquidity_amounts(
        &mut self,
        token_0: String,
        amount: Nat,
        token_1: String,
        response: Result<AddLiquidityAmountsReply, InternalError>,
    ) {
        self.add_liquidity_amounts_responses.insert(
            (token_0, amount.to_string(), token_1),
            response
        );
    }

    pub fn mock_add_liquidity(
        &mut self,
        token_0: String,
        amount_0: Nat,
        token_1: String,
        amount_1: Nat,
        ledger0: Principal,
        ledger1: Principal,
        response: Result<AddLiquidityReply, InternalError>,
    ) {
        self.add_liquidity_responses.insert(
            (token_0, amount_0.to_string(), token_1, amount_1.to_string(), ledger0.to_text(), ledger1.to_text()),
            response
        );
    }

    pub fn mock_user_balances(
        &mut self,
        principal_id: String,
        response: Result<Vec<UserBalancesReply>, InternalError>,
    ) {
        self.user_balances_responses.insert(principal_id, response);
    }

    pub fn mock_remove_liquidity_amounts(
        &mut self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat,
        response: Result<RemoveLiquidityAmountsReply, InternalError>,
    ) {
        self.remove_liquidity_amounts_responses.insert(
            (token_0, token_1, remove_lp_token_amount.to_string()),
            response
        );
    }

    pub fn mock_remove_liquidity(
        &mut self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat,
        response: Result<RemoveLiquidityReply, InternalError>,
    ) {
        self.remove_liquidity_responses.insert(
            (token_0, token_1, remove_lp_token_amount.to_string()),
            response
        );
    }
}

#[async_trait]
impl KongSwapProvider for MockKongSwapProvider {
    async fn pools(&self) -> Result<Vec<PoolReply>, InternalError> {
        self.pools_response.clone()
    }

    async fn swap_amounts(
        &self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
    ) -> Result<SwapAmountsReply, InternalError> {
        self.swap_amounts_responses
            .get(&(token_in.to_text(), amount.to_string(), token_out.to_text()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(2301, 01, 01), // 2301 01 01
                    "MockKongSwapProvider::swap_amounts".to_string(),
                    "Mock response not set for swap_amounts".to_string(),
                    Some(HashMap::from([
                        ("token_in".to_string(), token_in.to_text()),
                        ("amount".to_string(), amount.to_string()),
                        ("token_out".to_string(), token_out.to_text()),
                    ]))
                )),
                |r| r.to_owned()
            )
    }

    async fn swap(
        &self,
        token_in: CanisterId,
        amount: Nat,
        token_out: CanisterId,
        max_slippage: Option<f64>,
    ) -> Result<SwapReply, InternalError> {
        let key = (
            token_in.to_text(),
            amount.to_string(),
            token_out.to_text(),
            slippage_to_string(max_slippage)
        );

        self.swap_responses
            .get(&key)
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(2301, 01, 02), // 2301 01 02
                    "MockKongSwapProvider::swap".to_string(),
                    "Mock response not set for swap".to_string(),
                    Some(HashMap::from([
                        ("token_in".to_string(), token_in.to_text()),
                        ("amount".to_string(), amount.to_string()),
                        ("token_out".to_string(), token_out.to_text()),
                        ("max_slippage".to_string(), slippage_to_string(max_slippage)),
                    ]))
                )),
                |r| r.to_owned()
            )
    }

    async fn add_liquidity_amounts(
        &self,
        token_0: String,
        amount: Nat,
        token_1: String,
    ) -> Result<AddLiquidityAmountsReply, InternalError> {
        self.add_liquidity_amounts_responses
            .get(&(token_0.clone(), amount.to_string(), token_1.clone()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(2301, 01, 03), // 2301 01 03
                    "MockKongSwapProvider::add_liquidity_amounts".to_string(),
                    "Mock response not set for add_liquidity_amounts".to_string(),
                    Some(HashMap::from([
                        ("token_0".to_string(), token_0),
                        ("amount".to_string(), amount.to_string()),
                        ("token_1".to_string(), token_1),
                    ]))
                )),
                |r| r.to_owned()
            )
    }

    async fn add_liquidity(
        &self,
        token_0: String,
        amount_0: Nat,
        token_1: String,
        amount_1: Nat,
        ledger0: Principal,
        ledger1: Principal,
    ) -> Result<AddLiquidityReply, InternalError> {
        self.add_liquidity_responses
            .get(&(token_0.clone(), amount_0.to_string(), token_1.clone(), amount_1.to_string(), ledger0.to_text(), ledger1.to_text()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(2301, 01, 04), // 2301 01 04
                    "MockKongSwapProvider::add_liquidity".to_string(),
                    "Mock response not set for add_liquidity".to_string(),
                    Some(HashMap::from([
                        ("token_0".to_string(), token_0),
                        ("amount_0".to_string(), amount_0.to_string()),
                        ("token_1".to_string(), token_1),
                        ("amount_1".to_string(), amount_1.to_string()),
                        ("ledger0".to_string(), ledger0.to_text()),
                        ("ledger1".to_string(), ledger1.to_text()),
                    ]))
                )),
                |r| r.to_owned()
            )
    }

    async fn user_balances(
        &self,
        principal_id: String,
    ) -> Result<Vec<UserBalancesReply>, InternalError> {
        self.user_balances_responses
            .get(&principal_id)
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(2301, 01, 05), // 2301 01 05
                    "MockKongSwapProvider::user_balances".to_string(),
                    "Mock response not set for user_balances".to_string(),
                    Some(HashMap::from([
                        ("principal_id".to_string(), principal_id),
                    ]))
                )),
                |r| r.to_owned()
            )
    }

    async fn remove_liquidity_amounts(
        &self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat,
    ) -> Result<RemoveLiquidityAmountsReply, InternalError> {
        self.remove_liquidity_amounts_responses
            .get(&(token_0.clone(), token_1.clone(), remove_lp_token_amount.to_string()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(2301, 01, 06), // 2301 01 06
                    "MockKongSwapProvider::remove_liquidity_amounts".to_string(),
                    "Mock response not set for remove_liquidity_amounts".to_string(),
                    Some(HashMap::from([
                        ("token_0".to_string(), token_0),
                        ("token_1".to_string(), token_1),
                        ("remove_lp_token_amount".to_string(), remove_lp_token_amount.to_string()),
                    ]))
                )),
                |r| r.to_owned()
            )
    }

    async fn remove_liquidity(
        &self,
        token_0: String,
        token_1: String,
        remove_lp_token_amount: Nat,
    ) -> Result<RemoveLiquidityReply, InternalError> {
        self.remove_liquidity_responses
            .get(&(token_0.clone(), token_1.clone(), remove_lp_token_amount.to_string()))
            .map_or_else(
                || Err(InternalError::not_found(
                    build_error_code(2301, 01, 07), // 2301 01 07
                    "MockKongSwapProvider::remove_liquidity".to_string(),
                    "Mock response not set for remove_liquidity".to_string(),
                    Some(HashMap::from([
                        ("token_0".to_string(), token_0),
                        ("token_1".to_string(), token_1),
                        ("remove_lp_token_amount".to_string(), remove_lp_token_amount.to_string()),
                    ]))
                )),
                |r| r.to_owned()
            )
    }
}