use candid::Nat;
use candid::Principal;

use types::exchanges::TokenInfo;
use kongswap_canister::PoolReply;
use crate::types::types::TokensInfo;

pub fn nat_to_f64(n: &Nat) -> f64 {
    let nat_str = n.0.to_str_radix(10); // Convert to string
    nat_str.parse::<f64>().unwrap_or(0.0) // Parse as f64
}

pub fn nat_to_u128(n: &Nat) -> u128 {
    let nat_str = n.0.to_str_radix(10); // Convert to string
    nat_str.parse::<u128>().unwrap_or(0) // Parse as u128
}
