use candid::Principal;
use ic_cdk::api::time;

use crate::events::event::{UserEvent, SystemEvent, IEvent};
use crate::enums::{UserEventParams, SystemEventParams};
use crate::repository::events_repo::{
    get_user_events_count,
    get_system_events_count,
    get_user_events as repo_get_user_events,
    get_system_events as repo_get_system_events,
};

pub fn create_user_event(event_params: UserEventParams, user: Principal) -> UserEvent {
    let event = UserEvent::from_params(
        get_user_events_count(),
        event_params,
        time(),
        user,
    );

    event.save();
    event
}

pub fn create_system_event(event_params: SystemEventParams) -> SystemEvent {
    let event = SystemEvent::from_params(
        get_system_events_count(),
        event_params,
        time(),
    );

    event.save();
    event
}

pub fn get_user_events(user: Principal) -> Vec<UserEvent> {
    repo_get_user_events(user)
}

pub fn get_system_events() -> Vec<SystemEvent> {
    repo_get_system_events()
}
