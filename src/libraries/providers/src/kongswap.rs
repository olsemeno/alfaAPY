use candid::{Nat, Principal};
use types::CanisterId;
use std::collections::HashMap;

use kongswap_canister::add_liquidity::{Args as AddLiquidityArgs, AddLiquidityReply};
use kongswap_canister::remove_liquidity::{Args as RemoveLiquidityArgs, RemoveLiquidityReply};
use kongswap_canister::remove_liquidity_amounts::{Args as RemoveLiquidityAmountsArgs, RemoveLiquidityAmountsReply};
use kongswap_canister::queries::pools::PoolReply;
use kongswap_canister::queries::add_liquidity_amounts::AddLiquidityAmountsReply;
use kongswap_canister::swap_amounts::SwapAmountsReply;
use kongswap_canister::user_balances::UserBalancesReply;
use kongswap_canister::swap::SwapReply;
use kongswap_canister::swap::Args as SwapArgs;
use icrc_ledger_client;
use errors::internal_error::error::InternalError;
use errors::internal_error::error::build_error_code;
use utils::constants::KONGSWAP_CANISTER_ID;

fn token_kongswap_format(token: CanisterId) -> String {
    format!("IC.{}", token.to_text())
}

pub async fn pools() -> Result<Vec<PoolReply>, InternalError> {
    kongswap_canister_c2c_client::pools(*KONGSWAP_CANISTER_ID).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1001, 4, 1), // 1001 04 01
                "KongSwapProvider::pools".to_string(),
                format!("IC error calling 'kongswap_canister_c2c_client::pools': {error:?}"),
                None
            )
        })?
        .map_err(|error_message| {
            InternalError::business_logic(
                build_error_code(1001, 3, 2), // 1001 03 02
                "KongSwapProvider::pools".to_string(),
                format!("Error calling 'kongswap_canister_c2c_client::pools': {error_message:?}"),
                None
            )
        })
}

pub async fn swap_amounts(
    token_in: CanisterId,
    amount: Nat,
    token_out: CanisterId,
) -> Result<SwapAmountsReply, InternalError> {

    let token_in = token_kongswap_format(token_in.clone());
    let token_out = token_kongswap_format(token_out.clone());

    let (result,) = kongswap_canister_c2c_client::swap_amounts(
        *KONGSWAP_CANISTER_ID,
        (token_in.clone(), amount.clone(), token_out.clone())
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1001, 4, 3), // 1001 04 03
                "KongSwapProvider::swap_amounts".to_string(),
                format!("IC error calling 'kongswap_canister_c2c_client::swap_amounts': {error:?}"),
                Some(HashMap::from([
                    ("token_in".to_string(), token_in.clone()),
                    ("token_out".to_string(), token_out.clone()),
                    ("amount".to_string(), amount.to_string()),
                ]))
            )
        })?;

    result.map_err(|error_message| {
        InternalError::business_logic(
            build_error_code(1001, 3, 4), // 1001 03 04
            "KongSwapProvider::swap_amounts".to_string(),
            format!("Error calling 'kongswap_canister_c2c_client::swap_amounts': {error_message:?}"),
            Some(HashMap::from([
                ("token_in".to_string(), token_in),
                ("token_out".to_string(), token_out),
                ("amount".to_string(), amount.to_string()),
            ]))
        )
    })
}

pub async fn swap(
    token_in: CanisterId,
    amount: Nat,
    token_out: CanisterId,
    max_slippage: Option<f64>
) -> Result<SwapReply, InternalError> {
    let args = SwapArgs {
        pay_amount: amount.into(),
        pay_token: token_kongswap_format(token_in.clone()),
        receive_token: token_kongswap_format(token_out.clone()),
        max_slippage,
    };

    let result = kongswap_canister_c2c_client::swap(
        *KONGSWAP_CANISTER_ID,
        &args
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1001, 4, 5), // 1001 04 05
                "KongSwapProvider::swap".to_string(),
                format!("Error calling 'kongswap_canister_c2c_client::swap': {error:?}"),
                Some(HashMap::from([
                    ("pay_amount".to_string(), args.pay_amount.to_string()),
                    ("pay_token".to_string(), args.pay_token.to_string()),
                    ("receive_token".to_string(), args.receive_token.to_string()),
                    ("max_slippage".to_string(), args.max_slippage.unwrap_or(0.0).to_string()),
                ])),
            )
        })?
        .map_err(|error| {
            InternalError::business_logic(
                build_error_code(1001, 3, 6), // 1001 03 06
                "KongSwapProvider::swap".to_string(),
                format!("Error calling 'kongswap_canister_c2c_client::swap': {error:?}"),
                Some(HashMap::from([
                    ("pay_amount".to_string(), args.pay_amount.to_string()),
                    ("pay_token".to_string(), args.pay_token.to_string()),
                    ("receive_token".to_string(), args.receive_token.to_string()),
                    ("max_slippage".to_string(), args.max_slippage.unwrap_or(0.0).to_string()),
                ])),
            )
        })?;

    Ok(result)
}

