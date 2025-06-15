use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;
use derive_more::Display;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum InternalErrorKind {
    NotFound,
    Validation,
    BusinessLogic,
    ExternalService { service: String },
    AccessDenied,
    Infrastructure,
    Timeout,
    Unknown,
}

pub struct InternalErrors {
    pub errors: Vec<InternalError>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Display)]
#[display("{:?}: {} ({})", kind, message, context)]
pub struct InternalError {
    // pub code: String,
    pub kind: InternalErrorKind,
    pub context: String,
    pub message: String,
    pub extra: Option<HashMap<String, String>>,
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

    pub fn wrap(
        &self,
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            self.kind.clone(),
            context,
            message,
            extra
        )
    }

    pub fn business_logic(
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            InternalErrorKind::BusinessLogic,
            context,
            message,
            extra
        )
    }

    pub fn external_service(
        service: String,
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            InternalErrorKind::ExternalService { service },
            context,
            message,
            extra
        )
    }

    pub fn not_found(
        context: String,
        message: String,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            InternalErrorKind::NotFound,
            context,
            message,
            extra
        )
    }
}
