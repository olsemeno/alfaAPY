use std::cell::RefCell;
use candid::Principal;

use crate::events::event::{IEvent, IUserEvent, UserEvent, SystemEvent};
use crate::events::event_candid::EventCandid;

thread_local! {
    pub static USER_EVENTS: RefCell<Vec<Box<dyn IEvent>>> = RefCell::new(Default::default());
    pub static SYSTEM_EVENTS: RefCell<Vec<Box<dyn IEvent>>> = RefCell::new(Default::default());
}

pub fn save_event(event: Box<dyn IEvent>) {
    match event.to_candid() {
        EventCandid::User(user_event) => {
            USER_EVENTS.with(|events| events.borrow_mut().push(Box::new(user_event)));
        }
        EventCandid::System(system_event) => {
            SYSTEM_EVENTS.with(|events| events.borrow_mut().push(Box::new(system_event)));
        }
    }
}

pub fn get_user_events_count() -> u64 {
    USER_EVENTS.with(|events| events.borrow().len() as u64)
}

pub fn get_system_events_count() -> u64 {
    SYSTEM_EVENTS.with(|events| events.borrow().len() as u64)
}


pub fn get_user_events(user: Principal, offset: usize, limit: usize) -> Vec<UserEvent> {
    USER_EVENTS.with(|events| {
        events.borrow()
            .iter()
            .skip(offset)
            .take(limit)
            .map(|event| match event.to_candid() {
                EventCandid::User(user_event) => user_event,
                _ => unreachable!(),
            })
            .filter(|user_event| user_event.get_user() == user)
            .collect()
    })
}

pub fn get_system_events(offset: usize, limit: usize) -> Vec<SystemEvent> {
    SYSTEM_EVENTS.with(|events| {
        events.borrow()
            .iter()
            .skip(offset)
            .take(limit)
            .map(|event| match event.to_candid() {
                EventCandid::System(system_event) => system_event,
                _ => unreachable!(),
            })
            .collect()
    })
}
