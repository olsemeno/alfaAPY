use candid::{Nat, Principal};

use crate::ICPSwapSwapPoolResult;

pub type Args = (Principal,);

pub type Response = (ICPSwapSwapPoolResult<Vec<Nat>>,);
