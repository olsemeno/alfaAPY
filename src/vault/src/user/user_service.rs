use std::cell::RefCell;
use candid::{Nat, Principal};
use std::collections::HashMap;
use ic_cdk::api::time;
use ic_cdk::{caller, id, trap};

use types::CanisterId;
use types::context::Context;
use errors::internal_error::error::InternalError;
use ::utils::token_transfer::icrc2_transfer_from_user;
use ::utils::util::nat_to_u64;

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
    strategy_id: StrategyId
) -> Result<(), InternalError> {
    let block_index = icrc2_transfer_from_user(
        context.user.unwrap(),
        ledger,
        amount.clone()
    ).await
        .map_err(|error| {
            error.wrap(
                "user_service::accept_deposit".to_string(),
                format!("Error calling 'Utils::icrc2_transfer_from_user'"),
                Some(HashMap::from([
                    ("ledger".to_string(), ledger.to_string()),
                    ("amount".to_string(), amount.to_string()),
                ])),
            )
        })?;

    let deposit = UserDeposit {
        amount,
        strategy: strategy_id,
        ledger: ledger.into(),
        block_index: nat_to_u64(&block_index),
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
