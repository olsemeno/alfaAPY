use std::collections::HashMap;
use candid::{Nat, Principal, Int};
use types::CanisterId;
use serde::{Serialize, Deserialize};
use candid::CandidType;

use icpswap_swap_factory_canister::ICPSwapPool;
use icpswap_swap_pool_canister::getTokenMeta::TokenMeta;
use icpswap_swap_pool_canister::metadata::Metadata;
use icpswap_swap_pool_canister::getUserPosition::UserPosition;
use icpswap_swap_pool_canister::decreaseLiquidity::DecreaseLiquidityResponse;
use icpswap_swap_pool_canister::claim::ClaimResponse;
use icpswap_swap_pool_canister::getUserUnusedBalance::UserUnusedBalance;
use icpswap_swap_pool_canister::getUserPositionsByPrincipal::UserPositionWithId;
use icpswap_node_index_canister::getAllTokens::TokenData;
use icpswap_swap_calculator_canister::getTokenAmountByLiquidity::GetTokenAmountByLiquidityResponse;
use icpswap_tvl_storage_canister::getPoolChartTvl::PoolChartTvl;
use errors::internal_error::error::{InternalError, build_error_code};

use crate::icpswap::ICPSwapProvider;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct MockICPSwapProvider {
    pub get_pool_responses: HashMap<(String, String), Result<ICPSwapPool, InternalError>>,
    pub quote_responses: HashMap<(String, String, bool, String), Result<Nat, InternalError>>,
    pub swap_responses: HashMap<(String, String, bool, String), Result<Nat, InternalError>>,
    pub get_token_meta_responses: HashMap<String, Result<TokenMeta, InternalError>>,
    pub deposit_from_responses: HashMap<(String, String, String, String), Result<Nat, InternalError>>,
    pub withdraw_responses: HashMap<(String, String, String, String), Result<Nat, InternalError>>,
    pub metadata_responses: HashMap<String, Result<Metadata, InternalError>>,
    pub mint_responses: HashMap<(String, String, String, String, String, String, String, String), Result<Nat, InternalError>>,
    pub get_user_position_ids_responses: HashMap<(String, String), Result<Vec<Nat>, InternalError>>,
    pub get_user_positions_responses: HashMap<(String, String), Result<Vec<UserPositionWithId>, InternalError>>,
    pub get_user_unused_balance_responses: HashMap<(String, String), Result<UserUnusedBalance, InternalError>>,
    pub increase_liquidity_responses: HashMap<(String, String, String, String), Result<Nat, InternalError>>,
    pub decrease_liquidity_responses: HashMap<(String, String, String), Result<DecreaseLiquidityResponse, InternalError>>,
    pub get_user_position_responses: HashMap<(String, String), Result<UserPosition, InternalError>>,
    pub claim_responses: HashMap<(String, String), Result<ClaimResponse, InternalError>>,
    pub get_price_responses: HashMap<(String, String, String), Result<f64, InternalError>>,
    pub get_token_amount_by_liquidity_responses: HashMap<(String, String, String, String), Result<GetTokenAmountByLiquidityResponse, InternalError>>,
    pub get_all_tokens_responses: Result<Vec<TokenData>, InternalError>,
    pub get_tvl_storage_canister_responses: Result<Vec<String>, InternalError>,
    pub get_pool_chart_tvl_responses: HashMap<(String, String, String, String), Result<Vec<PoolChartTvl>, InternalError>>,
}

impl Default for MockICPSwapProvider {
    fn default() -> Self {
        Self {
            get_pool_responses: HashMap::new(),
            quote_responses: HashMap::new(),
            swap_responses: HashMap::new(),
            get_token_meta_responses: HashMap::new(),
            deposit_from_responses: HashMap::new(),
            withdraw_responses: HashMap::new(),
            metadata_responses: HashMap::new(),
            mint_responses: HashMap::new(),
            get_user_position_ids_responses: HashMap::new(),
            get_user_positions_responses: HashMap::new(),
            get_user_unused_balance_responses: HashMap::new(),
            increase_liquidity_responses: HashMap::new(),
            decrease_liquidity_responses: HashMap::new(),
            get_user_position_responses: HashMap::new(),
            claim_responses: HashMap::new(),
            get_price_responses: HashMap::new(),
            get_token_amount_by_liquidity_responses: HashMap::new(),
            get_pool_chart_tvl_responses: HashMap::new(),
            get_all_tokens_responses: Err(InternalError::not_found(
                build_error_code(2302, 01, 01), // 2302 01 01
                "MockICPSwapProvider::get_all_tokens".to_string(),
                "Mock response not set for get_all_tokens".to_string(),
                None
            )),
            get_tvl_storage_canister_responses: Err(InternalError::not_found(
                build_error_code(2302, 01, 02), // 2302 01 02
                "MockICPSwapProvider::get_tvl_storage_canister".to_string(),
                "Mock response not set for get_tvl_storage_canister".to_string(),
                None
            )),
        }
    }
}

