use candid::Nat;

use crate::validation_rule::ValidationRule;
use crate::field_validator::FieldValue;

pub struct GreaterThan {
    min: Nat,
}

impl GreaterThan {
    pub fn new(min: Nat) -> Self {
        Self { min }
    }
}

impl ValidationRule for GreaterThan {
    fn check_condition(&self, value: &FieldValue) -> bool {
        match value {
            FieldValue::Nat(val) => val > &self.min,
            _ => false,
        }
    }

    fn error_message(&self, field_name: &str) -> String {
        format!("{} must be greater than {}", field_name, self.min)
    }
} 