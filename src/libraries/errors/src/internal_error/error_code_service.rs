use crate::types::error_codes::{
    ErrorCode,
    ModuleCode,
    ErrorKindCode,
    ErrorNumber,
    ErrorCodeParts,
    ModuleCodeParts,
};
pub struct ErrorCodeService {
    module_code: ModuleCode,
}

impl ErrorCodeService {
    pub fn initialize(module_code_parts: ModuleCodeParts) -> Self {
        Self { module_code: module_code_parts.to_code() }
    }

    pub fn build(&self, kind: ErrorKindCode, number: ErrorNumber) -> ErrorCode {
        ErrorCodeParts::new(self.module_code, kind, number).to_code()
    }
}