impl MockICPSwapProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mock_get_pool(
        &mut self,
        token_in: CanisterId,
        token_out: CanisterId,
        response: Result<ICPSwapPool, InternalError>,
    ) {
        self.get_pool_responses.insert((token_in.to_text(), token_out.to_text()), response);
    }

    pub fn mock_quote(
        &mut self,
        canister_id: CanisterId,
        amount_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat,
        response: Result<Nat, InternalError>,
    ) {
        self.quote_responses.insert(
            (canister_id.to_text(), amount_in.to_string(), zero_for_one, amount_out_minimum.to_string()),
            response
        );
    }

    pub fn mock_swap(
        &mut self,
        canister_id: CanisterId,
        amount_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat,
        response: Result<Nat, InternalError>,
    ) {
        self.swap_responses.insert(
            (canister_id.to_text(), amount_in.to_string(), zero_for_one, amount_out_minimum.to_string()),
            response
        );
    }

    pub fn mock_get_token_meta(
        &mut self,
        canister_id: CanisterId,
        response: Result<TokenMeta, InternalError>,
    ) {
        self.get_token_meta_responses.insert(canister_id.to_text(), response);
    }

    pub fn mock_deposit_from(
        &mut self,
        canister_id: CanisterId,
        token_in: CanisterId,
        amount: Nat,
        token_fee: Nat,
        response: Result<Nat, InternalError>,
    ) {
        self.deposit_from_responses.insert(
            (canister_id.to_text(), token_in.to_text(), amount.to_string(), token_fee.to_string()),
            response
        );
    }

    pub fn mock_withdraw(
        &mut self,
        canister_id: CanisterId,
        token_out: CanisterId,
        amount: Nat,
        token_fee: Nat,
        response: Result<Nat, InternalError>,
    ) {
        self.withdraw_responses.insert(
            (canister_id.to_text(), token_out.to_text(), amount.to_string(), token_fee.to_string()),
            response
        );
    }

    pub fn mock_metadata(
        &mut self,
        canister_id: CanisterId,
        response: Result<Metadata, InternalError>,
    ) {
        self.metadata_responses.insert(canister_id.to_text(), response);
    }

    pub fn mock_mint(
        &mut self,
        canister_id: CanisterId,
        token0: String,
        token1: String,
        amount0_desired: String,
        amount1_desired: String,
        fee: Nat,
        tick_lower: Int,
        tick_upper: Int,
        response: Result<Nat, InternalError>,
    ) {
        self.mint_responses.insert(
            (
                canister_id.to_text(),
                token0,
                token1,
                amount0_desired,
                amount1_desired,
                fee.to_string(),
                tick_lower.to_string(),
                tick_upper.to_string(),
            ),
            response
        );
    }

    pub fn mock_get_user_position_ids(
        &mut self,
        canister_id: CanisterId,
        principal: Principal,
        response: Result<Vec<Nat>, InternalError>,
    ) {
        self.get_user_position_ids_responses.insert(
            (canister_id.to_text(), principal.to_text()),
            response
        );
    }

    pub fn mock_get_user_positions(
        &mut self,
        canister_id: CanisterId,
        principal: Principal,
        response: Result<Vec<UserPositionWithId>, InternalError>,
    ) {
        self.get_user_positions_responses.insert(
            (canister_id.to_text(), principal.to_text()),
            response
        );
    }

    pub fn mock_get_user_unused_balance(
        &mut self,
        canister_id: CanisterId,
        principal: String,
        response: Result<UserUnusedBalance, InternalError>,
    ) {
        self.get_user_unused_balance_responses.insert(
            (canister_id.to_text(), principal),
            response
        );
    }

    pub fn mock_increase_liquidity(
        &mut self,
        canister_id: CanisterId,
        position_id: Nat,
        amount0_desired: String,
        amount1_desired: String,
        response: Result<Nat, InternalError>,
    ) {
        self.increase_liquidity_responses.insert(
            (canister_id.to_text(), position_id.to_string(), amount0_desired, amount1_desired),
            response
        );
    }

    pub fn mock_decrease_liquidity(
        &mut self,
        canister_id: CanisterId,
        position_id: Nat,
        liquidity: String,
        response: Result<DecreaseLiquidityResponse, InternalError>,
    ) {
        self.decrease_liquidity_responses.insert(
            (canister_id.to_text(), position_id.to_string(), liquidity),
            response
        );
    }

    pub fn mock_get_user_position(
        &mut self,
        canister_id: CanisterId,
        position_id: Nat,
        response: Result<UserPosition, InternalError>,
    ) {
        self.get_user_position_responses.insert(
            (canister_id.to_text(), position_id.to_string()),
            response
        );
    }

    pub fn mock_claim(
        &mut self,
        canister_id: CanisterId,
        position_id: Nat,
        response: Result<ClaimResponse, InternalError>,
    ) {
        self.claim_responses.insert(
            (canister_id.to_text(), position_id.to_string()),
            response
        );
    }

    pub fn mock_get_price(
        &mut self,
        sqrt_price_x96: Nat,
        token_0_decimals: Nat,
        token_1_decimals: Nat,
        response: Result<f64, InternalError>,
    ) {
        self.get_price_responses.insert(
            (sqrt_price_x96.to_string(), token_0_decimals.to_string(), token_1_decimals.to_string()),
            response
        );
    }

    pub fn mock_get_token_amount_by_liquidity(
        &mut self,
        sqrt_price_x96: Nat,
        tick_lower: Int,
        tick_upper: Int,
        liquidity: Nat,
        response: Result<GetTokenAmountByLiquidityResponse, InternalError>,
    ) {
        self.get_token_amount_by_liquidity_responses.insert(
            (sqrt_price_x96.to_string(), tick_lower.to_string(), tick_upper.to_string(), liquidity.to_string()),
            response
        );
    }

    pub fn mock_get_all_tokens(
        &mut self,
        response: Result<Vec<TokenData>, InternalError>,
    ) {
        self.get_all_tokens_responses = response;
    }

    pub fn mock_get_tvl_storage_canister(
        &mut self,
        response: Result<Vec<String>, InternalError>,
    ) {
        self.get_tvl_storage_canister_responses = response;
    }

    pub fn mock_get_pool_chart_tvl(
        &mut self,
        canister_id: CanisterId,
        pool_canister_id: String,
        offset: Nat,
        limit: Nat,
        response: Result<Vec<PoolChartTvl>, InternalError>,
    ) {
        self.get_pool_chart_tvl_responses.insert(
            (canister_id.to_text(), pool_canister_id, offset.to_string(), limit.to_string()),
            response
        );
    }
}

