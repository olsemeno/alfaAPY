use super::error::{ResponseError, ResponseErrorCode};
use std::collections::HashMap;

use crate::internal_error::error::InternalError;

pub struct ResponseErrorBuilder {
    code: Option<ResponseErrorCode>,
    message: Option<String>,
    details: Option<HashMap<String, String>>,
}

impl ResponseErrorBuilder {
    pub fn new() -> Self {
        Self {
            code: None,
            message: None,
            details: None,
        }
    }

    // Code setters

    pub fn from_internal_error(internal_error: InternalError) -> Self {
        Self::new().code(internal_error.kind.into())
    }

    pub fn not_found() -> Self {
        Self::new().code(ResponseErrorCode::NotFound)
    }
    pub fn validation() -> Self {
        Self::new().code(ResponseErrorCode::Validation)
    }
    pub fn access_denied() -> Self {
        Self::new().code(ResponseErrorCode::AccessDenied)
    }
    pub fn timeout() -> Self {
        Self::new().code(ResponseErrorCode::Timeout)
    }
    pub fn internal_error() -> Self {
        Self::new().code(ResponseErrorCode::InternalError)
    }

    // Fields setters

    pub fn code(mut self, code: ResponseErrorCode) -> Self {
        self.code = Some(code);
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn details(mut self, details: HashMap<String, String>) -> Self {
        self.details = Some(details);
        self
    }

    pub fn build(self) -> ResponseError {
        ResponseError {
            code: self.code.expect("code is required"),
            message: self.message.unwrap_or_default(),
            details: self.details,
        }
    }
}
