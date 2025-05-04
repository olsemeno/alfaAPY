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

pub struct EventService {
    user_event_counter: u64,
    system_event_counter: u64,
}

impl EventService {
    pub fn new() -> Self {
        EventService {
            user_event_counter: 0,
            system_event_counter: 0,
        }
    }
}

impl IEventService for EventService {
    fn add_user_event(&mut self, event_type: UserEventType, details: UserEventDetails, user: Principal) {
        let event = UserEvent {
            id: self.user_event_counter,
            event_type,
            details,
            timestamp: ic_cdk::api::time(),
            user,
        };
        self.user_event_counter += 1;

        add_user_event(event);
    }

    fn add_system_event(&mut self, event_type: SystemEventType, details: SystemEventDetails) {
        let event: SystemEvent = SystemEvent {
            id: self.system_event_counter,
            event_type,
            details,
            timestamp: ic_cdk::api::time(),
        };
        self.system_event_counter += 1;

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
