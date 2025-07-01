use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;
use derive_more::Display;

use crate::response_error::error::{ResponseError, ResponseErrorKind};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum InternalErrorKind {
    NotFound,
    Validation,
    BusinessLogic,
    ExternalService,
    AccessDenied,
    Timeout,
    Unknown,
}

pub struct InternalErrors {
    pub errors: Vec<InternalError>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Display)]
#[display("{:?}: {} ({})", kind, message, context)]
pub struct InternalError {
    pub code: u32,
    pub kind: InternalErrorKind,
    pub context: String,
    pub message: String,
    pub extra: Option<HashMap<String, String>>,
}

impl InternalError {
    pub fn new(
        code: u32,
        kind: InternalErrorKind,
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>,
    ) -> Self {
        Self { code, kind, context, message, extra }
    }

    pub fn from_response_error(response_error: ResponseError, context: String) -> Self {
        Self::new(
            response_error.code,
            response_error.kind.into(),
            context,
            response_error.message,
            response_error.details,
        )
    }

    pub fn business_logic(
        code: u32,
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            code,
            InternalErrorKind::BusinessLogic,
            context,
            message,
            extra
        )
    }

    pub fn external_service(
        code: u32,
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            code,
            InternalErrorKind::ExternalService,
            context,
            message,
            extra
        )
    }

    pub fn not_found(
        code: u32,
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            code,
            InternalErrorKind::NotFound,
            context,
            message,
            extra
        )
    }

    pub fn validation(
        code: u32,
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            code,
            InternalErrorKind::Validation,
            context,
            message,
            extra
        )
    }
}


pub fn build_error_code(module: u16, kind: u8, number: u8) -> u32 {
    // Format: MMMM KKK NNN
    // MMMM: Module (4 digits)
    // KKK: Kind (2 digits)
    // NNN: Number (2 digits)
    // Example: module=1001, kind=4, number=1 -> 10010401
    (module as u32) * 10_000 + (kind as u32) * 100 + (number as u32)
}


impl From<ResponseErrorKind> for InternalErrorKind {
    fn from(kind: ResponseErrorKind) -> Self {
        match kind {
            ResponseErrorKind::NotFound => InternalErrorKind::NotFound,
            ResponseErrorKind::Validation => InternalErrorKind::Validation,
            ResponseErrorKind::BusinessLogic => InternalErrorKind::BusinessLogic,
            ResponseErrorKind::ExternalService => InternalErrorKind::ExternalService,
            ResponseErrorKind::AccessDenied => InternalErrorKind::AccessDenied,
            ResponseErrorKind::Timeout => InternalErrorKind::Timeout,
            ResponseErrorKind::Unknown => InternalErrorKind::Unknown,
        }
    }
}