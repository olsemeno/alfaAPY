use std::cell::RefCell;
use candid::{Nat, Principal};
use icrc_ledger_canister::updates::icrc2_transfer_from::Args as Icrc2TransferFromArgs;
use icrc_ledger_types::icrc1::account::Account;
use std::convert::TryInto;
use ic_cdk::api::time;
use ic_cdk::{call, caller, id, trap};
use ic_cdk::api::call::CallResult;
use ic_ledger_types::Timestamp;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use icrc_ledger_canister::icrc1_transfer::Args;
use icrc_ledger_canister_c2c_client::icrc1_transfer;
use types::CanisterId;
use crate::strategies::strategy::{Strategy, StrategyId};

thread_local! {
    pub static USER_ACCOUNTS: RefCell<Vec<UserAccount>> = RefCell::new(Default::default());
}

struct UserAccount {
    user_id: Principal,
    deposits: Vec<UserDeposit>
}

struct UserDeposit {
    amount: Nat,
    strategy: StrategyId,
    ledger: CanisterId,
    block_index: u64,
    timestamp: u64
}

pub async fn accept_deposit(amount: Nat, ledger: Principal, str_id: StrategyId) -> Result<(), String> {
    let transfer_args: Icrc2TransferFromArgs = Icrc2TransferFromArgs {
        spender_subaccount: None,
        from: Account { owner: caller(), subaccount: None },
        to:  {
            Account { owner: id(), subaccount: None }
        },
        amount: amount.clone(),
        fee: None,
        memo: None,
        created_at_time: None,
    };

    let  bi= match icrc2_transfer_from(ledger, &transfer_args).await {
        Ok(mut block_index) => {
            block_index
        }
        Err(message) => {
           trap(format!("Error transferring deposit: {message}").as_str());
        }
    };

    let deposit = UserDeposit {
        amount: amount,
        strategy: str_id,
        ledger: ledger.into(),
        block_index: bi,
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


pub async fn withdraw_from_strategy(strategy_id: StrategyId, amount: Nat , ledger: Principal) -> Result<Nat, String> {
    let a =  icrc1_transfer(ledger, &Args {
        from_subaccount: None,
        to: Account { owner: caller(), subaccount: None },
        fee: None,
        created_at_time: None,
        memo: None,
        amount,
    }).await;
    match a {
        Ok(Ok(block_index)) => {
            Ok(block_index)
        }
        Ok(Err(e)) => {
            trap(format!("Error withdrawing: {e}").as_str());
        }
        Err(e) => {
            trap(e.1.as_str());
        }
    }
}


async fn icrc2_transfer_from(ledger_canister_id: CanisterId, transfer_args: &icrc_ledger_canister::updates::icrc2_transfer_from::Args) -> Result<u64, String> {
    match icrc_ledger_canister_c2c_client::icrc2_transfer_from(ledger_canister_id, transfer_args).await {
        Ok(Ok(block_index)) => Ok(block_index.0.try_into().unwrap()),
        Ok(Err(err)) => Err(format!("Error calling 'icrc2_transfer_from': {err:?}")),
        Err(error) => Err(format!("IC error calling 'icrc2_transfer_from': {error:?}")),
    }
}