use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;
use derive_more::Display;

use crate::internal_error::error::InternalErrorKind;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseErrorCode {
    NotFound,        // NOT_FOUND
    Validation,      // VALIDATION 
    AccessDenied,    // ACCESS_DENIED
    Timeout,         // TIMEOUT
    InternalError,   // INTERNAL_ERROR
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Display)]
#[display("{:?}: {} ({:?})", code, message, details)]
pub struct ResponseError {
    pub code: ResponseErrorCode,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
}

impl ResponseError {
    pub fn new(
        code: ResponseErrorCode,
        message: impl Into<String>,
        details: Option<HashMap<String, String>>
    ) -> Self {
        Self {
            code,
            message: message.into(),
            details,
        }
    }
}

impl From<InternalErrorKind> for ResponseErrorCode {
    fn from(kind: InternalErrorKind) -> Self {
        match kind {
            InternalErrorKind::NotFound => ResponseErrorCode::NotFound,
            InternalErrorKind::Validation => ResponseErrorCode::Validation,
            InternalErrorKind::AccessDenied => ResponseErrorCode::AccessDenied,
            InternalErrorKind::Timeout => ResponseErrorCode::Timeout,
            InternalErrorKind::BusinessLogic
            | InternalErrorKind::ExternalService { service: _ }
            | InternalErrorKind::Infrastructure
            | InternalErrorKind::Unknown => ResponseErrorCode::InternalError,
        }
    }
}
