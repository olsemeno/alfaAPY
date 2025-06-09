use types::CanisterId;
use types::exchange_id::ExchangeId;
use utils::constants::*;

use types::pool::PoolTrait;

use crate::pools::pool::Pool;

pub fn init_pools() {
    // Panda/ICP KongSwap
    Pool::build(
        CanisterId::from_text(PANDA_TOKEN_CANISTER_ID).unwrap(),
        CanisterId::from_text(ICP_TOKEN_CANISTER_ID).unwrap(),
        ExchangeId::KongSwap,
    ).save();
    // Panda/ICP ICPSwap
    Pool::build(
        CanisterId::from_text(PANDA_TOKEN_CANISTER_ID).unwrap(),
        CanisterId::from_text(ICP_TOKEN_CANISTER_ID).unwrap(),
        ExchangeId::ICPSwap,
    ).save();
    // ICS/ICP KongSwap
    Pool::build(
        CanisterId::from_text(ICS_TOKEN_CANISTER_ID).unwrap(),
        CanisterId::from_text(ICP_TOKEN_CANISTER_ID).unwrap(),
        ExchangeId::KongSwap,
    ).save();
    // ICS/ICP ICPSwap
    Pool::build(
        CanisterId::from_text(ICS_TOKEN_CANISTER_ID).unwrap(),
        CanisterId::from_text(ICP_TOKEN_CANISTER_ID).unwrap(),
        ExchangeId::ICPSwap,
    ).save();
}
