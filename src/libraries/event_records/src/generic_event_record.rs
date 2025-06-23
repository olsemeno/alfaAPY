use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use errors::internal_error::error::InternalError;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GenericEventRecord<TEvent> {
    pub id: u64,
    pub timestamp: u64,
    pub event: TEvent,
    pub correlation_id: String,
    pub user: Option<Principal>,
    pub error: Option<InternalError>,
}
