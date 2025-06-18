use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;
use derive_more::Display;

use crate::internal_error::error::{InternalError, InternalErrorKind};

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseErrorKind {
    NotFound,
    Validation,
    BusinessLogic,
    ExternalService,
    AccessDenied,
    Timeout,
    Unknown,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, Display)]
#[display("{:?}: {} ({:?})", kind, message, details)]
pub struct ResponseError {
    pub code: u32,
    pub kind: ResponseErrorKind,
    pub message: String,
    pub source: Option<Box<InternalError>>,
    pub details: Option<HashMap<String, String>>,
}

impl ResponseError {
    pub fn new(
        code: u32,
        kind: ResponseErrorKind,
        message: impl Into<String>,
        source: Option<Box<InternalError>>,
        details: Option<HashMap<String, String>>
    ) -> Self {
        Self {
            code,
            kind,
            message: message.into(),
            source,
            details,
        }
    }

    pub fn from_internal_error(internal_error: InternalError) -> Self {
        Self::new(
            internal_error.code,
            internal_error.kind.clone().into(),
            internal_error.message.clone(),
            Some(Box::new(internal_error.clone())),
            internal_error.extra.clone()
        )
    }

    pub fn err_from_internal<T>(internal_error: InternalError) -> Result<T, Self> {
        Err(Self::from_internal_error(internal_error))
    }
}

impl From<InternalErrorKind> for ResponseErrorKind {
    fn from(kind: InternalErrorKind) -> Self {
        match kind {
            InternalErrorKind::NotFound => ResponseErrorKind::NotFound,
            InternalErrorKind::Validation => ResponseErrorKind::Validation,
            InternalErrorKind::AccessDenied => ResponseErrorKind::AccessDenied,
            InternalErrorKind::Timeout => ResponseErrorKind::Timeout,
            InternalErrorKind::BusinessLogic => ResponseErrorKind::BusinessLogic,
            InternalErrorKind::ExternalService => ResponseErrorKind::ExternalService,
            InternalErrorKind::Unknown => ResponseErrorKind::Unknown,
        }
    }
}
