use candid::{Nat, Principal};
use ic_cdk::id;
use std::{collections::HashMap, convert::TryInto};

use icrc_ledger_canister::updates::icrc2_transfer_from::Args as Icrc2TransferFromArgs;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use icrc_ledger_canister::updates::icrc1_transfer::Response as Icrc1TransferResponse;
use canister_client;
use errors::internal_error::error::InternalError;
use errors::internal_error::error::build_error_code;
use types::CanisterId;

// TODO: move to separate library
pub async fn icrc2_transfer_from_user(
    user: Principal,
    canister_id: CanisterId,
    amount: Nat,
) -> Result<Nat, InternalError> {
    let transfer_args = Icrc2TransferFromArgs {
        spender_subaccount: None,
        from: Account { owner: user, subaccount: None },
        to: Account { owner: id(), subaccount: None },
        amount: amount.clone(),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    icrc_ledger_canister_c2c_client::icrc2_transfer_from(
        canister_id,
        &transfer_args,
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1100, 4, 3), // 1100 04 03
                "Utils::icrc2_transfer_from_user".to_string(),
                format!("IC error calling 'icrc_ledger_canister_c2c_client::icrc2_transfer_from': {error:?}"),
                Some(HashMap::from([
                    ("user".to_string(), user.to_string()),
                    ("canister_id".to_string(), canister_id.to_string()),
                    ("amount".to_string(), amount.to_string()),
                ]))
            )
        })?
        .map_err(|err| {
            InternalError::business_logic(
                build_error_code(1100, 3, 4), // 1100 03 04
                "Utils::icrc2_transfer_from_user".to_string(),
                format!("Error calling 'icrc_ledger_canister_c2c_client::icrc2_transfer_from': {err:?}"),
                Some(HashMap::from([
                    ("user".to_string(), user.to_string()),
                    ("canister_id".to_string(), canister_id.to_string()),
                    ("amount".to_string(), amount.to_string()),
                ]))
            )
        })
        .map(|block_index| block_index.0.try_into().unwrap())
}

pub async fn icrc1_transfer_to_user(
    user: Principal,
    canister_id: CanisterId,
    amount: Nat,
) -> Result<Nat, InternalError> {
    let args = TransferArg {
        from_subaccount: None,
        to: Account { owner: user, subaccount: None },
        fee: None,
        created_at_time: None,
        memo: None,
        amount: amount.clone(),
    };

    canister_client::make_c2c_call(
        canister_id,
        "icrc1_transfer",
        &args,
        ::candid::encode_one,
        |r| ::candid::decode_one::<Icrc1TransferResponse>(r)
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1200, 4, 1), // 1200 04 01
                "Utils::icrc1_transfer_to_user".to_string(),
                format!("IC error calling 'canister_client::make_c2c_call': {error:?}"),
                Some(HashMap::from([
                    ("user".to_string(), user.to_string()),
                    ("canister_id".to_string(), canister_id.to_string()),
                    ("amount".to_string(), amount.to_string()),
                ])),
            )
        })?
        .map_err(|err| {
            InternalError::business_logic(
                build_error_code(1200, 3, 2), // 1200 03 02
                "Utils::icrc1_transfer_to_user".to_string(),
                format!("Error calling 'canister_client::make_c2c_call': {err:?}"),
                Some(HashMap::from([
                    ("user".to_string(), user.to_string()),
                    ("canister_id".to_string(), canister_id.to_string()),
                    ("amount".to_string(), amount.to_string()),
                ])),
            )
        })
        .map(|response| response.0.try_into().unwrap())
}
