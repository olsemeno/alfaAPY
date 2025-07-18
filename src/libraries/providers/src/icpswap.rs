use types::CanisterId;
use candid::{Nat, Principal, Int};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use candid::CandidType;

use icpswap_swap_factory_canister::{ICPSwapToken, ICPSwapPool};
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
use icpswap_swap_pool_canister::quote::Args as QuoteArgs;
use icpswap_swap_factory_canister::getPool::Args as GetPoolArgs;
use icpswap_swap_pool_canister::depositFrom::Args as DepositFromArgs;
use icpswap_swap_pool_canister::increaseLiquidity::Args as IncreaseLiquidityArgs;
use icpswap_swap_pool_canister::mint::Args as MintArgs;
use icpswap_swap_pool_canister::withdraw::Args as WithdrawArgs;
use icpswap_swap_pool_canister::getUserUnusedBalance::Args as GetUserUnusedBalanceArgs;
use icpswap_swap_pool_canister::decreaseLiquidity::Args as DecreaseLiquidityArgs;
use icpswap_swap_pool_canister::swap::Args as SwapArgs;
use icpswap_swap_pool_canister::claim::Args as ClaimArgs;
use errors::internal_error::error::{InternalError, build_error_code};
use utils::constants::{
    ICP_TOKEN_PRINCIPAL,
    ICPSWAP_SWAP_FACTORY_CANISTER_ID,
    ICPSWAP_SWAP_CALCULATOR_CANISTER_ID,
    ICPSWAP_NODE_INDEX_CANISTER_ID,
    ICPSWAP_GLOBAL_INDEX_CANISTER_ID,
};

pub const SWAP_FEE: u128 = 3000;
pub const ICRC2_TOKEN_STANDARD: &str = "ICRC2";
pub const ICP_TOKEN_STANDARD: &str = "ICP";

