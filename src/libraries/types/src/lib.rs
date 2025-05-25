pub mod exchanges;
pub mod cryptocurrency;
pub mod user;
pub mod swap_tokens;
pub mod pin_number;
pub mod exchange_id;
pub mod liquidity;

use candid::{CandidType, Principal};
use ic_ledger_types::Tokens;
use serde::{Deserialize, Serialize};

pub type AccessorId = Principal;
pub type CanisterId = Principal;
pub type FileId = u128;
pub type Hash = [u8; 32];
pub type ICP = Tokens;
pub type Milliseconds = u64;
pub type Nanoseconds = u64;
pub type NnsNeuronId = u64;
pub type ProposalId = u64;
pub type SnsNeuronId = [u8; 32];
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Empty {}

pub trait PushIfNotContains<T> {
    fn push_if_not_contains(&mut self, item: T) -> bool;
}

impl<T: PartialEq> PushIfNotContains<T> for Vec<T> {
    fn push_if_not_contains(&mut self, item: T) -> bool {
        if !self.contains(&item) {
            self.push(item);
            true
        } else {
            false
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ResultLowercase<T, E> {
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "err")]
    Err(E),
}

pub fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == Default::default()
}

pub trait Fallback: Sized {
    type FallbackType: Into<Self>;
}
