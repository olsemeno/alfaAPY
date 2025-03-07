use crate::token_swaps::kongswap::KongSwapClient;
use crate::token_swaps::swap_client::SwapClient;
use candid::{Nat, Principal};
use ic_cdk::api::management_canister::main::CanisterId;
use icrc_ledger_canister::icrc2_approve::ApproveArgs;
use types::exchanges::TokenInfo;
use types::swap_tokens::Response::{InternalError, Success, SwapFailed};
use types::swap_tokens::{Response, SuccessResult};

pub const KONG_BE_CANISTER: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 2, 48, 2, 23, 1, 1]);
pub const SNS_GOVERNANCE_CANISTER_ID: CanisterId = Principal::from_slice(&[0, 0, 0, 0, 2, 0, 0, 24, 1, 1]);

pub(crate) async fn swap_icrc2_kong(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: Nat
) -> Response {

    let swap_client =  Box::new(KongSwapClient::new(KONG_BE_CANISTER, input_token.clone(), output_token));

    let _ = match icrc_ledger_canister_c2c_client::icrc2_approve(
        input_token.ledger.clone(),
        &ApproveArgs {
            from_subaccount: None,
            spender: swap_client.canister_id().into(),
            amount,
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
        Ok(Err(error)) => Err(format!("{error:?}")),
        Err(error) => Err(format!("{error:?}")),
    };

    let swap_result = match swap_client
        .swap(1, 0)
        .await
    {
        Ok(r) => {
            r
        }
        Err(error) => {
            let msg = format!("{error:?}");
            return  InternalError(msg)
        }
    };


    match swap_result {
        Ok(x) => {
            Success(SuccessResult { amount_out: x.amount_out })
        }
        Err(_) => {
            SwapFailed
        }
    }
}