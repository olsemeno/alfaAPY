use crate::validation_rule::ValidationRule;
use crate::field_validator::FieldValue;

pub struct Required;

impl Required {
    pub fn new() -> Self {
        Self
    }
}

impl ValidationRule for Required {
    fn check_condition(&self, value: &FieldValue) -> bool {
        !matches!(value, FieldValue::None)
    }

    fn error_message(&self, field_name: &str) -> String {
        format!("{} is required", field_name)
    }
}