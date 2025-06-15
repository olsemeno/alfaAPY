use types::CanisterId;
use candid::{Nat, Principal, Int};
use once_cell::sync::Lazy;
use std::collections::HashMap;

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
use utils::util::principal_to_canister_id;
use utils::constants::*;
use errors::internal_error::error::InternalError;

pub static SWAP_FACTORY_CANISTER: Lazy<CanisterId> = Lazy::new(|| principal_to_canister_id("4mmnk-kiaaa-aaaag-qbllq-cai"));
pub static SWAP_CALCULATOR_CANISTER: Lazy<CanisterId> = Lazy::new(|| principal_to_canister_id("phr2m-oyaaa-aaaag-qjuoq-cai"));
pub static NODE_INDEX_CANISTER: Lazy<CanisterId> = Lazy::new(|| principal_to_canister_id("ggzvv-5qaaa-aaaag-qck7a-cai"));

pub const SWAP_FEE: u128 = 3000;
pub const ICRC2_TOKEN_STANDARD: &str = "ICRC2";
pub const ICP_TOKEN_STANDARD: &str = "ICP";

fn token_icpswap_format(token: &CanisterId) -> ICPSwapToken {
    let standard = match token.to_text().as_str() {
        ICP_TOKEN_CANISTER_ID => ICP_TOKEN_STANDARD.to_string(),
        _ => ICRC2_TOKEN_STANDARD.to_string(),
    };

    ICPSwapToken {
        address: token.to_text(),
        standard,
    }
}

// ================ Swap Factory canister ================

pub async fn get_pool(token_in: CanisterId, token_out: CanisterId) -> Result<ICPSwapPool, InternalError> {
    let pool_args = GetPoolArgs {
        fee: candid::Nat::from(SWAP_FEE as u128),
        token0: token_icpswap_format(&token_in),
        token1: token_icpswap_format(&token_out),
    };

    icpswap_swap_factory_canister_c2c_client::getPool(
        *SWAP_FACTORY_CANISTER,
        &pool_args
    ).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_factory_canister_c2c_client".to_string(),
                "ICPSwapProvider::get_pool".to_string(),
                format!("IC error calling 'icpswap_swap_factory_canister_c2c_client::getPool': {error:?}"),
                Some(HashMap::from([
                    ("token_in".to_string(), token_in.to_text()),
                    ("token_out".to_string(), token_out.to_text()),
                    ("fee".to_string(), pool_args.fee.to_string()),
                    ("swap_factory_canister".to_string(), SWAP_FACTORY_CANISTER.to_text()),
                ]))
            )
        })?
        .map_err(|error| {
            InternalError::business_logic(
                "ICPSwapProvider::get_pool".to_string(),
                format!("Error calling 'icpswap_swap_factory_canister_c2c_client::getPool': {error:?}"),
                Some(HashMap::from([
                    ("token_in".to_string(), token_in.to_text()),
                    ("token_out".to_string(), token_out.to_text()),
                    ("fee".to_string(), pool_args.fee.to_string()),
                    ("swap_factory_canister".to_string(), SWAP_FACTORY_CANISTER.to_text()),
                ]))
            )
        })
        .into_std()
}

// ================ Swap Pool canister ================

pub async fn quote(
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
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn swap(
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
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn get_token_meta(canister_id: CanisterId) -> Result<TokenMeta, InternalError> {
    icpswap_swap_pool_canister_c2c_client::getTokenMeta(canister_id).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_pool_canister_c2c_client".to_string(),
                "ICPSwapProvider::get_token_meta".to_string(),
                format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::getTokenMeta': {error:?}"),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                ]))
            )
        })?
        .map_err(|error| {
            InternalError::business_logic(
                "ICPSwapProvider::get_token_meta".to_string(),
                format!("Error calling 'icpswap_swap_pool_canister_c2c_client::getTokenMeta': {error:?}"),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                ]))
            )
        })
        .into_std()
}

