use candid::Nat;
use std::collections::HashMap;
use std::sync::Arc;

use types::swap_tokens::{SwapResponse, QuoteResponse};
use types::exchange_id::ExchangeId;
use utils::constants::KONGSWAP_CANISTER_ID;
use types::CanisterId;
use errors::internal_error::error::{InternalError, build_error_code};
use icrc_ledger_client;
use providers::kongswap::KongSwapProvider;
use providers::icpswap::ICPSwapProvider;
use providers::providers_factory::ProviderImpls;

use crate::token_swaps::kongswap::KongSwapSwapClient;
use crate::token_swaps::icpswap::ICPSwapSwapClient;
use crate::token_swaps::swap_client::SwapClient;

pub async fn swap_icrc2_optimal(
    provider_impls: ProviderImpls,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<SwapResponse, InternalError> {
    let provider = quote_swap_icrc2_optimal(
        provider_impls.clone(),
        input_token.clone(),
        output_token.clone(),
        amount.clone()
    ).await?.provider;

    swap_icrc2(
        provider_impls,
        input_token,
        output_token,
        amount,
        provider
    ).await
}

pub async fn swap_icrc2(
    provider_impls: ProviderImpls,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
    provider: ExchangeId,
) -> Result<SwapResponse, InternalError>
{
    match provider {
        ExchangeId::KongSwap => {
            swap_icrc2_kongswap(provider_impls.kongswap, input_token, output_token, amount).await
        }
        ExchangeId::ICPSwap => {
            swap_icrc2_icpswap(provider_impls.icpswap, input_token, output_token, amount).await
        }
        _ => Err(InternalError::business_logic(
            build_error_code(2000, 3, 1),
            "swap_service::swap_icrc2".to_string(),
            "Invalid provider".to_string(),
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
    provider_impls: ProviderImpls,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<QuoteResponse, InternalError>
{
    let kong_quote = quote_swap_kongswap(
        provider_impls.kongswap,
        input_token.clone(),
        output_token.clone(),
        amount
    ).await;
    // let icp_quote = quote_swap_icpswap(
    //     icpswap_provider,
    //     input_token.clone(),
    //     output_token.clone(),
    //     amount
    // ).await;

    //Return the quote with the highest amount_out
    // std::cmp::max_by(
    //     kong_quote.unwrap(),
    //     icp_quote.unwrap(),
    //     |a, b| a.amount_out.cmp(&b.amount_out)
    // )

    // TODO: remove this after testing and return the quote with the highest amount_out
    Ok(kong_quote?)
}

pub async fn quote_swap_icrc2(
    provider_impls: ProviderImpls,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
    provider: ExchangeId,
) -> Result<QuoteResponse, InternalError>
{
    match provider {
        ExchangeId::KongSwap => {
            quote_swap_kongswap(provider_impls.kongswap, input_token, output_token, amount).await
        }
        ExchangeId::ICPSwap => {
            quote_swap_icpswap(provider_impls.icpswap, input_token, output_token, amount).await
        }
        _ => Err(InternalError::business_logic(
            build_error_code(2000, 3, 2),
            "swap_service::quote_swap_icrc2".to_string(),
            "Invalid provider".to_string(),
            Some(HashMap::from([
                ("input_token".to_string(), input_token.to_text()),
                ("output_token".to_string(), output_token.to_text()),
                ("amount".to_string(), amount.to_string()),
                ("provider".to_string(), provider.to_string()),
            ])),
        )),
    }
}

// TODO: make private
pub async fn swap_icrc2_kongswap(
    provider_impl: Arc<dyn KongSwapProvider + Send + Sync>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<SwapResponse, InternalError>
{
    let swap_client = Box::new(
        KongSwapSwapClient::new(
            provider_impl,
            *KONGSWAP_CANISTER_ID,
            input_token.clone(),
            output_token
        )
    );

    icrc_ledger_client::icrc2_approve(
        swap_client.canister_id(),
        input_token.clone(),
        amount.clone()
    ).await?;

    let swap_result = swap_client.swap(amount.clone()).await?;

    Ok(SwapResponse {
        provider: ExchangeId::KongSwap,
        amount_out: swap_result.amount_out,
    })
}

// TODO: make private
pub async fn swap_icrc2_icpswap(
    provider_impl: Arc<dyn ICPSwapProvider + Send + Sync>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<SwapResponse, InternalError>
{
    let swap_client = Box::new(
        ICPSwapSwapClient::new(
            provider_impl,
            input_token.clone(),
            output_token
        ).with_pool().await?
    );

    icrc_ledger_client::icrc2_approve(
        swap_client.canister_id(),
        input_token.clone(),
        amount.clone()
    ).await?;

    let swap_result = swap_client.swap(amount.clone()).await?;

    Ok(SwapResponse {
        provider: ExchangeId::ICPSwap,
        amount_out: swap_result.amount_out,
    })
}

// TODO: make private
pub async fn quote_swap_kongswap(
    provider_impl: Arc<dyn KongSwapProvider + Send + Sync>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<QuoteResponse, InternalError>
{
    let swap_client = Box::new(
        KongSwapSwapClient::new(
            provider_impl,
            *KONGSWAP_CANISTER_ID,
            input_token.clone(),
            output_token.clone()
        )
    );

    let result = swap_client.quote(amount.clone()).await?;

    Ok(QuoteResponse {
        provider: ExchangeId::KongSwap,
        amount_out: result.amount_out,
    })
}

// TODO: make private
pub async fn quote_swap_icpswap(
    provider_impl: Arc<dyn ICPSwapProvider + Send + Sync>,
    input_token: CanisterId,
    output_token: CanisterId,
    amount: Nat,
) -> Result<QuoteResponse, InternalError>
{
    let swap_client = Box::new(
        ICPSwapSwapClient::new(
            provider_impl,
            input_token.clone(),
            output_token.clone()
        ).with_pool().await?
    );

    let result = swap_client.quote(amount.clone()).await?;

    Ok(QuoteResponse {
        provider: ExchangeId::ICPSwap,
        amount_out: result.amount_out,
    })
}
