use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;

pub type ErrorCode = u64;
pub type ModuleCode = u64;
pub type AreaCode = String;
pub type DomainCode = String;
pub type ComponentCode = String;
pub type ErrorKindCode = String;
pub type ErrorNumber = u8;

pub type ErrorExtraMap = HashMap<String, String>;
pub type ErrorExtra = Option<ErrorExtraMap>;

// ErrorCodeParts 

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct ErrorCodeParts {
    pub module_code: ModuleCode,
    pub kind_code: ErrorKindCode,
    pub number: ErrorNumber,
}

impl ErrorCodeParts {
    pub fn new(
        module_code: ModuleCode,
        kind_code: ErrorKindCode,  
        number: ErrorNumber  
    ) -> Self {
        Self { module_code, kind_code, number }
    }

    pub fn to_code(&self) -> ErrorCode {
        format!("{:06}{:02}{:02}", self.module_code, self.kind_code, self.number)
            .parse::<ErrorCode>()
            .unwrap()
    }
}

// ModuleCodeParts

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct ModuleCodeParts {
    pub area_code: AreaCode,
    pub domain_code: DomainCode,
    pub component_code: ComponentCode,
}

impl ModuleCodeParts {
    pub fn new(
        area_code: AreaCode,
        domain_code: DomainCode,
        component_code: ComponentCode,
    ) -> Self {
        Self { area_code, domain_code, component_code }
    }

    pub fn to_code(&self) -> ModuleCode {
        format!("{:02}{:02}{:02}", self.area_code, self.domain_code, self.component_code)
            .parse::<ModuleCode>()
            .unwrap()
    }
}

#[macro_export]
macro_rules! define_error_code_builder_fn {
    ($fn_name:ident, $area_code:expr, $domain_code:expr, $component_code:expr) => {
        #[inline]
        pub fn $fn_name(
            kind_code: $crate::internal_error::error::InternalErrorKind,
            number: $crate::types::error_codes::ErrorNumber,
        ) -> $crate::types::error_codes::ErrorCode {
            static ERROR_CODE_SERVICE: std::sync::OnceLock<
                $crate::internal_error::error_code_service::ErrorCodeService
            > = std::sync::OnceLock::new();

            let error_code_service = ERROR_CODE_SERVICE.get_or_init(|| {
                $crate::internal_error::error_code_service::ErrorCodeService::initialize(
                    $crate::types::error_codes::ModuleCodeParts::new(
                        $area_code.to_string(),
                        $domain_code.to_string(),
                        $component_code.to_string(),
                    )
                )
            });

            error_code_service.build(kind_code.code().to_string(), number)
        }
    };
}
