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

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    fn fake_principal(id: u8) -> Principal {
        Principal::from_slice(&[id; 29])
    }

    mod get_config {
        use super::*;

        #[test]
        fn returns_default_config() {
            let conf = get_config();
            assert_eq!(conf.controllers, None);
        }

        #[test]
        fn returns_updated_config_after_set_config() {
            let original = get_config();

            let new_conf = Conf {
                controllers: Some(vec![fake_principal(1), fake_principal(2)]),
            };
            set_config(new_conf.clone());

            let updated = get_config();
            assert_eq!(updated, new_conf);

            set_config(original); // restore
        }
    }

    mod set_config {
        use super::*;

        #[test]
        fn sets_new_config_and_overwrites_old_one() {
            let original = get_config();

            let new_conf = Conf {
                controllers: Some(vec![fake_principal(3)]),
            };
            set_config(new_conf.clone());

            let after_set = get_config();
            assert_eq!(after_set, new_conf);

            set_config(original); // restore
        }
    }

    mod get_controllers {
        use super::*;

        #[test]
        fn returns_none_by_default() {
            assert_eq!(get_controllers(), None);
        }

        #[test]
        fn returns_controllers_after_setting_config() {
            let original = get_config();

            let new_conf = Conf {
                controllers: Some(vec![fake_principal(42)]),
            };
            set_config(new_conf.clone());

            let controllers = get_controllers();
            assert_eq!(controllers, Some(vec![fake_principal(42)]));

            set_config(original); // restore
        }
    }

    mod set_controllers {
        use super::*;

        #[test]
        fn sets_controllers_directly() {
            let original = get_config();

            set_controllers(Some(vec![fake_principal(9), fake_principal(10)]));
            let current = get_config();

            assert_eq!(
                current.controllers,
                Some(vec![fake_principal(9), fake_principal(10)])
            );

            set_config(original); // restore
        }

        #[test]
        fn sets_controllers_to_none() {
            let original = get_config();

            set_controllers(None);
            let current = get_controllers();
            assert_eq!(current, None);

            set_config(original); // restore
        }
    }
}
