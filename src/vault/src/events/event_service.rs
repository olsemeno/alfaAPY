use candid::Principal;

use crate::events::event::{UserEvent, SystemEvent};
use crate::enums::{UserEventType, UserEventDetails, SystemEventType, SystemEventDetails};
use crate::repository::events_repo::{add_event, get_user_events_count, get_system_events_count};

pub fn add_user_event(event_type: UserEventType, details: UserEventDetails, user: Principal) {
    let event = UserEvent::new(
        get_user_events_count(),
        event_type,
        details,
        ic_cdk::api::time(),
        user,
    );
    add_event(Box::new(event));
}

pub fn add_system_event(event_type: SystemEventType, details: SystemEventDetails) {
    let event = SystemEvent::new(
        get_system_events_count(),
        event_type,
        details,
        ic_cdk::api::time(),
    );
    add_event(Box::new(event));
}

pub fn get_user_events(user: Principal) -> Vec<UserEvent> {
    crate::repository::events_repo::get_user_events(user)
}

pub fn get_system_events() -> Vec<SystemEvent> {
    crate::repository::events_repo::get_system_events()
}