pub async fn deposit_from(
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
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn withdraw(
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
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn metadata(canister_id: CanisterId) -> Result<Metadata, InternalError> {
    icpswap_swap_pool_canister_c2c_client::metadata(canister_id).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_pool_canister_c2c_client".to_string(),
                "ICPSwapProvider::metadata".to_string(),
                format!("IC error calling 'icpswap_swap_pool_canister_c2c_client::metadata': {error:?}"),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                ]))
            )
        })?
        .map_err(|error| {
            InternalError::business_logic(
                "ICPSwapProvider::metadata".to_string(),
                format!("Error calling 'icpswap_swap_pool_canister_c2c_client::metadata': {error:?}"),
                Some(HashMap::from([
                    ("canister_id".to_string(), canister_id.to_text()),
                ]))
            )
        })
        .into_std()
}

pub async fn mint(
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
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn get_user_position_ids_by_principal(
    canister_id: CanisterId,
    principal: Principal
) -> Result<Vec<Nat>, InternalError> {
    let (result,) = icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal(
        canister_id,
        (principal,)
    ).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn get_user_positions_by_principal(
    canister_id: CanisterId,
    principal: Principal
) -> Result<Vec<UserPositionWithId>, InternalError> {
    let (result,) = icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal(
        canister_id,
        (principal,)
    ).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn get_user_unused_balance(
    canister_id: CanisterId,
    principal: String,
) -> Result<UserUnusedBalance, InternalError> {
    let args = GetUserUnusedBalanceArgs {
        principal: principal.clone(),
    };

    icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance(canister_id, &args).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn increase_liquidity(
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
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn decrease_liquidity(
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
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn get_user_position(canister_id: CanisterId, position_id: Nat) -> Result<UserPosition, InternalError> {
    let args = (position_id.clone(),);
    let (result,) = icpswap_swap_pool_canister_c2c_client::getUserPosition(
        canister_id,
        args
    ).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn claim(
    canister_id: CanisterId,
    position_id: Nat,
) -> Result<ClaimResponse, InternalError> {
    let args = ClaimArgs {
        positionId: position_id.clone(),
    };

    icpswap_swap_pool_canister_c2c_client::claim(canister_id, &args).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_pool_canister_c2c_client".to_string(),
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

pub async fn get_price(
    sqrt_price_x96: Nat,
    token_0_decimals: Nat,
    token_1_decimals: Nat
) -> Result<f64, InternalError> {
    let (price,) = icpswap_swap_calculator_canister_c2c_client::getPrice(
        *SWAP_CALCULATOR_CANISTER,
        (sqrt_price_x96.clone(), token_0_decimals.clone(), token_1_decimals.clone())
    ).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_calculator_canister_c2c_client".to_string(),
                "ICPSwapProvider::get_price".to_string(),
                format!("IC error calling 'icpswap_swap_calculator_canister_c2c_client::getPrice': {error:?}"),
                Some(HashMap::from([
                    ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                    ("token_0_decimals".to_string(), token_0_decimals.to_string()),
                    ("token_1_decimals".to_string(), token_1_decimals.to_string()),
                    ("swap_calculator_canister".to_string(), SWAP_CALCULATOR_CANISTER.to_text()),
                ]))
            )
        })
        .map_err(|error| {
            InternalError::business_logic(
                "ICPSwapProvider::get_price".to_string(),
                format!("Error calling 'icpswap_swap_calculator_canister_c2c_client::getPrice': {error:?}"),
                Some(HashMap::from([
                    ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                    ("token_0_decimals".to_string(), token_0_decimals.to_string()),
                    ("token_1_decimals".to_string(), token_1_decimals.to_string()),
                    ("swap_calculator_canister".to_string(), SWAP_CALCULATOR_CANISTER.to_text()),
                ]))
            )
        })?;

    Ok(price)
}

pub async fn get_token_amount_by_liquidity(
    sqrt_price_x96: Nat,
    tick_lower: Int,
    tick_upper: Int,
    liquidity: Nat
) -> Result<GetTokenAmountByLiquidityResponse, InternalError> {
    let (result,) = icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity(
        *SWAP_CALCULATOR_CANISTER,
        (sqrt_price_x96.clone(), tick_lower.clone(), tick_upper.clone(), liquidity.clone())
    ).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_swap_calculator_canister_c2c_client".to_string(),
                "ICPSwapProvider::get_token_amount_by_liquidity".to_string(),
                format!("IC error calling 'icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity': {error:?}"),
                Some(HashMap::from([
                    ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                    ("tick_lower".to_string(), tick_lower.to_string()),
                    ("tick_upper".to_string(), tick_upper.to_string()),
                    ("liquidity".to_string(), liquidity.to_string()),
                    ("swap_calculator_canister".to_string(), SWAP_CALCULATOR_CANISTER.to_text()),
                ]))
            )
        })
        .map_err(|error| {
            InternalError::business_logic(
                "ICPSwapProvider::get_token_amount_by_liquidity".to_string(),
                format!("Error calling 'icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity': {error:?}"),
                Some(HashMap::from([
                    ("sqrt_price_x96".to_string(), sqrt_price_x96.to_string()),
                    ("tick_lower".to_string(), tick_lower.to_string()),
                    ("tick_upper".to_string(), tick_upper.to_string()),
                    ("liquidity".to_string(), liquidity.to_string()),
                    ("swap_calculator_canister".to_string(), SWAP_CALCULATOR_CANISTER.to_text()),
                ]))
            )
        })?;

    Ok(result)
}

// ================ Node Index canister ================

pub async fn get_all_tokens() -> Result<Vec<TokenData>, InternalError> {
    let response = icpswap_node_index_canister_c2c_client::getAllTokens(
        *NODE_INDEX_CANISTER
    ).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_node_index_canister_c2c_client".to_string(),
                "ICPSwapProvider::get_all_tokens".to_string(),
                format!("IC error calling 'icpswap_node_index_canister_c2c_client::getAllTokens': {error:?}"),
                Some(HashMap::from([
                    ("node_index_canister".to_string(), NODE_INDEX_CANISTER.to_text()),
                ]))
            )
        })
        .map_err(|error| {
            InternalError::business_logic(
                "ICPSwapProvider::get_all_tokens".to_string(),
                format!("Error calling 'icpswap_node_index_canister_c2c_client::getAllTokens': {error:?}"),
                Some(HashMap::from([
                    ("node_index_canister".to_string(), NODE_INDEX_CANISTER.to_text()),
                ]))
            )
        })?;

    Ok(response)
}

