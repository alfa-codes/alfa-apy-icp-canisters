use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;
use derive_more::Display;

use crate::internal_error::error::{InternalError, InternalErrorKind};
use crate::types::error_codes::ErrorCode;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseErrorKind {
    NotFound,
    Validation,
    BusinessLogic,
    ExternalService,
    AccessDenied,
    Infrastructure,
    Timeout,
    Unknown,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, Display)]
#[display("{:?}: {} ({:?})", kind, message, details)]
pub struct ResponseError {
    pub code: ErrorCode,
    pub kind: ResponseErrorKind,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
}

impl ResponseError {
    pub fn new(
        code: ErrorCode,
        kind: ResponseErrorKind,
        message: impl Into<String>,
        details: Option<HashMap<String, String>>
    ) -> Self {
        Self {
            code,
            kind,
            message: message.into(),
            details,
        }
    }

    pub fn from_internal_error(internal_error: InternalError) -> Self {
        Self::new(
            internal_error.code,
            internal_error.kind.clone().into(),
            internal_error.message.clone(),
            internal_error.extra.clone()
        )
    }
}

impl From<InternalErrorKind> for ResponseErrorKind {
    fn from(kind: InternalErrorKind) -> Self {
        match kind {
            InternalErrorKind::NotFound => ResponseErrorKind::NotFound,
            InternalErrorKind::Validation => ResponseErrorKind::Validation,
            InternalErrorKind::AccessDenied => ResponseErrorKind::AccessDenied,
            InternalErrorKind::Infrastructure => ResponseErrorKind::Infrastructure,
            InternalErrorKind::Timeout => ResponseErrorKind::Timeout,
            InternalErrorKind::BusinessLogic => ResponseErrorKind::BusinessLogic,
            InternalErrorKind::ExternalService => ResponseErrorKind::ExternalService,
            InternalErrorKind::Unknown => ResponseErrorKind::Unknown,
        }
    }
}
