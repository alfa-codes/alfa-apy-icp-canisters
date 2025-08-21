use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

use errors::internal_error::error::{InternalError, InternalErrorKind};
use errors::internal_error::error_codes::module::areas::{
    libraries as library_area,
    libraries::domains::validation as validation_domain,
    libraries::domains::validation::components as validation_domain_components,
};

use crate::validation_rule_type::ValidationRuleType;

// Module code: "02-03-01"
errors::define_error_code_builder_fn!(
    build_error_code,
    library_area::AREA_CODE,           // Area code: "02"
    validation_domain::DOMAIN_CODE,    // Domain code: "03"
    validation_domain_components::CORE // Component code: "01"
);

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct FieldValidator {
    pub field_name: String,
    pub value: Option<FieldValue>,
    pub validation_rule: ValidationRuleType,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum FieldValue {
    Nat(Nat),
    Bool(bool),
    Text(String),
    Principal(Principal),
    None,
}

impl FieldValidator {
    pub fn new(field_name: &str, value: Option<FieldValue>, validation_rule: ValidationRuleType) -> Self {
        Self {
            field_name: field_name.to_string(),
            value,
            validation_rule,
        }
    }

    pub fn validate(&self) -> Result<(), InternalError> {
        let value = self.value.as_ref();
        match self.validation_rule.validate(&self.field_name, value) {
            Ok(()) => Ok(()),
            Err(error_message) => Err(InternalError::validation(
                build_error_code(InternalErrorKind::Validation, 1), // Error code: "02-03-01 02 01"
                "FieldValidator::validate".to_string(),
                error_message,
                errors::error_extra! {
                    "field_name" => self.field_name.clone(),
                }
            ))
        }
    }
}
