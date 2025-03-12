use crate::swap::swap_service::KONG_BE_CANISTER;
use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::{call, trap};
use icrc_ledger_types::icrc2::approve::ApproveArgs;
use kongswap_canister::add_liquidity::{Args, Response as AddLiquidityResponse};
use kongswap_canister::add_liquidity_amounts::AddLiquidityAmountsReply;
use kongswap_canister::pools::Response as PoolsResponse;
use kongswap_canister::queries::add_liquidity_amounts::Response as AddLiquidityAmountsResponse;
use kongswap_canister::swap_amounts::Response as SwapAmountsResponse;
use kongswap_canister::user_balances::{Args as UserBalancesArgs, UserBalancesReply};
use kongswap_canister::user_balances::Response as UserBalancesResponse;

pub async fn pools() -> PoolsResponse {
    kongswap_canister_c2c_client::pools(KONG_BE_CANISTER).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the pools call: {}: {}",
            code as u8, msg
        ).as_str())
    })
}


pub async fn swap_amounts(pay_token: String, pay_amount: Nat, receive_token: String) -> SwapAmountsResponse {
    kongswap_canister_c2c_client::swap_amounts(KONG_BE_CANISTER, (pay_token, pay_amount, receive_token)).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the swap_amounts call: {}: {}",
            code as u8, msg
        ).as_str())
    })
}

pub async fn add_liquidity_amounts(token_0: String, amount: Nat, token_1: String) -> AddLiquidityAmountsResponse {
    kongswap_canister_c2c_client::add_liquidity_amounts(KONG_BE_CANISTER, (token_0, amount, token_1)).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the swap_amounts call: {}: {}",
            code as u8, msg
        ).as_str())
    })
}

pub async fn add_liquidity(token_0: String, amount_0: Nat, token_1: String, amount_1: Nat, ledger1: Principal,ledger2: Principal) -> AddLiquidityResponse {
// "ICP" Nat(4691453851855749120) "ckUSDT" Nat(4672478016327122944)'.
//     trap(format!("AddLiquidityArgs: {:?} {:?} {:?} {:?}", token_0, amount_0, token_1, amount_1).as_str());


    let x = match icrc_ledger_canister_c2c_client::icrc2_approve(
        ledger1,
        &ApproveArgs {
            from_subaccount: None,
            spender: KONG_BE_CANISTER.into(),
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
        Ok(Err(error)) => Err(format!("ICRC2 approve {error:?}")),
        Err(error) => Err(format!("ICRC2 approve  {error:?}")),
    };

    match x {
        Ok(_) => {}
        Err(a) => {
            trap(format!("ICRC2 approve  {a:?}").as_str());
        }
    }

    let x2 = match icrc_ledger_canister_c2c_client::icrc2_approve(
        ledger2,
        &ApproveArgs {
            from_subaccount: None,
            spender: KONG_BE_CANISTER.into(),
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
        Ok(Err(error)) => Err(format!("ICRC2 approve {error:?}")),
        Err(error) => Err(format!("ICRC2 approve  {error:?}")),
    };

    match x2 {
        Ok(_) => {}
        Err(a) => {
            trap(format!("ICRC2 approve  {a:?}").as_str());
        }
    }

    kongswap_canister_c2c_client::add_liquidity(KONG_BE_CANISTER, &Args {
        token_0,
        amount_0,
        tx_id_0: None, //use icrc2
        token_1,
        amount_1,
        tx_id_1: None,
    }).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the add_liquidity call: {}: {}",
            code as u8, msg
        ).as_str())
    }
    )
}
pub async fn user_balances(principal_id: String) -> (Result<Vec<UserBalancesReply>, String>,) {
    kongswap_canister_c2c_client::user_balances(KONG_BE_CANISTER, (principal_id,)).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the user_balances call: {}: {}",
            code as u8, msg
        ).as_str())
    }
    )
}

pub async fn requests(request_id: Option<u64>) -> kongswap_canister::queries::requests::Response {
    kongswap_canister_c2c_client::requests(KONG_BE_CANISTER, &kongswap_canister::queries::requests::Args {
        request_id
    } ).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the requests call: {}: {}",
            code as u8, msg
        ).as_str())
    }
    )
}

pub async fn remove_liquidity_amounts(token_0: String, token_1: String, remove_lp_token_amount: Nat) -> kongswap_canister::remove_liquidity_amounts::Response {
    kongswap_canister_c2c_client::remove_liquidity_amounts(KONG_BE_CANISTER, &kongswap_canister::remove_liquidity_amounts::Args {
        token_0,
        token_1,
        remove_lp_token_amount,
    }).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the remove_liquidity_amounts call: {}: {}",
            code as u8, msg
        ).as_str())
    }
    )
}

pub async fn remove_liquidity(token_0: String, token_1: String, remove_lp_token_amount: Nat) -> kongswap_canister::remove_liquidity::Response {
    kongswap_canister_c2c_client::remove_liquidity(KONG_BE_CANISTER, &kongswap_canister::remove_liquidity::Args {
        token_0,
        token_1,
        remove_lp_token_amount,
    }).await.unwrap_or_else(|(code, msg)| {
        trap(format!(
            "An error happened during the remove_liquidity call: {}: {}",
            code as u8, msg
        ).as_str())
    }
    )
}