pub async fn get_tvl_storage_canister() -> Result<Vec<String>, InternalError> {
    let response = icpswap_node_index_canister_c2c_client::tvlStorageCanister(
        *NODE_INDEX_CANISTER
    ).await
        .map_err(|error| {
            InternalError::external_service(
                "icpswap_node_index_canister_c2c_client".to_string(),
                "ICPSwap provider::get_tvl_storage_canister".to_string(),
                format!("IC error calling 'icpswap_node_index_canister_c2c_client::tvlStorageCanister': {error:?}"),
                Some(HashMap::from([
                    ("node_index_canister".to_string(), NODE_INDEX_CANISTER.to_text()),
                ]))
            )
        })
        .map_err(|error| {
            InternalError::business_logic(
                "ICPSwapProvider::get_tvl_storage_canister".to_string(),
                format!("Error calling 'icpswap_node_index_canister_c2c_client::tvlStorageCanister': {error:?}"),
                Some(HashMap::from([
                    ("node_index_canister".to_string(), NODE_INDEX_CANISTER.to_text()),
                ]))
            )
        })?;

    Ok(response)
}

// ================ TVL Storage canister ================

pub async fn get_pool_chart_tvl(
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
                "icpswap_tvl_storage_canister_c2c_client".to_string(),
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
