#![allow(deprecated)]
use candid::CandidType;
use serde::{Deserialize, Serialize};

const ICP_FEE: u128 = 10_000;

#[deprecated]
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Cryptocurrency {
    InternetComputer,
    SNS1,
    CKBTC,
    CHAT,
    KINIC,
    Other(String),
}

impl Cryptocurrency {
    pub fn token_symbol(&self) -> &str {
        match self {
            Cryptocurrency::InternetComputer => "ICP",
            Cryptocurrency::SNS1 => "SNS1",
            Cryptocurrency::CKBTC => "ckBTC",
            Cryptocurrency::CHAT => "CHAT",
            Cryptocurrency::KINIC => "KINIC",
            Cryptocurrency::Other(symbol) => symbol,
        }
    }
}

#[allow(deprecated)]
impl From<String> for Cryptocurrency {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ICP" => Cryptocurrency::InternetComputer,
            "ckBTC" => Cryptocurrency::CKBTC,
            "CHAT" => Cryptocurrency::CHAT,
            "KINIC" => Cryptocurrency::KINIC,
            _ => Cryptocurrency::Other(value),
        }
    }
}

pub type TransactionHash = [u8; 32];

pub mod icrc1 {
    use candid::Principal;
    use super::*;

    #[derive(CandidType, Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq)]
    pub struct Account {
        pub owner: Principal,
        pub subaccount: Option<[u8; 32]>,
    }

    impl<T: Into<Principal>> From<T> for Account {
        fn from(value: T) -> Self {
            Account {
                owner: value.into(),
                subaccount: None,
            }
        }
    }

    impl From<Account> for icrc_ledger_types::icrc1::account::Account {
        fn from(value: Account) -> Self {
            icrc_ledger_types::icrc1::account::Account {
                owner: value.owner,
                subaccount: value.subaccount,
            }
        }
    }

    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub enum CryptoAccount {
        Mint,
        Account(Account),
    }

    impl From<Account> for CryptoAccount {
        fn from(value: Account) -> Self {
            CryptoAccount::Account(value)
        }
    }
}

pub mod icrc2 {
    use super::*;

    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub enum ApproveError {
        BadFee { expected_fee: u128 },
        // The caller does not have enough funds to pay the approval fee.
        InsufficientFunds { balance: u128 },
        // The caller specified the [expected_allowance] field, and the current
        // allowance did not match the given value.
        AllowanceChanged { current_allowance: u128 },
        // The approval request expired before the ledger had a chance to apply it.
        Expired { ledger_time: u64 },
        TooOld,
        CreatedInFuture { ledger_time: u64 },
        Duplicate { duplicate_of: u128 },
        TemporarilyUnavailable,
        GenericError { error_code: u128, message: String },
    }

    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub enum TransferFromError {
        BadFee { expected_fee: u128 },
        BadBurn { min_burn_amount: u128 },
        // The [from] account does not hold enough funds for the transfer.
        InsufficientFunds { balance: u128 },
        // The caller exceeded its allowance.
        InsufficientAllowance { allowance: u128 },
        TooOld,
        CreatedInFuture { ledger_time: u64 },
        Duplicate { duplicate_of: u128 },
        TemporarilyUnavailable,
        GenericError { error_code: u128, message: String },
    }



}


