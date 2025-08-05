use candid::Nat;
use crate::validation_rule::ValidationRule;
use crate::field_validator::FieldValue;

pub struct Positive;

impl Positive {
    pub fn new() -> Self {
        Self
    }
}

impl ValidationRule for Positive {
    fn check_condition(&self, value: &FieldValue) -> bool {
        match value {
            FieldValue::Nat(val) => *val >= Nat::from(0u64),
            _ => false,
        }
    }

    fn error_message(&self, field_name: &str) -> String {
        format!("{} must be positive", field_name)
    }
}
