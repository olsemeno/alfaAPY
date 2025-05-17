use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

pub use crate::ICPSwapSwapFactoryResult;
pub use crate::ICPSwapToken;
pub use crate::ICPSwapPool;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub fee: Nat,
    pub token0: ICPSwapToken,
    pub token1: ICPSwapToken,
}

pub type Response = ICPSwapSwapFactoryResult<ICPSwapPool>;
