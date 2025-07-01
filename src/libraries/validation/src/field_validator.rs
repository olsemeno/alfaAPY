use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use errors::internal_error::error::{InternalError, build_error_code};

use crate::validation_rule_type::ValidationRuleType;

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
                build_error_code(2200, 2, 1), // 2200 02 01
                "FieldValidator::validate".to_string(),
                error_message,
                Some(HashMap::from([
                    ("field_name".to_string(), self.field_name.clone()),
                ]))
            ))
        }
    }
}
