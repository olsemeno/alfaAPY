use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::pools::pool::Pool;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug)]
pub struct PoolData {
    pub pool: Pool,
    pub apy: f64,
}
