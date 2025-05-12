use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

pub type Args = (Nat,);

pub type Response = Result<Position, String>;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub tickUpper: i32,
    pub tokensOwed0: Nat,
    pub tokensOwed1: Nat,
    pub feeGrowthInside1LastX128: Nat,
    pub liquidity: Nat,
    pub feeGrowthInside0LastX128: Nat,
    pub tickLower: i32,
}