#[async_trait::async_trait]
pub trait ICPSwapProvider: Send + Sync + 'static {
    async fn get_pool(&self, token_in: CanisterId, token_out: CanisterId) -> Result<ICPSwapPool, InternalError>;
    async fn quote(&self, canister_id: CanisterId, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, InternalError>;
    async fn swap(&self, canister_id: CanisterId, amount_in: Nat, zero_for_one: bool, amount_out_minimum: Nat) -> Result<Nat, InternalError>;
    async fn get_token_meta(&self, canister_id: CanisterId) -> Result<TokenMeta, InternalError>;
    async fn deposit_from(&self, canister_id: CanisterId, token_in: CanisterId, amount: Nat, token_fee: Nat) -> Result<Nat, InternalError>;
    async fn withdraw(&self, canister_id: CanisterId, token_out: CanisterId, amount: Nat, token_fee: Nat) -> Result<Nat, InternalError>;
    async fn metadata(&self, canister_id: CanisterId) -> Result<Metadata, InternalError>;
    async fn mint(&self, canister_id: CanisterId, token0: String, token1: String, amount0_desired: String, amount1_desired: String, fee: Nat, tick_lower: Int, tick_upper: Int) -> Result<Nat, InternalError>;
    async fn get_user_position_ids_by_principal(&self, canister_id: CanisterId, principal: Principal) -> Result<Vec<Nat>, InternalError>;
    async fn get_user_positions_by_principal(&self, canister_id: CanisterId, principal: Principal) -> Result<Vec<UserPositionWithId>, InternalError>;
    async fn get_user_unused_balance(&self, canister_id: CanisterId, principal: String) -> Result<UserUnusedBalance, InternalError>;
    async fn increase_liquidity(&self, canister_id: CanisterId, position_id: Nat, amount0_desired: String, amount1_desired: String) -> Result<Nat, InternalError>;
    async fn decrease_liquidity(&self, canister_id: CanisterId, position_id: Nat, liquidity: String) -> Result<DecreaseLiquidityResponse, InternalError>;
    async fn get_user_position(&self, canister_id: CanisterId, position_id: Nat) -> Result<UserPosition, InternalError>;
    async fn claim(&self, canister_id: CanisterId, position_id: Nat) -> Result<ClaimResponse, InternalError>;
    async fn get_price(&self, sqrt_price_x96: Nat, token_0_decimals: Nat, token_1_decimals: Nat) -> Result<f64, InternalError>;
    async fn get_token_amount_by_liquidity(&self, sqrt_price_x96: Nat, tick_lower: Int, tick_upper: Int, liquidity: Nat) -> Result<GetTokenAmountByLiquidityResponse, InternalError>;
    async fn get_all_tokens(&self) -> Result<Vec<TokenData>, InternalError>;
    async fn get_tvl_storage_canister(&self) -> Result<Vec<String>, InternalError>;
    async fn get_pool_chart_tvl(&self, canister_id: CanisterId, pool_canister_id: String, offset: Nat, limit: Nat) -> Result<Vec<PoolChartTvl>, InternalError>;
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct DefaultICPSwapProvider;

impl DefaultICPSwapProvider {
    fn token_icpswap_format(token: &CanisterId) -> ICPSwapToken {
        let standard = match token.to_text().as_str() {
            ICP_TOKEN_PRINCIPAL => ICP_TOKEN_STANDARD.to_string(),
            _ => ICRC2_TOKEN_STANDARD.to_string(),
        };

        ICPSwapToken {
            address: token.to_text(),
            standard,
        }
    }
}

#[async_trait::async_trait]
impl ICPSwapProvider for DefaultICPSwapProvider {
    // ================ Swap Factory canister ================

    async fn get_pool(
        &self,
        token_in: CanisterId,
        token_out: CanisterId
    ) -> Result<ICPSwapPool, InternalError> {
        let pool_args = GetPoolArgs {
            fee: candid::Nat::from(SWAP_FEE as u128),
            token0: Self::token_icpswap_format(&token_in),
            token1: Self::token_icpswap_format(&token_out),
        };

        icpswap_swap_factory_canister_c2c_client::getPool(
            *ICPSWAP_SWAP_FACTORY_CANISTER_ID,
            &pool_args
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 1), // 1002 03 01
                    "ICPSwapProvider::get_pool".to_string(),
                    format!("IC error calling 'icpswap_swap_factory_canister_c2c_client::getPool': {error:?}"),
                    Some(HashMap::from([
                        ("token_in".to_string(), token_in.to_text()),
                        ("token_out".to_string(), token_out.to_text()),
                        ("fee".to_string(), pool_args.fee.to_string()),
                        ("swap_factory_canister".to_string(), ICPSWAP_SWAP_FACTORY_CANISTER_ID.to_text()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 2), // 1002 03 02
                    "ICPSwapProvider::get_pool".to_string(),
                    format!("Error calling 'icpswap_swap_factory_canister_c2c_client::getPool': {error:?}"),
                    Some(HashMap::from([
                        ("token_in".to_string(), token_in.to_text()),
                        ("token_out".to_string(), token_out.to_text()),
                        ("fee".to_string(), pool_args.fee.to_string()),
                        ("swap_factory_canister".to_string(), ICPSWAP_SWAP_FACTORY_CANISTER_ID.to_text()),
                    ]))
                )
            })
            .into_std()
    }

    // ================ Swap Pool canister ================

    async fn quote(
        &self,
        canister_id: CanisterId,
        amount_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat
    ) -> Result<Nat, InternalError> {
        let quote_args = &QuoteArgs {
            amountIn: amount_in.to_string(),
            zeroForOne: zero_for_one,
            amountOutMinimum: amount_out_minimum.to_string(),
        };

        icpswap_swap_pool_canister_c2c_client::quote(canister_id, quote_args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 3), // 1002 04 03
                    "ICPSwapProvider::quote".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::quote': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("amount_in".to_string(), amount_in.to_string()),
                        ("zero_for_one".to_string(), zero_for_one.to_string()),
                        ("amount_out_minimum".to_string(), amount_out_minimum.to_string()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 4), // 1002 03 04
                    "ICPSwapProvider::quote".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::quote': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("amount_in".to_string(), amount_in.to_string()),
                        ("zero_for_one".to_string(), zero_for_one.to_string()),
                        ("amount_out_minimum".to_string(), amount_out_minimum.to_string()),
                    ]))
                )
            })
            .into_std()
    }

    async fn swap(
        &self,
        canister_id: CanisterId,
        amount_in: Nat,
        zero_for_one: bool,
        amount_out_minimum: Nat
    ) -> Result<Nat, InternalError> {
        let args = SwapArgs {
            amountIn: amount_in.to_string(),
            zeroForOne: zero_for_one,
            amountOutMinimum: amount_out_minimum.to_string(),
        };

        icpswap_swap_pool_canister_c2c_client::swap(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 5), // 1002 04 05
                    "ICPSwapProvider::swap".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::swap': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("amount_in".to_string(), amount_in.to_string()),
                        ("zero_for_one".to_string(), zero_for_one.to_string()),
                        ("amount_out_minimum".to_string(), amount_out_minimum.to_string()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 6), // 1002 03 06
                    "ICPSwapProvider::swap".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::swap': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("amount_in".to_string(), amount_in.to_string()),
                        ("zero_for_one".to_string(), zero_for_one.to_string()),
                        ("amount_out_minimum".to_string(), amount_out_minimum.to_string()),
                    ]))
                )
            })
            .into_std()
    }

    async fn get_token_meta(
        &self,
        canister_id: CanisterId
    ) -> Result<TokenMeta, InternalError> {
        icpswap_swap_pool_canister_c2c_client::getTokenMeta(canister_id).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 7), // 1002 04 07
                    "ICPSwapProvider::get_token_meta".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getTokenMeta': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 8), // 1002 03 08
                    "ICPSwapProvider::get_token_meta".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getTokenMeta': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                    ]))
                )
            })
            .into_std()
    }

    async fn deposit_from(
        &self,
        canister_id: CanisterId,
        token_in: CanisterId,
        amount: Nat,
        token_fee: Nat
    ) -> Result<Nat, InternalError> {
        let args = DepositFromArgs {
            token: token_in.to_text(),
            amount: amount.clone(),
            fee: token_fee.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::depositFrom(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 9), // 1002 04 09
                    "ICPSwapProvider::deposit_from".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::depositFrom': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("token_in".to_string(), token_in.to_text()),
                        ("amount".to_string(), amount.to_string()),
                        ("token_fee".to_string(), token_fee.to_string()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 10), // 1002 03 10
                    "ICPSwapProvider::deposit_from".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::depositFrom': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("token_in".to_string(), token_in.to_text()),
                        ("amount".to_string(), amount.to_string()),
                        ("token_fee".to_string(), token_fee.to_string()),
                    ]))
                )
            })
            .into_std()
    }

    async fn withdraw(
        &self,
        canister_id: CanisterId,
        token_out: CanisterId,
        amount: Nat,
        token_fee: Nat
    ) -> Result<Nat, InternalError> {
        let args = WithdrawArgs {
            token: token_out.to_text(),
            amount: amount.clone(),
            fee: token_fee.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::withdraw(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 11), // 1002 04 11
                    "ICPSwapProvider::withdraw".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::withdraw': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("token_out".to_string(), token_out.to_text()),
                        ("amount".to_string(), amount.to_string()),
                        ("token_fee".to_string(), token_fee.to_string()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 12), // 1002 03 12
                    "ICPSwapProvider::withdraw".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::withdraw': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("token_out".to_string(), token_out.to_text()),
                        ("amount".to_string(), amount.to_string()),
                        ("token_fee".to_string(), token_fee.to_string()),
                    ]))
                )
            })
            .into_std()
    }

    async fn metadata(
        &self,
        canister_id: CanisterId
    ) -> Result<Metadata, InternalError> {
        icpswap_swap_pool_canister_c2c_client::metadata(canister_id).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 13), // 1002 04 13
                    "ICPSwapProvider::metadata".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::metadata': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 14), // 1002 03 14
                    "ICPSwapProvider::metadata".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::metadata': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                    ]))
                )
            })
            .into_std()
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
        let args = MintArgs {
            fee: fee.clone(),
            tickUpper: tick_upper.clone(),
            token0: token0.clone(),
            token1: token1.clone(),
            amount0Desired: amount0_desired.clone(),
            amount1Desired: amount1_desired.clone(),
            tickLower: tick_lower.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::mint(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 15), // 10020 4 15
                    "ICPSwapProvider::mint".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::mint': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("token0".to_string(), token0.clone()),
                        ("token1".to_string(), token1.clone()),
                        ("amount0_desired".to_string(), amount0_desired.clone()),
                        ("amount1_desired".to_string(), amount1_desired.clone()),
                        ("fee".to_string(), fee.to_string()),
                        ("tick_lower".to_string(), tick_lower.to_string()),
                        ("tick_upper".to_string(), tick_upper.to_string()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 16), // 1002 03 16
                    "ICPSwapProvider::mint".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::mint': {error:?}"),
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
                )
            })
            .into_std()
    }

    async fn get_user_position_ids_by_principal(
        &self,
        canister_id: CanisterId,
        principal: Principal
    ) -> Result<Vec<Nat>, InternalError> {
        let (result,) = icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal(
            canister_id,
            (principal,)
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 17), // 1002 04 17
                    "ICPSwapProvider::get_user_position_ids_by_principal".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("principal".to_string(), principal.to_text()),
                    ]))
                )
            })?;

        result.map_err(|error| {
            InternalError::business_logic(
                build_error_code(1002, 3, 18), // 1002 03 18
                "ICPSwapProvider::get_user_position_ids_by_principal".to_string(),
                format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal': {error:?}"),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("principal".to_string(), principal.to_text()),
                ]))
            )
        })
        .into_std()
    }

    async fn get_user_positions_by_principal(
        &self,
        canister_id: CanisterId,
        principal: Principal
    ) -> Result<Vec<UserPositionWithId>, InternalError> {
        let (result,) = icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal(
            canister_id,
            (principal,)
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 19), // 1002 04 19
                    "ICPSwapProvider::get_user_positions_by_principal".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("principal".to_string(), principal.to_text()),
                    ]))
                )
            })?;

        result.map_err(|error| {
            InternalError::business_logic(
                build_error_code(1002, 3, 20), // 1002 03 20
                "ICPSwapProvider::get_user_positions_by_principal".to_string(),
                format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal': {error:?}"),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("principal".to_string(), principal.to_text()),
                ]))
            )
        })
        .into_std()
    }

    async fn get_user_unused_balance(
        &self,
        canister_id: CanisterId,
        principal: String,
    ) -> Result<UserUnusedBalance, InternalError> {
        let args = GetUserUnusedBalanceArgs {
            principal: principal.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 21), // 1002 04 21
                    "ICPSwapProvider::get_user_unused_balance".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("principal".to_string(), principal.clone()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 22), // 1002 03 22
                    "ICPSwapProvider::get_user_unused_balance".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("principal".to_string(), principal),
                    ]))
                )
            })
            .into_std()
    }

    async fn increase_liquidity(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
        amount0_desired: String,
        amount1_desired: String
    ) -> Result<Nat, InternalError> {
        let args = IncreaseLiquidityArgs {
            positionId: position_id.clone(),
            amount0Desired: amount0_desired.clone(),
            amount1Desired: amount1_desired.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::increaseLiquidity(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 23), // 1002 04 23
                    "ICPSwapProvider::increase_liquidity".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::increaseLiquidity': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("position_id".to_string(), position_id.to_string()),
                        ("amount0_desired".to_string(), amount0_desired.clone()),
                        ("amount1_desired".to_string(), amount1_desired.clone()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 24), // 1002 03 24
                    "ICPSwapProvider::increase_liquidity".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::increaseLiquidity': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("position_id".to_string(), position_id.to_string()),
                        ("amount0_desired".to_string(), amount0_desired),
                        ("amount1_desired".to_string(), amount1_desired),
                    ]))
                )
            })
            .into_std()
    }

    async fn decrease_liquidity(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
        liquidity: String,
    ) -> Result<DecreaseLiquidityResponse, InternalError> {
        let args = DecreaseLiquidityArgs {
            positionId: position_id.clone(),
            liquidity: liquidity.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::decreaseLiquidity(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 25), // 1002 04 25
                    "ICPSwapProvider::decrease_liquidity".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::decreaseLiquidity': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("position_id".to_string(), position_id.to_string()),
                        ("liquidity".to_string(), liquidity.clone()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 26), // 1002 03 26
                    "ICPSwapProvider::decrease_liquidity".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::decreaseLiquidity': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("position_id".to_string(), position_id.to_string()),
                        ("liquidity".to_string(), liquidity),
                    ]))
                )
            })
            .into_std()
    }

    async fn get_user_position(
        &self,
        canister_id: CanisterId,
        position_id: Nat
    ) -> Result<UserPosition, InternalError> {
        let args = (position_id.clone(),);
        let (result,) = icpswap_swap_pool_canister_c2c_client::getUserPosition(
            canister_id,
            args
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 27), // 1002 04 27
                    "ICPSwapProvider::get_user_position".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserPosition': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("position_id".to_string(), position_id.to_string()),
                    ]))
                )
            })?;

        result.map_err(|error| {
            InternalError::business_logic(
                build_error_code(1002, 3, 28), // 1002 03 28
                "ICPSwapProvider::get_user_position".to_string(),
                format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getUserPosition': {error:?}"),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                    ("position_id".to_string(), position_id.to_string()),
                ]))
            )
        })
        .into_std()
    }

    async fn claim(
        &self,
        canister_id: CanisterId,
        position_id: Nat,
    ) -> Result<ClaimResponse, InternalError> {
        let args = ClaimArgs {
            positionId: position_id.clone(),
        };

        icpswap_swap_pool_canister_c2c_client::claim(canister_id, &args).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 29), // 1002 04 29
                    "ICPSwapProvider::claim".to_string(),
                    format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::claim': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("position_id".to_string(), position_id.to_string()),
                    ]))
                )
            })?
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 30), // 1002 03 30
                    "ICPSwapProvider::claim".to_string(),
                    format!("Error calling 'icpswap_swap_pool_canister_c2c_client::claim': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("position_id".to_string(), position_id.to_string()),
                    ]))
                )
            })
            .into_std()
    }

    // ================ Swap Calculator canister ================

    async fn get_price(
        &self,
        sqrt_price_x96: Nat,
        token_0_decimals: Nat,
        token_1_decimals: Nat
    ) -> Result<f64, InternalError> {
        let (price,) = icpswap_swap_calculator_canister_c2c_client::getPrice(
            *ICPSWAP_SWAP_CALCULATOR_CANISTER_ID,
            (sqrt_price_x96.clone(), token_0_decimals.clone(), token_1_decimals.clone())
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 31), // 100204  31
                    "ICPSwapProvider::get_price".to_string(),
                    format!("IC error calling 'icpswap_swap_calculator_canister_c2c_client::getPrice': {error:?}"),
                    Some(HashMap::from([
                        ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                        ("token_0_decimals".to_string(), token_0_decimals.to_string()),
                        ("token_1_decimals".to_string(), token_1_decimals.to_string()),
                        ("swap_calculator_canister".to_string(), ICPSWAP_SWAP_CALCULATOR_CANISTER_ID.to_text()),
                    ]))
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 32), // 1002 03 32
                    "ICPSwapProvider::get_price".to_string(),
                    format!("Error calling 'icpswap_swap_calculator_canister_c2c_client::getPrice': {error:?}"),
                    Some(HashMap::from([
                        ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                        ("token_0_decimals".to_string(), token_0_decimals.to_string()),
                        ("token_1_decimals".to_string(), token_1_decimals.to_string()),
                        ("swap_calculator_canister".to_string(), ICPSWAP_SWAP_CALCULATOR_CANISTER_ID.to_text()),
                    ]))
                )
            })?;

        Ok(price)
    }

    async fn get_token_amount_by_liquidity(
        &self,
        sqrt_price_x96: Nat,
        tick_lower: Int,
        tick_upper: Int,
        liquidity: Nat
    ) -> Result<GetTokenAmountByLiquidityResponse, InternalError> {
        let (result,) = icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity(
            *ICPSWAP_SWAP_CALCULATOR_CANISTER_ID,
            (sqrt_price_x96.clone(), tick_lower.clone(), tick_upper.clone(), liquidity.clone())
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 33), // 1002 04 33
                    "ICPSwapProvider::get_token_amount_by_liquidity".to_string(),
                    format!("IC error calling 'icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity': {error:?}"),
                    Some(HashMap::from([
                        ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                        ("tick_lower".to_string(), tick_lower.to_string()),
                        ("tick_upper".to_string(), tick_upper.to_string()),
                        ("liquidity".to_string(), liquidity.to_string()),
                        ("swap_calculator_canister".to_string(), ICPSWAP_SWAP_CALCULATOR_CANISTER_ID.to_text()),
                    ]))
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 34), // 1002 03 34
                    "ICPSwapProvider::get_token_amount_by_liquidity".to_string(),
                    format!("Error calling 'icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity': {error:?}"),
                    Some(HashMap::from([
                        ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                        ("tick_lower".to_string(), tick_lower.to_string()),
                        ("tick_upper".to_string(), tick_upper.to_string()),
                        ("liquidity".to_string(), liquidity.to_string()),
                        ("swap_calculator_canister".to_string(), ICPSWAP_SWAP_CALCULATOR_CANISTER_ID.to_text()),
                    ]))
                )
            })?;

        Ok(result)
    }

    // ================ Node Index canister ================

    async fn get_all_tokens(
        &self,
    ) -> Result<Vec<TokenData>, InternalError> {
        let response = icpswap_node_index_canister_c2c_client::getAllTokens(
            *ICPSWAP_NODE_INDEX_CANISTER_ID
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 35), // 1002 04 35
                    "ICPSwapProvider::get_all_tokens".to_string(),
                    format!("IC error calling 'icpswap_node_index_canister_c2c_client::getAllTokens': {error:?}"),
                    Some(HashMap::from([
                        ("node_index_canister".to_string(), ICPSWAP_NODE_INDEX_CANISTER_ID.to_text()),
                    ]))
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 36), // 1002 03 36
                    "ICPSwapProvider::get_all_tokens".to_string(),
                    format!("Error calling 'icpswap_node_index_canister_c2c_client::getAllTokens': {error:?}"),
                    Some(HashMap::from([
                        ("node_index_canister".to_string(), ICPSWAP_NODE_INDEX_CANISTER_ID.to_text()),
                    ]))
                )
            })?;

        Ok(response)
    }

    async fn get_tvl_storage_canister(
        &self,
    ) -> Result<Vec<String>, InternalError> {
        let response = icpswap_global_index_canister_c2c_client::tvlStorageCanister(
            *ICPSWAP_GLOBAL_INDEX_CANISTER_ID
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 37), // 1002 04 37
                    "ICPSwap provider::get_tvl_storage_canister".to_string(),
                    format!("IC error calling 'icpswap_global_index_canister_c2c_client::tvlStorageCanister': {error:?}"),
                    Some(HashMap::from([
                        ("global_index_canister".to_string(), ICPSWAP_GLOBAL_INDEX_CANISTER_ID.to_text()),
                    ]))
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 38), // 1002 03 38
                    "ICPSwapProvider::get_tvl_storage_canister".to_string(),
                    format!("Error calling 'icpswap_global_index_canister_c2c_client::tvlStorageCanister': {error:?}"),
                    Some(HashMap::from([
                        ("global_index_canister".to_string(), ICPSWAP_GLOBAL_INDEX_CANISTER_ID.to_text()),
                    ]))
                )
            })?;

        Ok(response)
    }

    // ================ TVL Storage canister ================

    async fn get_pool_chart_tvl(
        &self,
        canister_id: CanisterId,
        pool_canister_id: String,
        offset: Nat,
        limit: Nat
    ) -> Result<Vec<PoolChartTvl>, InternalError> {
        let (result,) = icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl(
            canister_id.clone(),
            (pool_canister_id.clone(), offset.clone(), limit.clone())
        ).await
            .map_err(|error| {
                InternalError::external_service(
                    build_error_code(1002, 4, 39), // 1002 04 39
                    "ICPSwapProvider::get_pool_chart_tvl".to_string(),
                    format!("IC error calling 'icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("pool_canister_id".to_string(), pool_canister_id.clone()),
                        ("offset".to_string(), offset.to_string()),
                        ("limit".to_string(), limit.to_string()),
                    ]))
                )
            })
            .map_err(|error| {
                InternalError::business_logic(
                    build_error_code(1002, 3, 40), // 1002 03 40
                    "ICPSwapProvider::get_pool_chart_tvl".to_string(),
                    format!("Error calling 'icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl': {error:?}"),
                    Some(HashMap::from([
                        ("canister_id".to_string(), canister_id.to_text()),
                        ("pool_canister_id".to_string(), pool_canister_id),
                        ("offset".to_string(), offset.to_string()),
                        ("limit".to_string(), limit.to_string()),
                    ]))
                )
            })?;

        Ok(result)
    }
}
