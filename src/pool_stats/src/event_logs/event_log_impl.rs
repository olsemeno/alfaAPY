use candid::Principal;
use ic_cdk::api::time;

use errors::internal_error::error::InternalError;
use event_logs::generic_event_log::GenericEventLog;

use crate::event_logs::event_log::{EventLog, EventLogType, EventLogParams};
use crate::repository::event_logs_repo;

impl EventLog {
    pub fn new(
        id: u64,
        correlation_id: String,
        event_type: EventLogType,
        params: EventLogParams,
        timestamp: u64,
        user: Option<Principal>,
        error: Option<InternalError>,
    ) -> Self {
        Self(GenericEventLog {
            id,
            event_type,
            params,
            timestamp,
            correlation_id,
            user,
            error,
        })
    }

    pub fn build(
        id: u64,
        correlation_id: String,
        params: EventLogParams,
        user: Option<Principal>,
        error: Option<InternalError>,
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

    pub fn save(&self) {
        event_logs_repo::save_event_log(self.clone());
    }
}
