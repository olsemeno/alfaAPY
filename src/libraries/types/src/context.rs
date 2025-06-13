use candid::CandidType;
use serde::{Deserialize, Serialize};
use candid::Principal;
use uuid::Uuid;

pub type CorrelationId = String;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Context {
    pub correlation_id: CorrelationId,
    pub user: Option<Principal>,
}

impl Context {
    pub fn new(correlation_id: CorrelationId, user: Option<Principal>) -> Self {
        Self { correlation_id, user }
    }

    pub fn generate(user: Option<Principal>) -> Self {
        Self {
            correlation_id: Uuid::new_v4().to_string(),
            user,
        }
    }
}
