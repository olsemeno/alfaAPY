use std::cell::RefCell;
use candid::Principal;

use crate::strategies::strategy::{IStrategy, StrategyIterator};

thread_local! {
    pub static STRATEGIES: RefCell<Vec<Box<dyn IStrategy >>> = RefCell::new(Default::default());
}

pub fn get_all_strategies() -> Vec<Box<dyn IStrategy>> {
    STRATEGIES.with(|strategies| {
        StrategyIterator::new(strategies.borrow_mut())
            .into_iter()
            .collect()
    })
}

pub fn get_user_strategies(user: Principal) -> Vec<Box<dyn IStrategy>> {
    STRATEGIES.with(|strategies| {
        StrategyIterator::new(strategies.borrow_mut())
            .into_iter()
            .filter(|s| s.get_user_shares().contains_key(&user))
            .collect()
    })
}

pub fn get_strategy_by_id(id: u16) -> Option<Box<dyn IStrategy>> {
    STRATEGIES.with(|strategies| {
        StrategyIterator::new(strategies.borrow_mut())
            .into_iter()
            .find(|s| s.get_id() == id)
    })
}

pub fn add_or_update_strategy(strategy: Box<dyn IStrategy>) {
    STRATEGIES.with(|strategies| {
        let mut strategies = strategies.borrow_mut();
        let index = strategies.iter().position(|s| s.get_id() == strategy.get_id());
        if let Some(index) = index {
            strategies[index] = strategy;
        } else {
            strategies.push(strategy);
        }
    });
}

pub fn add_if_not_exists(strategy: Box<dyn IStrategy>) {
    STRATEGIES.with(|strategies| {
        let mut strategies = strategies.borrow_mut();
        let index = strategies.iter().position(|s| s.get_id() == strategy.get_id());
        if index.is_none() {
            strategies.push(strategy);
        }
    });
}

