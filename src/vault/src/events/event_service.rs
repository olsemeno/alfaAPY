use candid::Principal;

use crate::events::event::{UserEvent, SystemEvent, ErrorEvent};
use crate::enums::{UserEventParams, SystemEventParams, ErrorEventParams};
use crate::repository::events_repo;

// Create events

pub fn create_user_event(event_params: UserEventParams, user: Principal) -> UserEvent {
    UserEvent::create(event_params, user)
}

pub fn create_system_event(event_params: SystemEventParams) -> SystemEvent {
    SystemEvent::create(event_params)
}

pub fn create_error_event(event_params: ErrorEventParams, user: Option<Principal>) -> ErrorEvent {
    ErrorEvent::create(event_params, user)
}

// Get events

pub fn get_user_events(user: Principal, offset: usize, limit: usize) -> Vec<UserEvent> {
    events_repo::get_user_events(user, offset, limit)
}

pub fn get_system_events(offset: usize, limit: usize) -> Vec<SystemEvent> {
    events_repo::get_system_events(offset, limit)
}

pub fn get_error_events(offset: usize, limit: usize) -> Vec<ErrorEvent> {
    events_repo::get_error_events(offset, limit)
}
