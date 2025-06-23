use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use serde::Serialize;

use crate::{Conf, CONF};
use crate::strategies::strategy::IStrategy;
use crate::strategies::strategy_candid::{StrategyCandid, Candid as StrategyToCandid};
use crate::repository::strategies_repo::STRATEGIES;
use crate::repository::event_records_repo::EVENT_RECORDS;
use crate::event_records::event_record::EventRecord;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct StableState {
    pub strategies: Vec<StrategyCandid>,
    pub event_records: Vec<EventRecord>,
    pub config: Conf,
}

pub fn stable_save() {
    let conf = CONF.with(|conf| {
        conf.borrow().clone()
    });

    let strategies: Vec<StrategyCandid> = STRATEGIES.with(|strategies| {
        strategies.borrow().iter().map(|strategy| strategy.to_candid()).collect()
    });

    let event_records: Vec<EventRecord> = EVENT_RECORDS.with(|events| {
        events.borrow().clone()
    });

    let state = StableState {
        strategies,
        event_records,
        config: conf,
    };

    storage::stable_save((state, )).unwrap();
}

pub fn stable_restore() {
    let (state, ): (StableState, ) = storage::stable_restore().unwrap();

    // Conf
    CONF.with(|conf| {
        conf.borrow_mut();
        conf.replace(state.config.clone())
    });

    // Strategies
    let strategies: Vec<Box<dyn IStrategy>> = state.strategies.clone()
        .into_iter()
        .map(|x| x.to_strategy())
        .collect();

    STRATEGIES.with(|utrs| {
        utrs.borrow_mut();
        utrs.replace(strategies)
    });

    // EventRecords
    EVENT_RECORDS.with(|event_records| {
        event_records.borrow_mut();
        event_records.replace(state.event_records)
    });
}
