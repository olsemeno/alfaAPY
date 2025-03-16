use crate::swap::token_swaps::kongswap::KongSwapClient;
use crate::swap::token_swaps::swap_client::SwapClient;
use candid::Nat;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_cdk::trap;
use icrc_ledger_canister::icrc2_approve::ApproveArgs;
use types::exchanges::TokenInfo;
use types::swap_tokens::SuccessResult;

pub const KONG_BE_CANISTER: CanisterId = CanisterId::from_slice(&[0, 0, 0, 0, 2, 48, 2, 23, 1, 1]);

pub(crate) async fn swap_icrc2_kong(
    input_token: TokenInfo,
    output_token: TokenInfo,
    amount: u128,
) -> SuccessResult {

    let swap_client =  Box::new(KongSwapClient::new(KONG_BE_CANISTER, input_token.clone(), output_token));
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
        Ok(Err(error)) => Err(format!("ICRC2 approve SWAP {error:?}")),
        Err(error) => Err(format!("ICRC2 approve SWAP {error:?}")),
    };

    match x {
        Ok(_) => {}
        Err(a) => {
            let c = input_token.ledger.to_text();
            trap(format!("ICRC2 approve SWAP aa {a:?} : {c:?}").as_str());
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
            let msg = format!("Swap error 1 : {error:?}");
            trap(msg.as_str());
        }
    };

    match swap_result {
        Ok(x) => {
            SuccessResult { amount_out: x.amount_out }
        }
        Err(e) => {
            let msg = format!("Swap error 2 : {e:?} arguments: {}", amount);
            trap(msg.as_str());
        }
    }
}