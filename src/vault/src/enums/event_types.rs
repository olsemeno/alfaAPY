use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum UserEventType {
    AddLiquidity,
    RemoveLiquidity,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SystemEventType {
    Rebalance,
    Swap,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum UserEventDetails {
    AddLiquidity {
        amount: Nat,
        token: String,
        symbol: String,
    },
    RemoveLiquidity {
        amount: Nat,
        token: String,
        symbol: String,
    },
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SystemEventDetails {
    Rebalance {
        old_pool: String,
        new_pool: String,
    },
    Swap,
}
