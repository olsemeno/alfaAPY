use types::CanisterId;
use types::exchange_id::ExchangeId;
use utils::constants::*;

use crate::pools::pool::Pool;

pub fn init_pools() {
    // Panda/ICP KongSwap
    Pool::create(
        CanisterId::from_text(PANDA_TOKEN_CANISTER_ID).unwrap(),
        CanisterId::from_text(ICP_TOKEN_CANISTER_ID).unwrap(),
        ExchangeId::KongSwap,
    );
    // Panda/ICP ICPSwap
    Pool::create(
        CanisterId::from_text(PANDA_TOKEN_CANISTER_ID).unwrap(),
        CanisterId::from_text(ICP_TOKEN_CANISTER_ID).unwrap(),
        ExchangeId::ICPSwap,
    );
    // ICS/ICP KongSwap
    Pool::create(
        CanisterId::from_text(ICS_TOKEN_CANISTER_ID).unwrap(),
        CanisterId::from_text(ICP_TOKEN_CANISTER_ID).unwrap(),
        ExchangeId::KongSwap,
    );
    // ICS/ICP ICPSwap
    Pool::create(
        CanisterId::from_text(ICS_TOKEN_CANISTER_ID).unwrap(),
        CanisterId::from_text(ICP_TOKEN_CANISTER_ID).unwrap(),
        ExchangeId::ICPSwap,
    );
}
