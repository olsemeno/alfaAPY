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

#[cfg(test)]
mod tests {
    use super::*;
    use types::CanisterId;
    use candid::Nat;
    use std::time::{SystemTime, UNIX_EPOCH};

    use event_records::generic_event_record::GenericEventRecord;
    use errors::internal_error::error::{InternalError, build_error_code};

    use crate::types::types::{ListItemsPaginationRequest, SortOrder};
    use crate::event_records::event_record::{EventRecord, Event};

    fn mock_event_with_type(event_type: &str, timestamp: u64) -> EventRecord {
        let event = match event_type {
            "AddLiquidityToPoolStarted" => Event::add_liquidity_to_pool_started(
                "pool1".to_string(),
                Some(Nat::from(1000_u64)),
                Some(Nat::from(2000_u64)),
            ),
            "StrategyDepositStarted" => Event::strategy_deposit_started(
                "strategy1".to_string(),
                Some("pool1".to_string()),
                Some(Nat::from(100_u64)),
            ),
            _ => Event::swap_token_failed(
                "poolX".to_string(),
                CanisterId::from_text("aaaaa-aa").unwrap(),
                CanisterId::from_text("bbbbb-bb").unwrap(),
                Some(Nat::from(1000_u64)),
                InternalError::not_found(
                    build_error_code(0000, 00, 00),
                    "test".to_string(),
                    "fail".to_string(),
                    None,
                ),
            ),
        };

        EventRecord(GenericEventRecord {
            id: 0,
            timestamp,
            event,
            correlation_id: "".to_string(),
            user: None,
        })
    }

    fn timestamp(offset_secs: u64) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + offset_secs
    }

    mod save_event_record {
        use super::*;

        #[test]
        fn saves_event_correctly() {
            EVENT_RECORDS.with(|r| r.borrow_mut().clear());

            let event = mock_event_with_type(
                "StrategyDepositStarted",
                111,
            );
            save_event_record(event.clone());

            let count = get_event_records_count();
            assert_eq!(count, 1);

            let all = get_event_records(ListItemsPaginationRequest {
                page: 1,
                page_size: 10,
                search: None,
                sort_order: SortOrder::Asc,
            });

            assert_eq!(all.len(), 1);
            assert_eq!(all[0].0.timestamp, 111);
            assert_eq!(all[0].0.event.type_str(), "StrategyDepositStarted");
        }
    }

    mod get_event_records_count {
        use super::*;

        #[test]
        fn returns_correct_count() {
            EVENT_RECORDS.with(|r| r.borrow_mut().clear());

            let event1 = mock_event_with_type("AddLiquidityToPoolStarted", 1);
            let event2 = mock_event_with_type("AddLiquidityToPoolStarted", 2);

            save_event_record(event1);
            save_event_record(event2);

            assert_eq!(get_event_records_count(), 2);
        }
    }

    mod get_event_records {
        use super::*;

        #[test]
        fn returns_paginated_records_sorted_asc() {
            EVENT_RECORDS.with(|r| r.borrow_mut().clear());

            let event1 = mock_event_with_type("AddLiquidityToPoolStarted", 20);
            let event2 = mock_event_with_type("StrategyDepositStarted", 10);

            save_event_record(event1);
            save_event_record(event2);

            let result = get_event_records(ListItemsPaginationRequest {
                page: 1,
                page_size: 10,
                search: None,
                sort_order: SortOrder::Asc,
            });

            assert_eq!(result.len(), 2);
            assert_eq!(result[0].0.timestamp, 10);
            assert_eq!(result[1].0.timestamp, 20);
        }

        #[test]
        fn returns_paginated_records_sorted_desc() {
            EVENT_RECORDS.with(|r| r.borrow_mut().clear());

            let event1 = mock_event_with_type("StrategyDepositStarted", 5);
            let event2 = mock_event_with_type("AddLiquidityToPoolStarted", 10);

            save_event_record(event1);
            save_event_record(event2);

            let result = get_event_records(ListItemsPaginationRequest {
                page: 1,
                page_size: 10,
                search: None,
                sort_order: SortOrder::Desc,
            });

            assert_eq!(result.len(), 2);
            assert_eq!(result[0].0.timestamp, 10);
            assert_eq!(result[1].0.timestamp, 5);
        }

        #[test]
        fn filters_by_event_type() {
            EVENT_RECORDS.with(|r| r.borrow_mut().clear());

            let event1 = mock_event_with_type("StrategyDepositStarted", 1);
            let event2 = mock_event_with_type("AddLiquidityToPoolStarted", 2);

            save_event_record(event1);
            save_event_record(event2);

            let result = get_event_records(ListItemsPaginationRequest {
                page: 1,
                page_size: 10,
                search: Some("Deposit".to_string()),
                sort_order: SortOrder::Asc,
            });

            assert_eq!(result.len(), 1);
            assert_eq!(result[0].0.event.type_str(), "StrategyDepositStarted");
        }

        #[test]
        fn handles_pagination_correctly() {
            EVENT_RECORDS.with(|r| r.borrow_mut().clear());

            for i in 1..=15 {
                let event = mock_event_with_type("AddLiquidityToPoolStarted", i);
                save_event_record(event);
            }

            let page1 = get_event_records(ListItemsPaginationRequest {
                page: 1,
                page_size: 5,
                search: None,
                sort_order: SortOrder::Asc,
            });

            let page2 = get_event_records(ListItemsPaginationRequest {
                page: 2,
                page_size: 5,
                search: None,
                sort_order: SortOrder::Asc,
            });

            assert_eq!(page1.len(), 5);
            assert_eq!(page2.len(), 5);
            assert!(page1[0].0.timestamp < page2[0].0.timestamp);
        }

        #[test]
        fn sort_order_asc_and_desc() {
            EVENT_RECORDS.with(|events| events.borrow_mut().clear());

            let ts1 = timestamp(10);
            let ts2 = timestamp(20);
            let ts3 = timestamp(30);

            let event1 = mock_event_with_type("AddLiquidityToPoolStarted", ts2);
            let event2 = mock_event_with_type("AddLiquidityToPoolStarted", ts3);
            let event3 = mock_event_with_type("AddLiquidityToPoolStarted", ts1);

            save_event_record(event1);
            save_event_record(event2);
            save_event_record(event3);

            let request_asc = ListItemsPaginationRequest {
                page: 1,
                page_size: 10,
                sort_order: SortOrder::Asc,
                search: None,
            };

            let request_desc = ListItemsPaginationRequest {
                page: 1,
                page_size: 10,
                sort_order: SortOrder::Desc,
                search: None,
            };

            let asc = get_event_records(request_asc);
            let desc = get_event_records(request_desc);

            assert_eq!(asc[0].0.timestamp, ts1);
            assert_eq!(asc[1].0.timestamp, ts2);
            assert_eq!(asc[2].0.timestamp, ts3);

            assert_eq!(desc[0].0.timestamp, ts3);
            assert_eq!(desc[1].0.timestamp, ts2);
            assert_eq!(desc[2].0.timestamp, ts1);
        }
    }
}
