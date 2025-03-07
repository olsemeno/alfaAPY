mod kong;

use crate::kong::kong::pools;
use candid::{candid_method, CandidType, Deserialize, Nat};
use candid::Principal;
use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_cdk::{call, trap};
use ic_cdk_macros::{init, query, update};
use kong_swap_canister::pools::{PoolsReply, Response};
use serde::Serialize;
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