#[async_trait::async_trait]
impl ICPSwapProvider for MockICPSwapProvider {
    async fn get_pool(&self, token_in: CanisterId, token_out: CanisterId) -> Result<ICPSwapPool, InternalError> {
        self.get_pool_responses
            .get(&(token_in.to_text(), token_out.to_text()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 03), // 2302 01 03
                "MockICPSwapProvider::get_pool".to_string(),
                "Mock response not set for get_pool".to_string(),
                Some(HashMap::from([
                    ("token_in".to_string(), token_in.to_text()),
                    ("token_out".to_string(), token_out.to_text()),
                ]))
            )))
    }

    async fn quote(&self, canister_id: CanisterId, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, InternalError> {
        self.quote_responses
            .get(&(canister_id.to_text(), amount_in.to_string(), zero_for_one, amount_out_minimum.to_string()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 04), // 2302 01 04
                "MockICPSwapProvider::quote".to_string(),
                "Mock response not set for quote".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("amount_in".to_string(), amount_in.to_string()),
                    ("zero_for_one".to_string(), zero_for_one.to_string()),
                    ("amount_out_minimum".to_string(), amount_out_minimum.to_string()),
                ]))
            )))
    }

    async fn swap(&self, canister_id: CanisterId, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, InternalError> {
        self.swap_responses
            .get(&(canister_id.to_text(), amount_in.to_string(), zero_for_one, amount_out_minimum.to_string()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 05), // 2302 01 05
                "MockICPSwapProvider::swap".to_string(),
                "Mock response not set for swap".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("amount_in".to_string(), amount_in.to_string()),
                    ("zero_for_one".to_string(), zero_for_one.to_string()),
                    ("amount_out_minimum".to_string(), amount_out_minimum.to_string()),
                ]))
            )))
    }

    async fn get_token_meta(&self, canister_id: CanisterId) -> Result<TokenMeta, InternalError> {
        self.get_token_meta_responses
            .get(&canister_id.to_text())
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 06), // 2302 01 06
                "MockICPSwapProvider::get_token_meta".to_string(),
                "Mock response not set for get_token_meta".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                ]))
            )))
    }

    async fn deposit_from(
        &self,
        canister_id: CanisterId,
        token_in: CanisterId,
        amount: Nat,
        token_fee: Nat
    ) -> Result<Nat, InternalError> {
        self.deposit_from_responses
            .get(&(canister_id.to_text(), token_in.to_text(), amount.to_string(), token_fee.to_string()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 07), // 2302 01 07
                "MockICPSwapProvider::deposit_from".to_string(),
                "Mock response not set for deposit_from".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("token_in".to_string(), token_in.to_text()),
                    ("amount".to_string(), amount.to_string()),
                    ("token_fee".to_string(), token_fee.to_string()),
                ]))
            )))
    }

    async fn withdraw(
        &self,
        canister_id: CanisterId,
        token_out: CanisterId,
        amount: Nat,
        token_fee: Nat
    ) -> Result<Nat, InternalError> {
        self.withdraw_responses
            .get(&(canister_id.to_text(), token_out.to_text(), amount.to_string(), token_fee.to_string()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 08), // 2302 01 08
                "MockICPSwapProvider::withdraw".to_string(),
                "Mock response not set for withdraw".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("token_out".to_string(), token_out.to_text()),
                    ("amount".to_string(), amount.to_string()),
                    ("token_fee".to_string(), token_fee.to_string()),
                ]))
            )))
    }

    async fn metadata(&self, canister_id: CanisterId) -> Result<Metadata, InternalError> {
        self.metadata_responses
            .get(&canister_id.to_text())
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 09), // 2302 01 09
                "MockICPSwapProvider::metadata".to_string(),
                "Mock response not set for metadata".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                ]))
            )))
    }

    async fn mint(
        &self,
        canister_id: CanisterId,
        token0: String,
        token1: String,
        amount0_desired: String,
        amount1_desired: String,
        fee: Nat,
        tick_lower: Int,
        tick_upper: Int
    ) -> Result<Nat, InternalError> {
        self.mint_responses
            .get(&(
                canister_id.to_text(),
                token0.clone(),
                token1.clone(),
                amount0_desired.clone(),
                amount1_desired.clone(),
                fee.to_string(),
                tick_lower.to_string(),
                tick_upper.to_string(),
            ))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 10), // 2302 01 10
                "MockICPSwapProvider::mint".to_string(),
                "Mock response not set for mint".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("token0".to_string(), token0),
                    ("token1".to_string(), token1),
                    ("amount0_desired".to_string(), amount0_desired),
                    ("amount1_desired".to_string(), amount1_desired),
                    ("fee".to_string(), fee.to_string()),
                    ("tick_lower".to_string(), tick_lower.to_string()),
                    ("tick_upper".to_string(), tick_upper.to_string()),
                ]))
            )))
    }

    async fn get_user_position_ids_by_principal(
        &self,
        canister_id: CanisterId,
        principal: Principal
    ) -> Result<Vec<Nat>, InternalError> {
        self.get_user_position_ids_responses
            .get(&(canister_id.to_text(), principal.to_text()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 11), // 2302 01 11
                "MockICPSwapProvider::get_user_position_ids_by_principal".to_string(),
                "Mock response not set for get_user_position_ids_by_principal".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("principal".to_string(), principal.to_text()),
                ]))
            )))
    }

    async fn get_user_positions_by_principal(
        &self,
        canister_id: CanisterId,
        principal: Principal
    ) -> Result<Vec<UserPositionWithId>, InternalError> {
        self.get_user_positions_responses
            .get(&(canister_id.to_text(), principal.to_text()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 12), // 2302 01 12
                "MockICPSwapProvider::get_user_positions_by_principal".to_string(),
                "Mock response not set for get_user_positions_by_principal".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("principal".to_string(), principal.to_text()),
                ]))
            )))
    }

    async fn get_user_unused_balance(
        &self,
        canister_id: CanisterId,
        principal: String
    ) -> Result<UserUnusedBalance, InternalError> {
        self.get_user_unused_balance_responses
            .get(&(canister_id.to_text(), principal.clone()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 13), // 2302 01 13
                "MockICPSwapProvider::get_user_unused_balance".to_string(),
                "Mock response not set for get_user_unused_balance".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("principal".to_string(), principal),
                ]))
            )))
    }

    async fn increase_liquidity(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
        amount0_desired: String,
        amount1_desired: String
    ) -> Result<Nat, InternalError> {
        self.increase_liquidity_responses
            .get(&(canister_id.to_text(), position_id.to_string(), amount0_desired.clone(), amount1_desired.clone()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 14), // 2302 01 14
                "MockICPSwapProvider::increase_liquidity".to_string(),
                "Mock response not set for increase_liquidity".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("position_id".to_string(), position_id.to_string()),
                    ("amount0_desired".to_string(), amount0_desired),
                    ("amount1_desired".to_string(), amount1_desired),
                ]))
            )))
    }

    async fn decrease_liquidity(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
        liquidity: String
    ) -> Result<DecreaseLiquidityResponse, InternalError> {
        self.decrease_liquidity_responses
            .get(&(canister_id.to_text(), position_id.to_string(), liquidity.clone()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 15), // 2302 01 15
                "MockICPSwapProvider::decrease_liquidity".to_string(),
                "Mock response not set for decrease_liquidity".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("position_id".to_string(), position_id.to_string()),
                    ("liquidity".to_string(), liquidity),
                ]))
            )))
    }

    async fn get_user_position(
        &self,
        canister_id: CanisterId,
        position_id: Nat
    ) -> Result<UserPosition, InternalError> {
        self.get_user_position_responses
            .get(&(canister_id.to_text(), position_id.to_string()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 16), // 2302 01 16
                "MockICPSwapProvider::get_user_position".to_string(),
                "Mock response not set for get_user_position".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("position_id".to_string(), position_id.to_string()),
                ]))
            )))
    }

    async fn claim(
        &self,
        canister_id: CanisterId,
        position_id: Nat
    ) -> Result<ClaimResponse, InternalError> {
        self.claim_responses
            .get(&(canister_id.to_text(), position_id.to_string()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 17), // 2302 01 17
                "MockICPSwapProvider::claim".to_string(),
                "Mock response not set for claim".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("position_id".to_string(), position_id.to_string()),
                ]))
            )))
    }

    async fn get_price(
        &self,
        sqrt_price_x96: Nat,
        token_0_decimals: Nat,
        token_1_decimals: Nat
    ) -> Result<f64, InternalError> {
        self.get_price_responses
            .get(&(sqrt_price_x96.to_string(), token_0_decimals.to_string(), token_1_decimals.to_string()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 18), // 2302 01 18
                "MockICPSwapProvider::get_price".to_string(),
                "Mock response not set for get_price".to_string(),
                Some(HashMap::from([
                    ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                    ("token_0_decimals".to_string(), token_0_decimals.to_string()),
                    ("token_1_decimals".to_string(), token_1_decimals.to_string()),
                ]))
            )))
    }

    async fn get_token_amount_by_liquidity(
        &self,
        sqrt_price_x96: Nat,
        tick_lower: Int,
        tick_upper: Int,
        liquidity: Nat
    ) -> Result<GetTokenAmountByLiquidityResponse, InternalError> {
        self.get_token_amount_by_liquidity_responses
            .get(&(sqrt_price_x96.to_string(), tick_lower.to_string(), tick_upper.to_string(), liquidity.to_string()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 19), // 2302 01 19
                "MockICPSwapProvider::get_token_amount_by_liquidity".to_string(),
                "Mock response not set for get_token_amount_by_liquidity".to_string(),
                Some(HashMap::from([
                    ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                    ("tick_lower".to_string(), tick_lower.to_string()),
                    ("tick_upper".to_string(), tick_upper.to_string()),
                    ("liquidity".to_string(), liquidity.to_string()),
                ]))
            )))
    }

    async fn get_all_tokens(&self) -> Result<Vec<TokenData>, InternalError> {
        self.get_all_tokens_responses.clone()
    }

    async fn get_tvl_storage_canister(&self) -> Result<Vec<String>, InternalError> {
        self.get_tvl_storage_canister_responses.clone()
    }

    async fn get_pool_chart_tvl(
        &self,
        canister_id: CanisterId,
        pool_canister_id: String,
        offset: Nat,
        limit: Nat
    ) -> Result<Vec<PoolChartTvl>, InternalError> {
        self.get_pool_chart_tvl_responses
            .get(&(canister_id.to_text(), pool_canister_id.clone(), offset.to_string(), limit.to_string()))
            .cloned()
            .unwrap_or_else(|| Err(InternalError::not_found(
                build_error_code(2302, 01, 20), // 2302 01 20
                "MockICPSwapProvider::get_pool_chart_tvl".to_string(),
                "Mock response not set for get_pool_chart_tvl".to_string(),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("pool_canister_id".to_string(), pool_canister_id),
                    ("offset".to_string(), offset.to_string()),
                    ("limit".to_string(), limit.to_string()),
                ]))
            )))
    }
}
