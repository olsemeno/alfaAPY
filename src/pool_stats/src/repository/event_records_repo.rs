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

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Nat;

    use event_records::generic_event_record::GenericEventRecord;

    use crate::event_records::event_record::Event;

    fn create_test_event_record(
        id: u64,
        correlation_id: String,
        event: Event,
        caller: Option<candid::Principal>,
    ) -> EventRecord {
        EventRecord(GenericEventRecord {
            id,
            event,
            timestamp: 1000 + id, // mock timestamp that increases with id
            correlation_id,
            user: caller,
        })
    }

    mod save_event_record {
        use super::*;

        #[test]
        fn it_saves_event_record_correctly() {
            EVENT_RECORDS.with(|events| events.borrow_mut().clear());

            let event = Event::add_liquidity_to_pool_started(
                "pool-1".to_string(),
                Some(Nat::from(10u64)),
                Some(Nat::from(20u64)),
            );
            let event_record = create_test_event_record(
                0,
                "corr-1".to_string(),
                event,
                None,
            );

            save_event_record(event_record.clone());

            EVENT_RECORDS.with(|events| {
                let data = events.borrow();
                assert_eq!(data.len(), 1);
                assert_eq!(data[0].0.event.type_str(), "AddLiquidityToPoolStarted");
            });
        }
    }

    mod get_event_records_count {
        use super::*;

        #[test]
        fn it_returns_correct_event_records_count() {
            EVENT_RECORDS.with(|events| {
                events.borrow_mut().clear();
            });
            assert_eq!(get_event_records_count(), 0);

            let event = Event::add_liquidity_to_pool_completed(
                "pool-1".to_string(),
                None,
                None,
            );
            let event_record = create_test_event_record(
                0,
                "corr-1".to_string(),
                event,
                None,
            );
            save_event_record(event_record);

            assert_eq!(get_event_records_count(), 1);
        }
    }

    mod get_event_records {
        use super::*;

        #[test]
        fn it_returns_correct_event_records_slice() {
            EVENT_RECORDS.with(|events| events.borrow_mut().clear());

            for i in 0..5 {
                let event = Event::withdraw_liquidity_from_pool_started(
                    format!("pool-{}", i),
                    Nat::from(100u64),
                    Nat::from(i as u64),
                );
                let event_record = create_test_event_record(
                    i,
                    format!("corr-{}", i),
                    event,
                    None,
                );
                save_event_record(event_record);
            }

            let event_records = get_event_records(1, 2);
            assert_eq!(event_records.len(), 2);
            assert_eq!(event_records[0].0.timestamp, 1001);
            assert_eq!(event_records[1].0.timestamp, 1002);
        }
    }
}
