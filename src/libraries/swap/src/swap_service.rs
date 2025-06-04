use candid::Nat;
use ic_cdk::trap;

use icrc_ledger_canister::icrc2_approve::ApproveArgs;
use types::exchanges::TokenInfo;
use types::swap_tokens::{SuccessResult, QuoteResult};
use types::exchange_id::ExchangeId;
use providers::kongswap::KONGSWAP_CANISTER;

use crate::token_swaps::kongswap::KongSwapClient;
use crate::token_swaps::icpswap::ICPSwapClient;
use crate::token_swaps::swap_client::SwapClient;

pub async fn swap_icrc2_optimal(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> SuccessResult {
    let provider = quote_swap_icrc2_optimal(input_token.clone(), output_token.clone(), amount).await.provider;
    swap_icrc2(input_token, output_token, amount, provider).await
}

pub async fn swap_icrc2(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
    provider: ExchangeId,
) -> SuccessResult {
    match provider {
        ExchangeId::KongSwap => {
            swap_icrc2_kongswap(input_token, output_token, amount).await
        },
        ExchangeId::ICPSwap => {
            swap_icrc2_icpswap(input_token, output_token, amount).await
        },
        _ => {
            trap("Quote services failed.");
        },
    }
}

pub async fn quote_swap_icrc2_optimal(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> QuoteResult {
    let kong_quote = quote_swap_kongswap(input_token.clone(), output_token.clone(), amount).await;
    let icp_quote = quote_swap_icpswap(input_token, output_token, amount).await;

    // Return the quote with the highest amount_out
    std::cmp::max_by(
        kong_quote,
        icp_quote,
        |a, b| a.amount_out.cmp(&b.amount_out)
    )
}


// TODO: move to separate services for each provider

// TODO: make private
pub async fn swap_icrc2_icpswap(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> SuccessResult {
    let swap_client = Box::new(
        ICPSwapClient::new(
            input_token.clone(),
            output_token
        ).await
    );

    // ICRC2 APPROVE
    let approve_result = match icrc_ledger_canister_c2c_client::icrc2_approve(
        input_token.ledger.clone(),
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
    {
        Ok(Ok(index)) => Ok(index),
        Ok(Err(error)) => Err(format!("ICRC2 approve SWAP (ICPSWAP) {error:?}")),
        Err(error) => Err(format!("ICRC2 approve SWAP (ICPSWAP) {error:?}")),
    };

    match approve_result {
        Ok(_) => {}
        Err(a) => {
            let c = input_token.ledger.to_text();
            trap(format!("ICRC2 approve SWAP (ICPSWAP) {a:?} : {c:?}").as_str());
        }
    }

    let swap_result = match swap_client.swap(amount).await {
        Ok(r) => {
            r
        }
        Err(error) => {
            let msg = format!("Swap error 1 (ICPSWAP): {error:?}");
            trap(msg.as_str());
        }
    };

    match swap_result {
        Ok(x) => {
            SuccessResult {
                provider: ExchangeId::ICPSwap,
                amount_out: x.amount_out
            }
        }
        Err(e) => {
            let msg = format!("Swap error 2 (ICPSWAP): {e:?}");
            trap(msg.as_str());
        }
    }
}

// TODO: make private
pub async fn swap_icrc2_kongswap(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> SuccessResult {
    let swap_client = Box::new(
        KongSwapClient::new(
            *KONGSWAP_CANISTER,
            input_token.clone(),
            output_token
        )
    );

    let x = match icrc_ledger_canister_c2c_client::icrc2_approve(
        input_token.ledger.clone(),
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
    {
        Ok(Ok(index)) => Ok(index),
        Ok(Err(error)) => Err(format!("ICRC2 approve SWAP (KONGSWAP) {error:?}")),
        Err(error) => Err(format!("ICRC2 approve SWAP (KONGSWAP) {error:?}")),
    };

    match x {
        Ok(_) => {}
        Err(a) => {
            let c = input_token.ledger.to_text();
            trap(format!("ICRC2 approve SWAP (KONGSWAP) {a:?} : {c:?}").as_str());
        }
    }

    let swap_result = match swap_client.swap(amount).await {
        Ok(r) => {
            r
        }
        Err(error) => {
            let msg = format!("Swap error 1 (KONGSWAP): {error:?}");
            trap(msg.as_str());
        }
    };

    match swap_result {
        Ok(x) => {
            SuccessResult {
                provider: ExchangeId::KongSwap,
                amount_out: x.amount_out
            }
        }
        Err(e) => {
            let msg = format!("Swap error 2 (KONGSWAP): {e:?} arguments: {}", amount);
            trap(msg.as_str());
        }
    }
}

// TODO: make private
pub async fn quote_swap_kongswap(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> QuoteResult {
    let swap_client = Box::new(
        KongSwapClient::new(
            *KONGSWAP_CANISTER,
            input_token.clone(),
            output_token.clone()
        )
    );

    let quote_result = match swap_client.quote(amount).await {
        Ok(result) => match result {
            Ok(quote) => Ok(quote),
            Err(e) => Err(format!("KongSwap quote error: {:?}", e)),
        },
        Err(e) => Err(format!("KongSwap quote failed: {:?}", e)),
    };

    match quote_result {
        Ok(quote) => {
            QuoteResult {
                provider: ExchangeId::KongSwap,
                amount_out: quote.amount_out
            }
        }
        Err(e) => {
            let msg = format!("KongSwap quote error: {e:?}");
            trap(msg.as_str());
        }
    }
}

// TODO: make private
pub async fn quote_swap_icpswap(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> QuoteResult {
    let swap_client = Box::new(
        ICPSwapClient::new(
            input_token.clone(),
            output_token.clone()
        ).await
    );

    let quote_result = match swap_client.quote(amount).await {
        Ok(result) => match result {
            Ok(quote) => Ok(quote),
            Err(e) => Err(format!("ICPSwap quote error: {:?}", e)),
        },
        Err(e) => Err(format!("ICPSwap quote failed: {:?}", e)),
    };

    match quote_result {
        Ok(quote) => {
            QuoteResult {
                provider: ExchangeId::ICPSwap,
                amount_out: quote.amount_out
            }
        }
        Err(e) => {
            let msg = format!("ICPSwap quote error: {e:?}");
            trap(msg.as_str());
        }
    }
}
