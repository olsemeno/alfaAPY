use candid::Nat;

use crate::validation_rule::ValidationRule;
use crate::field_validator::FieldValue;

pub struct LessThan {
    max_value: Nat,
}

impl LessThan {
    pub fn new(max_value: Nat) -> Self {
        Self { max_value }
    }
}

impl ValidationRule for LessThan {
    fn check_condition(&self, value: &FieldValue) -> bool {
        match value {
            FieldValue::Nat(val) => val < &self.max_value,
            _ => false,
        }
    }

    fn error_message(&self, field_name: &str) -> String {
        format!("{} must be less than {}", field_name, self.max_value)
    }
} 