use std::cell::{RefCell, RefMut};
use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use serde::Serialize;
use crate::{Conf, CONF};
use crate::strategies::strategy::{IStrategy, Strategy, StrategyIterator};
use crate::strategies::strategy_candid::{Candid, StrategyCandid};

thread_local! {
    pub static STRATEGIES: RefCell<Vec<Box<dyn IStrategy >>> = RefCell::new(Default::default());
}


pub fn get_all_strategies() -> Vec<Box<dyn IStrategy>> {
    STRATEGIES.with(|strategies| {
        let a = strategies.borrow_mut();
        let trs = StrategyIterator::new(a);
        trs.into_iter()
            .collect()
    })
}

pub fn get_strategy_by_id(id: u16) -> Option<Box<dyn IStrategy>> {
    STRATEGIES.with(|strategies| {
        let a = strategies.borrow_mut();
        let trs = StrategyIterator::new(a);
        trs.into_iter()
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

pub fn save_strategy(strategy: Box<dyn IStrategy>) {
    STRATEGIES.with(|strategies| {
        let mut strategies = strategies.borrow_mut();
        let index = strategies.iter().position(|s| s.get_id() == strategy.get_id());
        if let Some(index) = index {
            strategies[index] = strategy;
        }
    });
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct Memory {
    pub strategies: Vec<StrategyCandid>,
    pub config: Conf,
}

pub fn stable_save() {
    let strategies: Vec<StrategyCandid> = STRATEGIES.with(|trss| {
        let a = trss.borrow();
        a.iter()
            .map(|a| a.to_candid())
            .collect()
    });
    let conf = CONF.with(|conf| {
        let a = conf.borrow();
        a.clone()
    });
    let mem = Memory {
        strategies,
        config: conf,
    };
    storage::stable_save((mem, )).unwrap();
}

pub fn stable_restore() {
    let (mo, ): (Memory, ) = storage::stable_restore().unwrap();
    CONF.with(|conf| {
        conf.borrow_mut();
        conf.replace(mo.config.clone())
    });
    let strategies: Vec<Box<dyn IStrategy>> = mo.strategies
        .into_iter()
        .map(|x| x.to_strategy())
        .collect();

    STRATEGIES.with(|utrs| {
        utrs.borrow_mut();
        utrs.replace(strategies)
    });
}