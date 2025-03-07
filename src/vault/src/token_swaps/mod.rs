use std::convert::TryInto;
use candid::Nat;

pub mod swap_service;
pub mod kongswap;
pub mod swap_client;

fn nat_to_u128(value: Nat) -> u128 {
    value.0.try_into().unwrap()
}
