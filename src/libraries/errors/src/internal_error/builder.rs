use candid::{CandidType, Deserialize};
use std::collections::HashMap;
use serde::Serialize;

use crate::internal_error::error::{InternalError, InternalErrorKind};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct InternalErrorBuilder {
    kind: Option<InternalErrorKind>,
    context: Option<String>,
    message: Option<String>,
    extra: Option<HashMap<String, String>>,
}

impl InternalErrorBuilder {
    pub fn new() -> Self {
        Self {
            kind: None,
            context: None,
            message: None,
            extra: None,
        }
    }

    // Kind setters

    pub fn not_found() -> Self {
        Self {
            kind: Some(InternalErrorKind::NotFound),
            context: None,
            message: None,
            extra: None,
        }
    }

    pub fn business_logic() -> Self {
        Self {
            kind: Some(InternalErrorKind::BusinessLogic),
            context: None,
            message: None,
            extra: None,
        }
    }
    pub fn external_service(service: String) -> Self {
        Self {
            kind: Some(InternalErrorKind::ExternalService { service }),
            context: None,
            message: None,
            extra: None,
        }
    }
    pub fn validation() -> Self {
        Self {
            kind: Some(InternalErrorKind::Validation),
            context: None,
            message: None,
            extra: None,
        }
    }
    pub fn access_denied() -> Self {
        Self {
            kind: Some(InternalErrorKind::AccessDenied),
            context: None,
            message: None,
            extra: None,
        }
    }
    pub fn infrastructure() -> Self {
        Self {
            kind: Some(InternalErrorKind::Infrastructure),
            context: None,
            message: None,
            extra: None,
        }
    }
    pub fn timeout() -> Self {
        Self {
            kind: Some(InternalErrorKind::Timeout),
            context: None,
            message: None,
            extra: None,
        }
    }
    pub fn unknown() -> Self {
        Self {
            kind: Some(InternalErrorKind::Unknown),
            context: None,
            message: None,
            extra: None,
        }
    }
    
    // Fields setters

    pub fn kind(mut self, kind: InternalErrorKind) -> Self {
        self.kind = Some(kind);
        self
    }
    pub fn context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    pub fn extra(mut self, extra: HashMap<String, String>) -> Self {
        self.extra = Some(extra);
        self
    }

    // Build
    pub fn build(self) -> InternalError {
        InternalError {
            kind: self.kind.expect("kind is required"),
            context: self.context.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
            extra: self.extra,
        }
    }
}
