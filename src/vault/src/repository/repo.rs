use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use serde::Serialize;

use crate::{Conf, CONF};
use crate::strategies::strategy::IStrategy;
use crate::strategies::strategy_candid::{StrategyCandid, Candid as StrategyToCandid};
use crate::events::event::IEvent;
use crate::events::event_candid::{EventCandid, Candid as EventToCandid};
use crate::repository::strategies_repo::STRATEGIES;
use crate::repository::events_repo::USER_EVENTS;
use crate::repository::events_repo::SYSTEM_EVENTS;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct Memory {
    pub strategies: Vec<StrategyCandid>,
    pub user_events: Vec<EventCandid>,
    pub system_events: Vec<EventCandid>,
    pub config: Conf,
}

pub fn stable_save() {
    let conf = CONF.with(|conf| {
        let a = conf.borrow();
        a.clone()
    });

    let strategies: Vec<StrategyCandid> = STRATEGIES.with(|trss| {
        let a = trss.borrow();
        a.iter()
            .map(|a| a.to_candid())
            .collect()
    });

    let user_events: Vec<EventCandid> = USER_EVENTS.with(|events| {
        events.borrow().iter().map(|event| event.to_candid()).collect()
    });
    let system_events: Vec<EventCandid> = SYSTEM_EVENTS.with(|events| {
        events.borrow().iter().map(|event| event.to_candid()).collect()
    });

    let mem = Memory {
        strategies,
        user_events,
        system_events,
        config: conf,
    };

    storage::stable_save((mem, )).unwrap();
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

    // UserEvents
    let user_events: Vec<Box<dyn IEvent>> = memory.user_events
        .into_iter()
        .map(|event| event.to_event())
        .collect();

    USER_EVENTS.with(|events| {
        events.borrow_mut();
        events.replace(user_events)
    });

    // SystemEvents
    let system_events: Vec<Box<dyn IEvent>> = memory.system_events
        .into_iter()
        .map(|event| event.to_event())
        .collect();

    SYSTEM_EVENTS.with(|events| {
        events.borrow_mut();
        events.replace(system_events)
    });
}
