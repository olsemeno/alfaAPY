use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;
use derive_more::Display;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum InternalErrorKind {
    NotFound,
    Validation,
    BusinessLogic,
    ExternalService,
    AccessDenied,
    Infrastructure,
    Timeout,
    Unknown,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Display)]
#[display("{:?}: {} ({})", kind, message, context)]
pub struct InternalError {
    pub kind: InternalErrorKind,
    pub context: String,
    pub message: String,
    pub extra: Option<HashMap<String, String>>,
    // pub service: Option<String>,
    // pub component: Option<String>,
}

impl InternalError {
    pub fn new(
        kind: InternalErrorKind,
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>,
    ) -> Self {
        Self { kind, context, message, extra }
    }
}
