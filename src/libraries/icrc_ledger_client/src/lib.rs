use candid::{Principal, Nat};
use ic_cdk::id;
use std::collections::HashMap;

use ::types::CanisterId;
use errors::internal_error::error::InternalError;
use errors::internal_error::error::build_error_code;
use icrc_ledger_canister::icrc2_approve::ApproveArgs;
use icrc_ledger_canister::updates::icrc2_transfer_from::Args as Icrc2TransferFromArgs;
use icrc_ledger_types::icrc1::account::Account;

pub async fn icrc1_decimals(ledger_canister_id: CanisterId) -> Result<u8, InternalError> {
    icrc_ledger_canister_c2c_client::icrc1_decimals(ledger_canister_id).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1100, 4, 1), // 1100 04 01
                "icrc_ledger_canister_c2c_client::icrc1_decimals".to_string(),
                format!("IC error calling 'icrc_ledger_canister_c2c_client::icrc1_decimals': {error:?}"),
                Some(HashMap::from([
                    ("ledger_canister_id".to_string(), ledger_canister_id.to_text()), 
                ]))
            )
        })
}

pub async fn icrc2_approve(spender: Principal, ledger_canister_id: CanisterId, amount: Nat) -> Result<Nat, InternalError> {
    let args = ApproveArgs {
        from_subaccount: None,
            spender: spender.into(),
            amount: Nat::from(99999999999999 as u128), //TODO: amount + fee
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
    };

    icrc_ledger_canister_c2c_client::icrc2_approve(
        ledger_canister_id.clone(),
        &args,
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1100, 4, 2), // 1100 04 02
                "icrc_ledger_canister_c2c_client::icrc2_approve".to_string(),
                format!("IC error calling 'icrc_ledger_canister_c2c_client::icrc2_approve': {error:?}"),
                Some(HashMap::from([
                    ("spender".to_string(), spender.to_text()),
                    ("ledger_canister_id".to_string(), ledger_canister_id.to_text()),
                    ("amount".to_string(), amount.to_string()),
                ]))
            )
        })?
        .map_err(|error| {
            InternalError::business_logic(
                build_error_code(1100, 3, 3), // 1100 03 03
                "icrc_ledger_canister_c2c_client::icrc2_approve".to_string(),
                format!("Error calling 'icrc_ledger_canister_c2c_client::icrc2_approve': {error:?}"),
                Some(HashMap::from([
                    ("spender".to_string(), spender.to_text()),
                    ("ledger_canister_id".to_string(), ledger_canister_id.to_text()),
                    ("amount".to_string(), amount.to_string()),
                ]))
            )
        })
}

pub async fn icrc2_transfer_from(
    from: Principal,
    ledger_canister_id: CanisterId,
    amount: Nat,
) -> Result<Nat, InternalError> {
    let args = Icrc2TransferFromArgs {
        spender_subaccount: None,
        from: Account { owner: from, subaccount: None },
        to: Account { owner: id(), subaccount: None },
        amount: amount.clone(),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    icrc_ledger_canister_c2c_client::icrc2_transfer_from(
        ledger_canister_id,
        &args,
    ).await
        .map_err(|error| {
            InternalError::external_service(
                build_error_code(1100, 4, 4), // 1100 04 04
                "icrc_ledger_canister_c2c_client::icrc2_transfer_from".to_string(),
                format!("IC error calling 'icrc_ledger_canister_c2c_client::icrc2_transfer_from': {error:?}"),
                Some(HashMap::from([
                    ("from".to_string(), from.to_string()),
                    ("ledger_canister_id".to_string(), ledger_canister_id.to_string()),
                    ("amount".to_string(), amount.to_string()),
                ]))
            )
        })?
        .map_err(|err| {
            InternalError::business_logic(
                build_error_code(1100, 3, 5), // 1100 03 05
                "icrc_ledger_canister_c2c_client::icrc2_transfer_from".to_string(),
                format!("Error calling 'icrc_ledger_canister_c2c_client::icrc2_transfer_from': {err:?}"),
                Some(HashMap::from([
                    ("from".to_string(), from.to_string()),
                    ("ledger_canister_id".to_string(), ledger_canister_id.to_string()),
                    ("amount".to_string(), amount.to_string()),
                ]))
            )
        })
        .map(|block_index| block_index.0.try_into().unwrap())
}
