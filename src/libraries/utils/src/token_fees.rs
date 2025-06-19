use candid::Nat;
use types::CanisterId;

use crate::constants::*;

const ICP_TOKEN_FEE: u128 = 10_000;
const PANDA_TOKEN_FEE: u128 = 10_000;
const ICS_TOKEN_FEE: u128 = 1_000_000;

// TODO: remove this temporary function after fixing the token fees for ICPSwap
pub fn get_token_fee(ledger: CanisterId) -> Nat {
    match ledger.to_text().as_str() {
        ICP_TOKEN_CANISTER_ID => Nat::from(ICP_TOKEN_FEE),
        PANDA_TOKEN_CANISTER_ID => Nat::from(PANDA_TOKEN_FEE),
        ICS_TOKEN_CANISTER_ID => Nat::from(ICS_TOKEN_FEE),
        _ => Nat::from(0u128),
    }
}
