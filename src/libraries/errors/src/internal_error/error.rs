use candid::{CandidType, Deserialize};
use serde::Serialize;
use derive_more::Display;

use crate::internal_error::error_codes::{self};
use crate::response_error::error::{ResponseError, ResponseErrorKind};
use crate::types::error_codes::{
    ErrorCode,
    ErrorKindCode,
    ErrorExtra,
};

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

impl InternalErrorKind {
    pub fn code(&self) -> ErrorKindCode {
        match self {
            InternalErrorKind::NotFound => error_codes::error_kinds::NOT_FOUND.to_string(),
            InternalErrorKind::Validation => error_codes::error_kinds::VALIDATION.to_string(),
            InternalErrorKind::BusinessLogic => error_codes::error_kinds::BUSINESS_LOGIC.to_string(),
            InternalErrorKind::ExternalService => error_codes::error_kinds::EXTERNAL_SERVICE.to_string(),
            InternalErrorKind::AccessDenied => error_codes::error_kinds::ACCESS_DENIED.to_string(),
            InternalErrorKind::Infrastructure => error_codes::error_kinds::INFRASTRUCTURE.to_string(),
            InternalErrorKind::Timeout => error_codes::error_kinds::TIMEOUT.to_string(),
            InternalErrorKind::Unknown => error_codes::error_kinds::UNKNOWN.to_string(),
        }
    }
}

impl From<ResponseErrorKind> for InternalErrorKind {
    fn from(kind: ResponseErrorKind) -> Self {
        match kind {
            ResponseErrorKind::NotFound => InternalErrorKind::NotFound,
            ResponseErrorKind::Validation => InternalErrorKind::Validation,
            ResponseErrorKind::BusinessLogic => InternalErrorKind::BusinessLogic,
            ResponseErrorKind::ExternalService => InternalErrorKind::ExternalService,
            ResponseErrorKind::AccessDenied => InternalErrorKind::AccessDenied,
            ResponseErrorKind::Infrastructure => InternalErrorKind::Infrastructure,
            ResponseErrorKind::Timeout => InternalErrorKind::Timeout,
            ResponseErrorKind::Unknown => InternalErrorKind::Unknown,
        }
    }
}


pub struct InternalErrors {
    pub errors: Vec<InternalError>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Display)]
#[display("{:?}: {} ({})", kind, message, context)]
pub struct InternalError {
    pub code: ErrorCode,
    pub kind: InternalErrorKind,
    pub context: String,
    pub message: String,
    pub extra: ErrorExtra,
}

impl InternalError {
    pub fn new(
        code: ErrorCode,
        kind: InternalErrorKind,
        context: String,
        message: String,
        extra: ErrorExtra,
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
        code: ErrorCode,
        context: String,
        message: String,
        extra: ErrorExtra
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
        code: ErrorCode,
        context: String,
        message: String,
        extra: ErrorExtra
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
        code: ErrorCode,
        context: String,
        message: String,
        extra: ErrorExtra
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
        code: ErrorCode,
        context: String,
        message: String,
        extra: ErrorExtra
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

#[macro_export]
macro_rules! error_extra {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut m = ::std::collections::HashMap::<String, String>::new();
        $( m.insert($k.to_string(), format!("{:?}", $v)); )*
        Some(m)
    }};
}