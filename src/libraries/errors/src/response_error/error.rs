use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;
use derive_more::Display;

use crate::internal_error::error::{InternalError, InternalErrorKind};
use crate::response_error::builder::ResponseErrorBuilder;

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

impl ResponseError {
    pub fn from_internal_error<T>(internal_error: InternalError) -> Result<T, ResponseError> {
        Err(ResponseErrorBuilder::from_internal_error(internal_error).build())
    }

    pub fn internal_error<T>(message: impl Into<String>) -> Result<T, ResponseError> {
        Err(ResponseErrorBuilder::internal_error().message(message).build())
    }

    pub fn not_found<T>(message: impl Into<String>) -> Result<T, ResponseError> {
        Err(ResponseErrorBuilder::not_found().message(message).build())
    }
}