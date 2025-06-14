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

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Display)]
#[display("{:?}: {} ({})", kind, message, context)]
pub struct InternalError {
    pub kind: InternalErrorKind,
    pub context: String,
    pub message: String,
    pub source: Option<Box<Self>>,
    pub extra: Option<HashMap<String, String>>,
}

impl InternalError {
    pub fn new(
        kind: InternalErrorKind,
        context: String,
        message: String,
        source: Option<Box<Self>>,
        extra: Option<HashMap<String, String>>,
    ) -> Self {
        Self { kind, context, message, source, extra }
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
            Some(Box::new(self.clone())),
            extra
        )
    }

    pub fn business_logic(
        context: String,
        message: String,
        source: Option<Self>,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            InternalErrorKind::BusinessLogic,
            context,
            message,
            source.map(|s| Box::new(s)),
            extra
        )
    }

    pub fn external_service(
        service: String,
        context: String,
        message: String,
        source: Option<Self>,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            InternalErrorKind::ExternalService { service },
            context,
            message,
            source.map(|s| Box::new(s)),
            extra
        )
    }

    pub fn not_found(
        context: String,
        message: String,
        source: Option<Self>,
        extra: Option<HashMap<String, String>>
    ) -> Self {
        Self::new(
            InternalErrorKind::NotFound,
            context,
            message,
            source.map(|s| Box::new(s)),
            extra
        )
    }
}
