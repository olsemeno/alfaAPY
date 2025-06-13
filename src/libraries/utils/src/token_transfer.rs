use candid::{Nat, Principal};
use ic_cdk::id;

use std::convert::TryInto;
use icrc_ledger_canister::updates::icrc2_transfer_from::Args as Icrc2TransferFromArgs;
use icrc_ledger_types::icrc1::account::Account;
use errors::internal_error::error::InternalError;
use errors::internal_error::builder::InternalErrorBuilder;
use types::CanisterId;

pub async fn icrc2_transfer_from_user(
    user: Principal,
    ledger: CanisterId,
    amount: Nat,
) -> Result<u64, InternalError> {
    let transfer_args: Icrc2TransferFromArgs = Icrc2TransferFromArgs {
        spender_subaccount: None,
        from: Account { owner: user, subaccount: None },
        to: Account { owner: id(), subaccount: None },
        amount: amount.clone(),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    match icrc_ledger_canister_c2c_client::icrc2_transfer_from(ledger, &transfer_args).await {
        Ok(Ok(block_index)) => Ok(block_index.0.try_into().unwrap()),
        Ok(Err(err)) => {
            Err(
                InternalErrorBuilder::business_logic()
                    .context("Utils: icrc2_transfer_from_user")
                    .message(format!("Error calling 'icrc2_transfer_from': {err:?}"))
                    .build()
            )
        }
        Err(error) => {
            Err(
                InternalErrorBuilder::external_service("icrc_ledger_canister_c2c_client::icrc2_transfer_from".to_string())
                    .context("Utils: icrc2_transfer_from_user")
                    .message(format!("IC error calling 'icrc2_transfer_from': {error:?}"))
                    .build()
            )
        }
    }
}
