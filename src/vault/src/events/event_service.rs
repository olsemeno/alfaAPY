use candid::Principal;
use ic_cdk::api::time;

use crate::events::event::{UserEvent, SystemEvent, IEvent};
use crate::enums::{UserEventParams, SystemEventParams};
use crate::repository::events_repo;

pub fn create_user_event(event_params: UserEventParams, user: Principal) -> UserEvent {
    let event = UserEvent::from_params(
        events_repo::get_user_events_count(),
        event_params,
        time(),
        user,
    );

    event.save();
    event
}

pub fn create_system_event(event_params: SystemEventParams) -> SystemEvent {
    let event = SystemEvent::from_params(
        events_repo::get_system_events_count(),
        event_params,
        time(),
    );

    event.save();
    event
}

pub fn get_user_events(user: Principal, offset: usize, limit: usize) -> Vec<UserEvent> {
    events_repo::get_user_events(user, offset, limit)
}

pub fn get_system_events(offset: usize, limit: usize) -> Vec<SystemEvent> {
    events_repo::get_system_events(offset, limit)
}
