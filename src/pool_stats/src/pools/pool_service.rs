use types::exchange_id::ExchangeId;
use types::pool::PoolTrait;
use utils::constants::{
    ICP_TOKEN_CANISTER_ID,
    PANDA_TOKEN_CANISTER_ID,
    ICS_TOKEN_CANISTER_ID,
};

use crate::pools::pool::Pool;

pub fn init_pools() {
    // Panda/ICP KongSwap
    Pool::build(
        *PANDA_TOKEN_CANISTER_ID,
        *ICP_TOKEN_CANISTER_ID,
        ExchangeId::KongSwap,
    ).save();
    // Panda/ICP ICPSwap
    Pool::build(
        *PANDA_TOKEN_CANISTER_ID,
        *ICP_TOKEN_CANISTER_ID,
        ExchangeId::ICPSwap,
    ).save();
    // ICS/ICP KongSwap
    Pool::build(
        *ICS_TOKEN_CANISTER_ID,
        *ICP_TOKEN_CANISTER_ID,
        ExchangeId::KongSwap,
    ).save();
    // ICS/ICP ICPSwap
    Pool::build(
        *ICS_TOKEN_CANISTER_ID,
        *ICP_TOKEN_CANISTER_ID,
        ExchangeId::ICPSwap,
    ).save();
}
