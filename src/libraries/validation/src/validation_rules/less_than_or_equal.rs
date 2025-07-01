use candid::Nat;

use crate::validation_rule::ValidationRule;
use crate::field_validator::FieldValue;

pub struct LessThanOrEqual {
    max: Nat,
}

impl LessThanOrEqual {
    pub fn new(max: Nat) -> Self {
        Self { max }
    }
}

impl ValidationRule for LessThanOrEqual {
    fn check_condition(&self, value: &FieldValue) -> bool {
        match value {
            FieldValue::Nat(val) => val <= &self.max,
            _ => false,
        }
    }

    fn error_message(&self, field_name: &str) -> String {
        format!("{} must be less than or equal to {}", field_name, self.max)
    }
} 