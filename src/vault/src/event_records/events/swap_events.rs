use candid::{CandidType, Deserialize, Nat};
use serde::Serialize;

use types::CanisterId;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SwapTokenStarted {
    pub pool_id: String,
    pub token_in: CanisterId,
    pub token_out: CanisterId,
    pub amount_in: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SwapTokenCompleted {
    pub token_in: CanisterId,
    pub token_out: CanisterId,
    pub amount_in: Option<Nat>,
    pub amount_out: Option<Nat>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SwapTokenFailed {
    pub pool_id: String,
    pub token_in: CanisterId,
    pub token_out: CanisterId,
    pub amount_in: Option<Nat>,
}
