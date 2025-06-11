use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use serde::Serialize;

use crate::{Conf, CONF};
use crate::strategies::strategy::IStrategy;
use crate::strategies::strategy_candid::{StrategyCandid, Candid as StrategyToCandid};
use crate::repository::strategies_repo::STRATEGIES;
use crate::repository::events_repo::EVENT_LOGS;
use crate::event_logs::event_log::EventLog;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct Memory {
    pub strategies: Vec<StrategyCandid>,
    pub event_logs: Vec<EventLog>,
    pub config: Conf,
}

pub fn stable_save() {
    let conf = CONF.with(|conf| {
        conf.borrow().clone()
    });

    let strategies: Vec<StrategyCandid> = STRATEGIES.with(|strategies| {
        strategies.borrow().iter().map(|strategy| strategy.to_candid()).collect()
    });

    let event_logs: Vec<EventLog> = EVENT_LOGS.with(|events| {
        events.borrow().clone()
    });

    let memory = Memory {
        strategies,
        event_logs,
        config: conf,
    };

    storage::stable_save((memory, )).unwrap();
}

pub fn stable_restore() {
    let (memory, ): (Memory, ) = storage::stable_restore().unwrap();

    // Conf
    CONF.with(|conf| {
        conf.borrow_mut();
        conf.replace(memory.config.clone())
    });

    // Strategies
    let strategies: Vec<Box<dyn IStrategy>> = memory.strategies
        .into_iter()
        .map(|x| x.to_strategy())
        .collect();

    STRATEGIES.with(|utrs| {
        utrs.borrow_mut();
        utrs.replace(strategies)
    });

    // EventLogs
    let event_logs: Vec<EventLog> = memory.event_logs;

    EVENT_LOGS.with(|events| {
        events.borrow_mut();
        events.replace(event_logs)
    });
}
