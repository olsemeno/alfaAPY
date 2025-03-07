pub mod kong;
pub mod token_swaps;

use crate::kong::kong::pools;
use candid::{candid_method, CandidType, Deserialize};
use candid::Principal;
use ic_cdk::{ trap};
use ic_cdk_macros::{init, query, update};
pub use kongswap_canister::pools::{PoolsReply, Response};
use std::cell::RefCell;

thread_local! {
    pub static CONF: RefCell<Conf> = RefCell::new(Conf::default());
}

#[derive(CandidType, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Conf {
    pub controllers: Option<Vec<Principal>>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            controllers: Default::default()
        }
    }
}

#[init]
#[candid_method(init)]
fn init(conf: Option<Conf>) {
    match conf {
        None => {}
        Some(conf) => {
            CONF.with(|c| c.replace(conf));
        }
    };
}

//TODO remove / test method
#[update]
async fn kong_pools() -> PoolsReply {
    match pools().await {
        Ok(reply) => reply,
        Err(err) => {
            trap(format!("Error: {}", err).as_str());
        }
    }
}

#[query]
fn get_config() -> Conf {
    CONF.with(|c| c.borrow().clone())
}
