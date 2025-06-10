use candid::Principal;

use crate::events::event::{UserEvent, SystemEvent};
use crate::enums::{UserEventParams, SystemEventParams};
use crate::repository::events_repo;

pub fn create_user_event(event_params: UserEventParams, user: Principal) -> UserEvent {
    UserEvent::create(event_params, user)
}

pub fn create_system_event(event_params: SystemEventParams) -> SystemEvent {
    SystemEvent::create(event_params)
}

pub fn get_user_events(user: Principal, offset: usize, limit: usize) -> Vec<UserEvent> {
    events_repo::get_user_events(user, offset, limit)
}

pub fn get_system_events(offset: usize, limit: usize) -> Vec<SystemEvent> {
    events_repo::get_system_events(offset, limit)
}
