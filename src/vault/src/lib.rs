use std::cell::RefCell;
use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk_macros::{init, query};

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


#[query]
fn get_config() -> Conf {
    CONF.with(|c| c.borrow().clone())
}
