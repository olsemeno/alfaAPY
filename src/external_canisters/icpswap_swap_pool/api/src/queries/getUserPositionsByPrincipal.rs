use candid::{CandidType, Nat, Principal, Int};
use serde::{Deserialize, Serialize};

use crate::ICPSwapSwapPoolResult;

pub type Args = (Principal,);

pub type Response = (ICPSwapSwapPoolResult<Vec<UserPositionWithId>>,);

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct UserPositionWithId {
    pub id: Nat,
    pub tickUpper: Int,
    pub tokensOwed0: Nat,
    pub tokensOwed1: Nat,
    pub feeGrowthInside1LastX128: Nat,
    pub liquidity: Nat,
    pub feeGrowthInside0LastX128: Nat,
    pub tickLower: Int,
}