pub fn save_strategy(strategy: Box<dyn IStrategy>) {
    STRATEGIES.with(|strategies| {
        let mut strategies = strategies.borrow_mut();
        let index = strategies.iter().position(|s| s.get_id() == strategy.get_id());
        if let Some(index) = index {
            strategies[index] = strategy;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use candid::{Nat, Principal};

    use crate::strategies::basic_strategy::BasicStrategy;
    use crate::pools::pool::Pool;
    use crate::strategies::strategy_candid::StrategyCandid;

    #[derive(Clone)]
    struct MockStrategy {
        id: u16,
        user_shares: HashMap<Principal, Nat>,
    }

    impl MockStrategy {
        fn new(id: u16, user: Option<Principal>) -> Self {
            let mut user_shares = HashMap::new();
            if let Some(principal) = user.clone() {
                user_shares.insert(principal, Nat::from(100u64));
            }
            Self { id, user_shares }
        }
    }

    impl BasicStrategy for MockStrategy {
        fn get_id(&self) -> u16 {
            self.id
        }

        fn get_user_shares(&self) -> HashMap<Principal, Nat> {
            self.user_shares.clone()
        }

        fn get_users_count(&self) -> u32 {
            self.user_shares.len() as u32
        }

        fn get_position_id(&self) -> Option<u64> {
            None
        }

        fn get_user_shares_by_principal(&self, user: Principal) -> Nat {
            self.user_shares.get(&user).cloned().unwrap_or(Nat::from(0u64))
        }

        fn get_name(&self) -> String {
            "mock-strategy".to_string()
        }

        fn get_description(&self) -> String {
            "mock-desc".to_string()
        }

        fn get_pools(&self) -> Vec<Pool> {
            vec![]
        }

        fn get_current_pool(&self) -> Option<Pool> {
            None
        }

        fn get_total_shares(&self) -> Nat {
            Nat::from(100u64)
        }

        fn get_total_balance(&self) -> Nat {
            Nat::from(100u64)
        }

        fn get_initial_deposit(&self) -> HashMap<Principal, Nat> {
            HashMap::new()
        }

        fn get_current_liquidity(&self) -> Option<Nat> {
            None
        }

        fn get_current_liquidity_updated_at(&self) -> Option<u64> {
            None
        }

        fn set_user_shares(&mut self, _shares: HashMap<Principal, Nat>) {}
        fn set_total_shares(&mut self, _shares: Nat) {}
        fn set_total_balance(&mut self, _balance: Nat) {}
        fn set_initial_deposit(&mut self, _map: HashMap<Principal, Nat>) {}
        fn set_current_pool(&mut self, _pool: Option<Pool>) {}
        fn set_position_id(&mut self, _id: Option<u64>) {}
        fn set_current_liquidity(&mut self, _liq: Option<Nat>) {}
        fn set_current_liquidity_updated_at(&mut self, _ts: Option<u64>) {}
    }

    #[async_trait::async_trait]
    impl IStrategy for MockStrategy {
        fn to_candid(&self) -> StrategyCandid {
            unimplemented!()
        }
        fn clone_self(&self) -> Box<dyn IStrategy> {
            Box::new(Self {
                id: self.id,
                user_shares: self.user_shares.clone(),
            })
        }
    }

    mod get_all_strategies {
        use super::*;

        #[test]
        fn returns_all_strategies() {
            STRATEGIES.with(|s| s.borrow_mut().clear());

            add_if_not_exists(Box::new(MockStrategy::new(1, None)));
            add_if_not_exists(Box::new(MockStrategy::new(2, None)));

            let strategies = get_all_strategies();
            assert_eq!(strategies.len(), 2);
        }
    }

    mod get_user_strategies {
        use super::*;

        #[test]
        fn filters_by_user() {
            STRATEGIES.with(|s| s.borrow_mut().clear());
            let user = Principal::anonymous();

            add_if_not_exists(Box::new(MockStrategy::new(1, Some(user))));
            add_if_not_exists(Box::new(MockStrategy::new(2, None)));

            let strategies = get_user_strategies(user);
            assert_eq!(strategies.len(), 1);
            assert_eq!(strategies[0].get_id(), 1);
        }
    }

    mod get_strategy_by_id {
        use super::*;

        #[test]
        fn finds_correct_strategy() {
            STRATEGIES.with(|s| s.borrow_mut().clear());

            add_if_not_exists(Box::new(MockStrategy::new(42, None)));

            let strategy = get_strategy_by_id(42);
            assert!(strategy.is_some());
            assert_eq!(strategy.unwrap().get_id(), 42);
        }
    }

    mod add_or_update_strategy {
        use super::*;

        #[test]
        fn replaces_existing_strategy() {
            STRATEGIES.with(|s| s.borrow_mut().clear());

            let s1 = Box::new(MockStrategy::new(7, None));
            let s2 = Box::new(MockStrategy::new(7, None));

            add_or_update_strategy(s1);
            add_or_update_strategy(s2);

            let strategies = get_all_strategies();
            assert_eq!(strategies.len(), 1);
        }
    }

    mod add_if_not_exists {
        use super::*;

        #[test]
        fn does_not_add_duplicate() {
            STRATEGIES.with(|s| s.borrow_mut().clear());

            let s = Box::new(MockStrategy::new(8, None));
            add_if_not_exists(s.clone());
            add_if_not_exists(s);

            let strategies = get_all_strategies();
            assert_eq!(strategies.len(), 1);
        }
    }

    mod save_strategy {
        use super::*;

        #[test]
        fn updates_existing_strategy_only() {
            STRATEGIES.with(|s| s.borrow_mut().clear());

            let strategy = Box::new(MockStrategy::new(9, None));
            save_strategy(strategy.clone());
            assert_eq!(get_all_strategies().len(), 0);

            add_or_update_strategy(strategy.clone());
            save_strategy(strategy);
            assert_eq!(get_all_strategies().len(), 1);
        }
    }
}
