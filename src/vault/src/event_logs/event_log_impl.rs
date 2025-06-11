use candid::Principal;
use ic_cdk::api::time;
use std::collections::HashMap;

use crate::event_logs::event_log::{EventLog, EventLogType, EventLogParams, EventLogError, EventLogErrorKind};
use crate::repository::events_repo;

impl EventLog {
    pub fn new(
        id: u64,
        correlation_id: String,
        event_type: EventLogType,
        params: EventLogParams,
        timestamp: u64,
        user: Option<Principal>,
        error: Option<EventLogError>,
    ) -> Self {
        Self {
            id,
            event_type,
            params,
            timestamp,
            correlation_id,
            user,
            error,
        }
    }

    pub fn build(
        id: u64,
        correlation_id: String,
        params: EventLogParams,
        user: Option<Principal>,
        error: Option<EventLogError>,
    ) -> Self {
        Self::new(
            id,
            correlation_id,
            params.event_type(),
            params,
            time(),
            user,
            error,
        )
    }

    pub fn create(
        params: EventLogParams,
        correlation_id: String,
        user: Option<Principal>,
        error: Option<EventLogError>,
    ) -> Self {
        let event = Self::build(
            Self::next_id(),
            correlation_id,
            params,
            user,
            error,
        );
        event.save();
        event
    }

    pub fn save(&self) {
        events_repo::save_event_log(self.clone());
    }

    fn next_id() -> u64 {
        events_repo::get_event_logs_count()
    }
}

impl EventLogError {
    pub fn new(
        kind: EventLogErrorKind,
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>,
    ) -> Self {
        Self { kind, context, message, extra }
    }
}
