use candid::Principal;

use crate::event_logs::event_log::{EventLog, EventLogParams, EventLogError};
use crate::repository::events_repo;

pub fn create_event_log(
    event_params: EventLogParams,
    correlation_id: String,
    user: Option<Principal>,
    error: Option<EventLogError>,
) -> EventLog {
    EventLog::create(event_params, correlation_id, user, error)
}

pub fn get_event_logs(offset: usize, limit: usize) -> Vec<EventLog> {
    events_repo::get_event_logs(offset, limit)
}
