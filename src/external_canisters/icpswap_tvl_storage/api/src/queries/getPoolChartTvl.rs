use candid::{CandidType, Int, Nat};
use serde::{Deserialize, Serialize};

pub type Args = (String, Nat, Nat);

pub type Response = (Vec<PoolChartTvl>,);

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct PoolChartTvl {
    pub id: Nat,
    pub tvlUSD: f64,
    pub timestamp: Int,
}
