use crate::events::event::IEvent;
use std::cell::RefCell;
use crate::events::event::EventType;

thread_local! {
    pub static USER_EVENTS: RefCell<Vec<Box<dyn IUserEvent >>> = RefCell::new(Default::default());
    pub static SYSTEM_EVENTS: RefCell<Vec<Box<dyn ISystemEvent >>> = RefCell::new(Default::default());
}

pub fn add_user_event(event: Box<dyn IUserEvent>) {
    USER_EVENTS.with(|events| {
        events.borrow_mut().push(event);
    });
}

pub fn add_system_event(event: Box<dyn ISystemEvent>) {
    SYSTEM_EVENTS.with(|events| {
        events.borrow_mut().push(event);
    });
}

pub fn get_user_events() -> Vec<Box<dyn IUserEvent>> {
    USER_EVENTS.with(|events| {
        events.borrow().clone()
    })
}

pub fn get_system_events() -> Vec<Box<dyn ISystemEvent>> {
    SYSTEM_EVENTS.with(|events| {
        events.borrow().clone()
    })
}

pub fn get_user_events_by_user(user: Principal) -> Vec<Box<dyn IUserEvent>> {
    USER_EVENTS.with(|events| {
        events.borrow().iter().filter(|event| event.get_user() == user).collect()
    })
}

pub fn get_system_events_by_user(user: Principal) -> Vec<Box<dyn ISystemEvent>> {
    SYSTEM_EVENTS.with(|events| {
        events.borrow().iter().filter(|event| event.get_user() == user).collect()
    })
}

pub fn get_user_event_by_id(id: u64) -> Option<Box<dyn IUserEvent>> {
    USER_EVENTS.with(|events| {
        events.borrow().iter().find(|event| event.get_id() == id).cloned()
    })
}

pub fn get_system_event_by_id(id: u64) -> Option<Box<dyn ISystemEvent>> {
    SYSTEM_EVENTS.with(|events| {
        events.borrow().iter().find(|event| event.get_id() == id).cloned()
    })
}


