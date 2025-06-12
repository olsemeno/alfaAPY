use candid::Principal;

use errors::internal_error::error::InternalError;

use crate::event_logs::event_log::{EventLog, EventLogParams};
use crate::repository::event_logs_repo;

pub fn create_event_log(
    event_params: EventLogParams,
    correlation_id: String,
    user: Option<Principal>,
    error: Option<InternalError>,
) -> EventLog {
    EventLog::create(event_params, correlation_id, user, error)
}

pub fn get_event_logs(offset: usize, limit: usize) -> Vec<EventLog> {
    event_logs_repo::get_event_logs(offset, limit)
}
