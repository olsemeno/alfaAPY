use candid::{Int, Principal, Nat};
use types::CanisterId;
use num_bigint::{BigUint, Sign};

pub fn nat_to_f64(n: &Nat) -> f64 {
    let nat_str = n.0.to_str_radix(10); // Convert to string
    nat_str.parse::<f64>().unwrap_or(0.0) // Parse as f64
}

pub fn nat_to_u128(n: &Nat) -> u128 {
    let nat_str = n.0.to_str_radix(10); // Convert to string
    nat_str.parse::<u128>().unwrap_or(0) // Parse as u128
}

pub fn nat_to_u64(n: &Nat) -> u64 {
    let nat_str = n.0.to_str_radix(10); // Convert to string
    nat_str.parse::<u64>().unwrap_or(0) // Parse as u64
}

pub fn int_to_nat(int_value: Int) -> Option<Nat> {
    if int_value.0.sign() == num_bigint::Sign::Minus {
        // Negative value cannot be converted to Nat
        None
    } else {
        Some(Nat(BigUint::from_bytes_be(&int_value.0.to_bytes_be().1)))
    }
}

pub fn principal_to_canister_id(principal_str: &str) -> CanisterId {
    let principal = Principal::from_text(principal_str).unwrap();
    CanisterId::from_slice(principal.as_slice())
}

pub fn current_timestamp() -> u64 {
    ic_cdk::api::time() / 1_000_000_000 // convert to seconds
}
