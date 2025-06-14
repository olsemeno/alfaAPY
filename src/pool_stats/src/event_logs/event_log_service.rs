use candid::Principal;

use errors::internal_error::error::InternalError;

use crate::event_logs::event_log::{EventLog, EventLogParams};
use crate::repository::event_logs_repo;

pub fn create_event_log(
    params: EventLogParams,
    correlation_id: String,
    user: Option<Principal>,
    error: Option<InternalError>,
) -> EventLog {
    let event_log = EventLog::build(
        next_id(),
        correlation_id,
        params,
        user,
        error,
    );
    event_log.save();
    event_log
}

pub fn get_event_logs(offset: usize, limit: usize) -> Vec<EventLog> {
    event_logs_repo::get_event_logs(offset, limit)
}

fn next_id() -> u64 {
    event_logs_repo::get_event_logs_count()
}
