use std::cell::RefCell;

use crate::event_records::event_record::EventRecord;
use crate::types::types::{ListItemsPaginationRequest, SortOrder};

thread_local! {
    pub static EVENT_RECORDS: RefCell<Vec<EventRecord>> = RefCell::new(Default::default());
}

pub fn save_event_record(event: EventRecord) {
    EVENT_RECORDS.with(|events| events.borrow_mut().push(event));
}

pub fn get_event_records_count() -> u64 {
    EVENT_RECORDS.with(|events| events.borrow().len() as u64)
}

pub fn get_event_records(request: ListItemsPaginationRequest) -> Vec<EventRecord> {
    let skip = ((request.page - 1) * request.page_size) as usize;
    let limit = request.page_size as usize;

    EVENT_RECORDS.with(|event_records| {
        let mut records: Vec<EventRecord> = event_records
            .borrow()
            .iter()
            .filter(|record| {
                if let Some(search) = &request.search {
                    record.0.event.type_str().contains(search)
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        // Sort by timestamp
        match request.sort_order {
            SortOrder::Asc => records.sort_by(|a, b| {
                a.0.timestamp.cmp(&b.0.timestamp)
            }),
            SortOrder::Desc => records.sort_by(|a, b| {
                b.0.timestamp.cmp(&a.0.timestamp)
            }),
        }

        records
            .into_iter()
            .skip(skip)
            .take(limit)
            .collect()
    })
}
