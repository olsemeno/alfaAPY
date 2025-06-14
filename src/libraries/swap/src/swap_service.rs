use candid::Nat;
use std::collections::HashMap;

use icrc_ledger_canister::icrc2_approve::ApproveArgs;
use types::swap_tokens::{SuccessResult, QuoteResult};
use types::exchange_id::ExchangeId;
use providers::kongswap::KONGSWAP_CANISTER;
use types::CanisterId;

use crate::token_swaps::kongswap::KongSwapSwapClient;
use crate::token_swaps::icpswap::ICPSwapSwapClient;
use crate::token_swaps::swap_client::SwapClient;
use errors::internal_error::error::InternalError;

pub async fn swap_icrc2_optimal(
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<SuccessResult, InternalError> {
    let provider = quote_swap_icrc2_optimal(
        input_token.clone(),
        output_token.clone(),
        amount.clone()
    ).await?.provider;

    swap_icrc2(input_token, output_token, amount, provider).await
}

pub async fn swap_icrc2(
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
    provider: ExchangeId,
) -> Result<SuccessResult, InternalError> {
    match provider {
        ExchangeId::KongSwap => swap_icrc2_kongswap(input_token, output_token, amount).await,
        ExchangeId::ICPSwap => swap_icrc2_icpswap(input_token, output_token, amount).await,
        _ => Err(InternalError::business_logic(
            "swap_service::swap_icrc2".to_string(),
            "Invalid provider".to_string(),
            None,
            Some(HashMap::from([
                ("input_token".to_string(), input_token.to_text()),
                ("output_token".to_string(), output_token.to_text()),
                ("amount".to_string(), amount.to_string()),
                ("provider".to_string(), provider.to_string()),
            ])),
        )),
    }
}

pub async fn quote_swap_icrc2_optimal(
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<QuoteResult, InternalError> {
    let kong_quote = quote_swap_kongswap(input_token.clone(), output_token.clone(), amount).await;
    // let icp_quote = quote_swap_icpswap(input_token, output_token, amount).await;

    //Return the quote with the highest amount_out
    // std::cmp::max_by(
    //     kong_quote.unwrap(),
    //     icp_quote.unwrap(),
    //     |a, b| a.amount_out.cmp(&b.amount_out)
    // )

    // TODO: remove this after testing and return the quote with the highest amount_out
    Ok(kong_quote?)
}


// TODO: move to separate services for each provider

// TODO: make private
pub async fn swap_icrc2_icpswap(
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<SuccessResult, InternalError> {
    let swap_client = Box::new(
        ICPSwapSwapClient::new(
            input_token.clone(),
            output_token
        ).with_pool().await?
    );

    // ICRC2 APPROVE
    icrc_ledger_canister_c2c_client::icrc2_approve(
        input_token.clone(),
        &ApproveArgs {
            from_subaccount: None,
            spender: swap_client.canister_id().into(),
            amount: Nat::from(99999999999999 as u128), //TODO
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        },
    )
    .await
    .map_err(|error| {
        InternalError::external_service(
            "icrc_ledger_canister_c2c_client".to_string(),
            "swap_service::swap_icrc2_icpswap".to_string(),
            format!("Error calling 'icrc_ledger_canister_c2c_client::icrc2_approve': {error:?}"),
            None,
            Some(HashMap::from([
                ("input_token".to_string(), input_token.to_text()),
                ("output_token".to_string(), output_token.to_text()),
                ("amount".to_string(), amount.to_string()),
            ])),
        )
    })?
    .map_err(|error| {
        InternalError::business_logic(
            "swap_service::swap_icrc2_icpswap".to_string(),
            format!("Error calling 'icrc_ledger_canister_c2c_client::icrc2_approve': {error:?}"),
            None,
            Some(HashMap::from([
                ("input_token".to_string(), input_token.to_text()),
                ("output_token".to_string(), output_token.to_text()),
                ("amount".to_string(), amount.to_string()),
            ])),
        )
    })?;

    let swap_result = swap_client.swap(amount.clone()).await
        .map_err(|error| {
            error.wrap(
                "swap_service::swap_icrc2_icpswap".to_string(),
                "Error calling 'swap_client::swap'".to_string(),
                Some(HashMap::from([
                    ("input_token".to_string(), input_token.to_text()),
                    ("output_token".to_string(), output_token.to_text()),
                    ("amount".to_string(), amount.clone().to_string()),
                ])),
            )
        })?;

    Ok(SuccessResult {
        provider: ExchangeId::ICPSwap,
        amount_out: swap_result.amount_out,
    })
}

// TODO: make private
pub async fn swap_icrc2_kongswap(
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<SuccessResult, InternalError> {
    let swap_client = Box::new(
        KongSwapSwapClient::new(
            *KONGSWAP_CANISTER,
            input_token.clone(),
            output_token
        )
    );

    icrc_ledger_canister_c2c_client::icrc2_approve(
        input_token.clone(),
        &ApproveArgs {
            from_subaccount: None,
            spender: swap_client.canister_id().into(),
            amount: Nat::from(99999999999999 as u128), //TODO
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        },
    )
    .await
    .map_err(|error| {
        InternalError::external_service(
            "icrc_ledger_canister_c2c_client".to_string(),
            "swap_service::swap_icrc2_kongswap".to_string(),
            format!("Error calling 'icrc_ledger_canister_c2c_client::icrc2_approve': {error:?}"),
            None,
            Some(HashMap::from([
                ("input_token".to_string(), input_token.to_text()),
                ("amount".to_string(), amount.clone().to_string()),
            ])),
        )
    })?
    .map_err(|error| {
        InternalError::business_logic(
            "swap_service::swap_icrc2_kongswap".to_string(),
            format!("Error calling 'icrc_ledger_canister_c2c_client::icrc2_approve': {error:?}"),
            None,
            Some(HashMap::from([
                ("input_token".to_string(), input_token.to_text()),
                ("amount".to_string(), amount.clone().to_string()),
            ])),
        )
    })?;

    let swap_result = swap_client.swap(amount.clone()).await
        .map_err(|error| {
            error.wrap(
                "swap_service::swap_icrc2_kongswap".to_string(),
                "Error calling 'swap_client::swap'".to_string(),
                Some(HashMap::from([
                    ("input_token".to_string(), input_token.to_text()),
                    ("output_token".to_string(), output_token.to_text()),
                    ("amount".to_string(), amount.clone().to_string()),
                ])),
            )
        })?;

    Ok(SuccessResult {
        provider: ExchangeId::KongSwap,
        amount_out: swap_result.amount_out,
    })
}

// TODO: make private
pub async fn quote_swap_kongswap(
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<QuoteResult, InternalError> {
    let swap_client = Box::new(
        KongSwapSwapClient::new(
            *KONGSWAP_CANISTER,
            input_token.clone(),
            output_token.clone()
        )
    );

    let result = swap_client.quote(amount.clone()).await
        .map_err(|error| {
            error.wrap(
                "swap_service::quote_kongswap".to_string(),
                "Error calling 'swap_client::quote'".to_string(),
                Some(HashMap::from([
                    ("input_token".to_string(), input_token.to_text()),
                    ("output_token".to_string(), output_token.to_text()),
                    ("amount".to_string(), amount.to_string()),
                ])),
            )
        })?;

    Ok(QuoteResult {
        provider: ExchangeId::KongSwap,
        amount_out: result.amount_out,
    })
}

// TODO: make private
pub async fn quote_swap_icpswap(
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<QuoteResult, InternalError> {
    let swap_client = Box::new(
        ICPSwapSwapClient::new(
            input_token.clone(),
            output_token.clone()
        ).with_pool().await?
    );

    let result = swap_client.quote(amount.clone()).await
        .map_err(|error| {
            error.wrap(
                "swap_service::quote".to_string(),
                "Error calling 'swap_client::quote'".to_string(),
                Some(HashMap::from([
                    ("input_token".to_string(), input_token.to_text()),
                    ("output_token".to_string(), output_token.to_text()),
                    ("amount".to_string(), amount.to_string()),
                ])),
            )
        })?;

    Ok(QuoteResult {
        provider: ExchangeId::ICPSwap,
        amount_out: result.amount_out,
    })
}
