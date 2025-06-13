use std::cell::RefCell;
use candid::{Nat, Principal};
use icrc_ledger_canister::updates::icrc2_transfer_from::Args as Icrc2TransferFromArgs;
use icrc_ledger_types::icrc1::account::Account;
use std::convert::TryInto;
use ic_cdk::api::time;
use ic_cdk::{caller, id, trap};
use types::CanisterId;
use types::context::Context;
use crate::types::types::StrategyId;

thread_local! {
    pub static USER_ACCOUNTS: RefCell<Vec<UserAccount>> = RefCell::new(Default::default());
}

struct UserAccount {
    user_id: Principal,
    deposits: Vec<UserDeposit>
}

#[allow(unused)]
struct UserDeposit {
    amount: Nat,
    strategy: StrategyId,
    ledger: CanisterId,
    block_index: u64,
    timestamp: u64
}

pub async fn accept_deposit(
    context: Context,
    amount: Nat,
    ledger: Principal,
    str_id: StrategyId
) -> Result<(), String> {
    // TODO: use utils::icrc2_transfer_from_user
    let transfer_args: Icrc2TransferFromArgs = Icrc2TransferFromArgs {
        spender_subaccount: None,
        from: Account { owner: caller(), subaccount: None },
        to: {
            Account { owner: id(), subaccount: None }
        },
        amount: amount.clone(),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    let block_index = match icrc2_transfer_from(ledger, &transfer_args).await {
        Ok(block_index) => {
            block_index
        }
        Err(message) => {
            trap(format!("Error transferring deposit: {message}").as_str());
        }
    };

    let deposit = UserDeposit {
        amount,
        strategy: str_id,
        ledger: ledger.into(),
        block_index: block_index,
        timestamp: time()
    };

    USER_ACCOUNTS.with(|accounts| {
        let mut accounts = accounts.borrow_mut();
        let index = accounts.iter().position(|a| a.user_id == caller());
        if let Some(index) = index {
            accounts[index].deposits.push(deposit);
        } else {
            accounts.push(UserAccount {
                user_id: caller(),
                deposits: vec![deposit]
            });
        }
    });

    Ok(())
}



async fn icrc2_transfer_from(ledger_canister_id: CanisterId, transfer_args: &icrc_ledger_canister::updates::icrc2_transfer_from::Args) -> Result<u64, String> {
    match icrc_ledger_canister_c2c_client::icrc2_transfer_from(ledger_canister_id, transfer_args).await {
        Ok(Ok(block_index)) => Ok(block_index.0.try_into().unwrap()),
        Ok(Err(err)) => Err(format!("Error calling 'icrc2_transfer_from': {err:?}")),
        Err(error) => Err(format!("IC error calling 'icrc2_transfer_from': {error:?}")),
    }
}