pub async fn add_liquidity_amounts(
    token_0: String,
    amount: Nat,
    token_1: String
) -> Result<AddLiquidityAmountsReply, InternalError> {
    let (result,) = kongswap_canister_c2c_client::add_liquidity_amounts(
        *KONGSWAP_CANISTER_ID,
        (token_0.clone(), amount.clone(), token_1.clone())
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1001, 4, 7), // 1001 04 07
                "KongSwapProvider::add_liquidity_amounts".to_string(),
                format!("IC error calling 'kongswap_canister_c2c_client::add_liquidity_amounts': {error:?}"),
                Some(HashMap::from([
                    ("token0".to_string(), token_0.clone()),
                    ("amount".to_string(), amount.to_string()),
                    ("token1".to_string(), token_1.clone()),
                ]))
            )
        })?;

    result.map_err(|error_message| {
        InternalError::business_logic(
            build_error_code(1001, 3, 8), // 1001 03 08
            "KongSwapProvider::add_liquidity_amounts".to_string(),
            format!("Error calling 'kongswap_canister_c2c_client::add_liquidity_amounts': {error_message:?}"),
            Some(HashMap::from([
                ("token0".to_string(), token_0),
                ("amount".to_string(), amount.to_string()),
                ("token1".to_string(), token_1),
            ]))
        )
    })
}

pub async fn add_liquidity(
    token_0: String, 
    amount_0: Nat, 
    token_1: String, 
    amount_1: Nat, 
    ledger0: Principal,
    ledger1: Principal
) -> Result<AddLiquidityReply, InternalError> {
    icrc_ledger_client::icrc2_approve(
        KONGSWAP_CANISTER_ID.clone().into(),
        ledger0,
        amount_0.clone()
    ).await?;

    icrc_ledger_client::icrc2_approve(
        KONGSWAP_CANISTER_ID.clone().into(),
        ledger1,
        amount_1.clone()
    ).await?;

    let args = AddLiquidityArgs {
        token_0: token_0.clone(),
        amount_0: amount_0.clone(),
        tx_id_0: None, //use icrc2
        token_1: token_1.clone(),
        amount_1: amount_1.clone(),
        tx_id_1: None,
    };

    let result = kongswap_canister_c2c_client::add_liquidity(
        *KONGSWAP_CANISTER_ID,
        &args
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1001, 4, 9), // 1001 04 09
                "KongSwapProvider::add_liquidity".to_string(),
                format!("IC error calling 'kongswap_canister_c2c_client::add_liquidity': {error:?}"),
                Some(HashMap::from([
                    ("token_0".to_string(), token_0.clone()),
                    ("amount_0".to_string(), amount_0.to_string()),
                    ("token_1".to_string(), token_1.clone()),
                    ("amount_1".to_string(), amount_1.to_string()),
                    ("ledger0".to_string(), ledger0.to_string()),
                    ("ledger1".to_string(), ledger1.to_string()),
                ]))
            )
        })?;

    result.map_err(|error_message| {
        InternalError::business_logic(
            build_error_code(1001, 3, 10), // 1001 03 10
            "KongSwapProvider::add_liquidity".to_string(),
            format!("Error calling 'kongswap_canister_c2c_client::add_liquidity': {error_message:?}"),
            Some(HashMap::from([
                ("token_0".to_string(), token_0),
                ("amount_0".to_string(), amount_0.to_string()),
                ("token_1".to_string(), token_1),
                ("amount_1".to_string(), amount_1.to_string()),
                ("ledger0".to_string(), ledger0.to_string()),
                ("ledger1".to_string(), ledger1.to_string()),
            ]))
        )
    })
}

