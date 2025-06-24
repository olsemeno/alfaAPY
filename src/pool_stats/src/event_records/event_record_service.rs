use candid::Principal;

use crate::event_records::event_record::{EventRecord, Event};
use crate::repository::event_records_repo;

pub fn create_event_record(
    event: Event,
    correlation_id: String,
    user: Option<Principal>,
) -> EventRecord {
    let event_record = EventRecord::build(
        next_id(),
        correlation_id,
        event,
        user,
    );
    event_record.save();
    event_record
}

pub fn get_event_records(offset: usize, limit: usize) -> Vec<EventRecord> {
    event_records_repo::get_event_records(offset, limit)
}

fn next_id() -> u64 {
    event_records_repo::get_event_records_count()
}
