use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

pub use crate::Token;

pub type Response = Result<Metadata, String>;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub fee: Nat,
    pub key: String,
    pub sqrtPriceX96: Nat,
    pub tick: i32,
    pub liquidity: Nat,
    pub token0: Token,
    pub token1: Token,
    pub maxLiquidityPerTick: Nat,
    pub nextPositionId: Nat,
}