pub async fn user_balances(principal_id: String) -> Result<Vec<UserBalancesReply>, InternalError> {
    let (result,) = kongswap_canister_c2c_client::user_balances(
        *KONGSWAP_CANISTER_ID,
        (principal_id.clone(),)
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1001, 4, 11), // 1001 04 11
                "KongSwapProvider::user_balances".to_string(),
                format!("IC error calling 'kongswap_canister_c2c_client::user_balances': {error:?}"),
                Some(HashMap::from([
                    ("principal_id".to_string(), principal_id.clone()),
                ]))
            )
        })?;

    result.map_err(|error_message| {
        InternalError::business_logic(
            build_error_code(1001, 3, 12), // 1001 03 12
            "KongSwapProvider::user_balances".to_string(),
            format!("Error calling 'kongswap_canister_c2c_client::user_balances': {error_message:?}"),
            Some(HashMap::from([
                ("principal_id".to_string(), principal_id),
            ]))
        )
    })
}

pub async fn remove_liquidity_amounts(
    token_0: String, 
    token_1: String, 
    remove_lp_token_amount: Nat
) -> Result<RemoveLiquidityAmountsReply, InternalError> {
    let args = RemoveLiquidityAmountsArgs {
        token_0: token_0.clone(),
        token_1: token_1.clone(),
        remove_lp_token_amount: remove_lp_token_amount.clone(),
    };

    let result = kongswap_canister_c2c_client::remove_liquidity_amounts(
        *KONGSWAP_CANISTER_ID,
        &args
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1001, 4, 13), // 1001 04 13
                "KongSwapProvider::remove_liquidity_amounts".to_string(),
                format!("IC error calling 'kongswap_canister_c2c_client::remove_liquidity_amounts': {error:?}"),
                Some(HashMap::from([
                    ("token_0".to_string(), token_0.clone()),
                    ("token_1".to_string(), token_1.clone()),
                    ("remove_lp_token_amount".to_string(), remove_lp_token_amount.to_string()),
                ]))
            )
        })?;

    result.map_err(|error_message| {
        InternalError::business_logic(
            build_error_code(1001, 3, 14), // 1001 03 14
            "KongSwapProvider::remove_liquidity_amounts".to_string(),
            format!("Error calling 'kongswap_canister_c2c_client::remove_liquidity_amounts': {error_message:?}"),
            Some(HashMap::from([
                ("token_0".to_string(), token_0),
                ("token_1".to_string(), token_1),
                ("remove_lp_token_amount".to_string(), remove_lp_token_amount.to_string()),
            ]))
        )
    })
}

pub async fn remove_liquidity(
    token_0: String, 
    token_1: String, 
    remove_lp_token_amount: Nat
) -> Result<RemoveLiquidityReply, InternalError> {
    let args = RemoveLiquidityArgs {
        token_0: token_0.clone(),
        token_1: token_1.clone(),
        remove_lp_token_amount: remove_lp_token_amount.clone(),
    };

    let result = kongswap_canister_c2c_client::remove_liquidity(
        *KONGSWAP_CANISTER_ID,
        &args
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1001, 4, 15), // 1001 04 15
                "KongSwapProvider::remove_liquidity".to_string(),
                format!("IC error calling 'kongswap_canister_c2c_client::remove_liquidity': {error:?}"),
                Some(HashMap::from([
                    ("token_0".to_string(), token_0.clone()),
                    ("token_1".to_string(), token_1.clone()),
                    ("remove_lp_token_amount".to_string(), remove_lp_token_amount.to_string()),
                ]))
            )
        })?;

    result.map_err(|error_message| {
        InternalError::business_logic(
            build_error_code(1001, 3, 16), // 1001 03 16
            "KongSwapProvider::remove_liquidity".to_string(),
            format!("Error calling 'kongswap_canister_c2c_client::remove_liquidity': {error_message:?}"),
            Some(HashMap::from([
                ("token_0".to_string(), token_0),
                ("token_1".to_string(), token_1),
                ("remove_lp_token_amount".to_string(), remove_lp_token_amount.to_string()),
            ]))
        )
    })
}
