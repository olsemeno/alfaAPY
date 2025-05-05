use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum UserEventParams {
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

impl UserEventParams {
    pub fn event_type(&self) -> UserEventType {
        match self {
            UserEventParams::AddLiquidity { .. } => UserEventType::AddLiquidity,
            UserEventParams::RemoveLiquidity { .. } => UserEventType::RemoveLiquidity,
        }
    }

    pub fn details(&self) -> UserEventDetails {
        match self {
            UserEventParams::AddLiquidity { amount, token, symbol } => UserEventDetails::AddLiquidity {
                amount: amount.clone(),
                token: token.clone(),
                symbol: symbol.clone(),
            },
            UserEventParams::RemoveLiquidity { amount, token, symbol } => UserEventDetails::RemoveLiquidity {
                amount: amount.clone(),
                token: token.clone(),
                symbol: symbol.clone(),
            },
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SystemEventParams {
    Rebalance {
        old_pool: String,
        new_pool: String,
    },
    Swap,
}

impl SystemEventParams {
    pub fn event_type(&self) -> SystemEventType {
        match self {
            SystemEventParams::Rebalance { .. } => SystemEventType::Rebalance,
            SystemEventParams::Swap => SystemEventType::Swap,
        }
    }

    pub fn details(&self) -> SystemEventDetails {
        match self {
            SystemEventParams::Rebalance { old_pool, new_pool } => SystemEventDetails::Rebalance {
                old_pool: old_pool.clone(),
                new_pool: new_pool.clone(),
            },
            SystemEventParams::Swap => SystemEventDetails::Swap,
        }
    }
}

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
