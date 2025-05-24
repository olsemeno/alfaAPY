use candid::{CandidType, Nat, Int};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

pub type Args = (Nat,);

pub type Response = (ICPSwapSwapPoolResult<UserPosition>,);

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct UserPosition {
    pub tickUpper: Int,
    pub tokensOwed0: Nat,
    pub tokensOwed1: Nat,
    pub feeGrowthInside1LastX128: Nat,
    pub liquidity: Nat,
    pub feeGrowthInside0LastX128: Nat,
    pub tickLower: Int,
}
