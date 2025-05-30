use candid::{CandidType, Nat, Int};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

pub use crate::Token;

pub type Response = ICPSwapSwapPoolResult<Metadata>;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub fee: Nat,
    pub key: String,
    pub sqrtPriceX96: Nat,
    pub tick: Int,
    pub liquidity: Nat,
    pub token0: Token,
    pub token1: Token,
    pub maxLiquidityPerTick: Nat,
    pub nextPositionId: Nat,
}
