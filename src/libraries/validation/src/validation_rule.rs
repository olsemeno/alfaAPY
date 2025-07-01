use crate::field_validator::FieldValue;

pub trait ValidationRule {
    fn check_condition(&self, value: &FieldValue) -> bool;
    fn error_message(&self, field_name: &str) -> String;
} 
