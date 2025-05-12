use candid::Nat;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_cdk::trap;
use icrc_ledger_canister::icrc2_approve::ApproveArgs;

use types::exchanges::TokenInfo;
use types::swap_tokens::SuccessResult;

use crate::swap::token_swaps::kongswap::KongSwapClient;
use crate::swap::token_swaps::icpswap::ICPSwapClient;
use crate::swap::token_swaps::swap_client::SwapClient;

pub const KONG_BE_CANISTER: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 2, 48, 2, 23, 1, 1]);
pub const ICPSWAP_SWAP_POOL_CANISTER: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 2, 48, 2, 23, 1, 1]); // TODO: Fix canister id

pub(crate) async fn swap_icrc2(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> SuccessResult {
    let kongswap_client = Box::new(
        KongSwapClient::new(
            KONG_BE_CANISTER,
            input_token.clone(),
            output_token.clone()
        )
    );

    let icpswap_client = Box::new(
        ICPSwapClient::new(
            ICPSWAP_SWAP_POOL_CANISTER,
            input_token.clone(),
            output_token.clone()
        )
    );

    // Fetch KongSwap quote
    let kongswap_quote_result = match kongswap_client.quote(amount).await {
        Ok(result) => match result {
            Ok(quote) => Ok(quote),
            Err(e) => Err(format!("KongSwap quote error: {:?}", e)),
        },
        Err(e) => Err(format!("KongSwap quote failed: {:?}", e)),
    };

    // Fetch ICPSwap quote
    let icpswap_quote_result = match icpswap_client.quote(amount).await {
        Ok(result) => match result {
            Ok(quote) => Ok(quote),
            Err(e) => Err(format!("ICPSwap quote error: {:?}", e)),
        },
        Err(e) => Err(format!("ICPSwap quote failed: {:?}", e)),
    };

    match (kongswap_quote_result, icpswap_quote_result) {
        (Ok(kong_quote), Ok(icp_quote)) => {
            if kong_quote.amount_out > icp_quote.amount_out {
                swap_icrc2_kong(input_token, output_token, amount).await
            } else {
                swap_icrc2_icpswap(input_token, output_token, amount).await
            }
        },
        (Ok(_), Err(_)) => {
            // Only KongSwap gave a result
            swap_icrc2_kong(input_token, output_token, amount).await
        },
        (Err(_), Ok(_)) => {
            // Only ICPSwap gave a result
            swap_icrc2_icpswap(input_token, output_token, amount).await
        },
        (Err(kong_err), Err(icp_err)) => {
            trap(format!("Both quote services failed. KongSwap: {}, ICPSwap: {}", kong_err, icp_err).as_str());
        }
    }
}

pub(crate) async fn swap_icrc2_icpswap(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> SuccessResult {
    let swap_client = Box::new(
        ICPSwapClient::new(
            ICPSWAP_SWAP_POOL_CANISTER,
            input_token.clone(),
            output_token
        )
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

    let swap_result = match swap_client
        .swap(amount)
        .await
    {
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
            SuccessResult { amount_out: x.amount_out }
        }
        Err(e) => {
            let msg = format!("Swap error 2 (ICPSWAP): {e:?}");
            trap(msg.as_str());
        }
    }
}

pub(crate) async fn swap_icrc2_kong(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> SuccessResult {

    let swap_client = Box::new(
        KongSwapClient::new(
            KONG_BE_CANISTER,
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

    let swap_result = match swap_client
        .swap(amount)
        .await
    {
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
            SuccessResult { amount_out: x.amount_out }
        }
        Err(e) => {
            let msg = format!("Swap error 2 (KONGSWAP): {e:?} arguments: {}", amount);
            trap(msg.as_str());
        }
    }
}
