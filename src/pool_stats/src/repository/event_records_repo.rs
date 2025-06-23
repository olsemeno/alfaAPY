use std::cell::RefCell;

use crate::event_records::event_record::EventRecord;

thread_local! {
    pub static EVENT_RECORDS: RefCell<Vec<EventRecord>> = RefCell::new(Default::default());
}

pub fn save_event_record(event: EventRecord) {
    EVENT_RECORDS.with(|events| events.borrow_mut().push(event));
}

pub fn get_event_records_count() -> u64 {
    EVENT_RECORDS.with(|events| events.borrow().len() as u64)
}

pub fn get_event_records(offset: usize, limit: usize) -> Vec<EventRecord> {
    EVENT_RECORDS.with(|events| {
        events
            .borrow()
            .iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect()
    })
}
