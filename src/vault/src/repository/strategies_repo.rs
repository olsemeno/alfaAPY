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
