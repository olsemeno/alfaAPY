use std::cell::RefCell;

use crate::event_logs::event_log::EventLog;

thread_local! {
    pub static EVENT_LOGS: RefCell<Vec<EventLog>> = RefCell::new(Default::default());
}

pub fn save_event_log(event: EventLog) {
    EVENT_LOGS.with(|events| events.borrow_mut().push(event));
}

pub fn get_event_logs_count() -> u64 {
    EVENT_LOGS.with(|events| events.borrow().len() as u64)
}

pub fn get_event_logs(offset: usize, limit: usize) -> Vec<EventLog> {
    EVENT_LOGS.with(|events| {
        events
            .borrow()
            .iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect()
    })
}
