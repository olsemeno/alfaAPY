use candid::Principal;
use crate::events::event::{UserEvent, SystemEvent};
use crate::enums::{UserEventType, UserEventDetails, SystemEventType, SystemEventDetails};
use crate::events_repo::repo::{
    add_system_event,
    add_user_event,
    get_system_events,
    get_user_events,
    get_user_events_by_user,
    get_system_events_by_user,
    get_user_event_by_id,
    get_system_event_by_id,
};
use std::cell::RefCell;
use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize)]
pub struct EventCounters {
    user_event_counter: u64,
    system_event_counter: u64,
}

thread_local! {
    static EVENT_COUNTERS: RefCell<EventCounters> = RefCell::new(EventCounters {
        user_event_counter: 0,
        system_event_counter: 0,
    });

    static EVENT_SERVICE: RefCell<EventService> = RefCell::new(EventService::default());
}

pub trait IEventService {
    fn add_user_event(&mut self, event_type: UserEventType, details: UserEventDetails, user: Principal);
    fn add_system_event(&mut self, event_type: SystemEventType, details: SystemEventDetails);
    fn get_user_events(&self) -> Vec<UserEvent>;
    fn get_system_events(&self) -> Vec<SystemEvent>;
    fn get_user_events_by_user(&self, user: Principal) -> Vec<UserEvent>;
    fn get_system_events_by_user(&self, user: Principal) -> Vec<SystemEvent>;
    fn get_user_event_by_id(&self, id: u64) -> Option<UserEvent>;
    fn get_system_event_by_id(&self, id: u64) -> Option<SystemEvent>;
}

pub struct EventService {}

impl Default for EventService {
    fn default() -> Self {
        EventService {}
    }
}

impl EventService {
    pub fn instance() -> &'static RefCell<EventService> {
        EVENT_SERVICE.with(|service| unsafe {
            std::mem::transmute::<&RefCell<EventService>, &'static RefCell<EventService>>(service)
        })
    }

    pub fn get_counters() -> EventCounters {
        EVENT_COUNTERS.with(|counters| {
            let borrowed = counters.borrow();
            EventCounters {
                user_event_counter: borrowed.user_event_counter,
                system_event_counter: borrowed.system_event_counter,
            }
        })
    }

    pub fn set_counters(counters: EventCounters) {
        EVENT_COUNTERS.with(|c| *c.borrow_mut() = counters);
    }
}

impl IEventService for EventService {
    fn add_user_event(&mut self, event_type: UserEventType, details: UserEventDetails, user: Principal) {
        let mut counters = Self::get_counters();
        let event = UserEvent {
            id: counters.user_event_counter,
            event_type,
            details,
            timestamp: ic_cdk::api::time(),
            user,
        };
        counters.user_event_counter += 1;
        Self::set_counters(counters);
        add_user_event(event);
    }

    fn add_system_event(&mut self, event_type: SystemEventType, details: SystemEventDetails) {
        let mut counters = Self::get_counters();
        let event = SystemEvent {
            id: counters.system_event_counter,
            event_type,
            details,
            timestamp: ic_cdk::api::time(),
        };
        counters.system_event_counter += 1;
        Self::set_counters(counters);
        add_system_event(event);
    }

    fn get_user_events(&self) -> Vec<UserEvent> {
        get_user_events()
    }

    fn get_system_events(&self) -> Vec<SystemEvent> {
        get_system_events()
    }

    fn get_user_events_by_user(&self, user: Principal) -> Vec<UserEvent> {
        get_user_events_by_user(user)
    }

    fn get_system_events_by_user(&self, user: Principal) -> Vec<SystemEvent> {
        get_system_events_by_user(user)
    }

    fn get_user_event_by_id(&self, id: u64) -> Option<UserEvent> {
        get_user_event_by_id(id)
    }

    fn get_system_event_by_id(&self, id: u64) -> Option<SystemEvent> {
        get_system_event_by_id(id)
    }
}
