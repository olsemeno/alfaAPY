use std::cell::RefCell;
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Deserialize, Clone, Debug, Hash, PartialEq, Serialize)]
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

thread_local! {
    static CONF: RefCell<Conf> = RefCell::new(Conf::default());
}

pub fn get_config() -> Conf {
    CONF.with(|c| c.borrow().clone())
}

pub fn set_config(config: Conf) {
    CONF.with(|c| {
        c.borrow_mut();
        c.replace(config);
    });
}

pub fn get_controllers() -> Option<Vec<Principal>> {
    CONF.with(|c| c.borrow().controllers.clone())
}

pub fn set_controllers(controllers: Option<Vec<Principal>>) {
    CONF.with(|c| {
        c.borrow_mut().controllers = controllers;
    });
}
