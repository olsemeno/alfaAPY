use candid::CandidType;
use serde::{Deserialize, Serialize};
use candid::Principal;

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
            correlation_id: Self::generate_correlation_id(),
            user,
        }
    }

    fn generate_correlation_id() -> String {
        // TODO: replace with uuid or another library
        ic_cdk::api::time().to_string()
    }
}
