use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use errors::internal_error::error::InternalError;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GenericEventLog<TEventLogType, TEventLogParams> {
    pub id: u64,
    pub timestamp: u64,
    pub event_type: TEventLogType,
    pub params: TEventLogParams,
    pub correlation_id: String,
    pub user: Option<Principal>,
    pub error: Option<InternalError>,
}
