use candid::Nat;
use serde::{Deserialize, Serialize};
use candid::CandidType;

use errors::internal_error::error::InternalError;

use crate::validation_rule_type::ValidationRuleType;
use crate::field_validator::{FieldValidator, FieldValue};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct FieldsValidator {
    validators: Vec<FieldValidator>,
}


impl FieldsValidator {
    pub fn new() -> Self {
        Self { validators: Vec::new() }
    }

    pub fn add_validator(mut self, validator: FieldValidator) -> Self {
        self.validators.push(validator);
        self
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn required(mut self, field_name: &str, value: Option<FieldValue>) -> Self {
        self = self.add_validator(FieldValidator {
            field_name: field_name.to_string(),
            validation_rule: ValidationRuleType::Required,
            value
        });
        self
    }

    pub fn positive(mut self, field_name: &str, value: Nat) -> Self {
        self = self.add_validator(FieldValidator {
            field_name: field_name.to_string(),
            validation_rule: ValidationRuleType::Positive,
            value: Some(FieldValue::Nat(value)),
        });
        self
    }

    pub fn not_zero(mut self, field_name: &str, value: Nat) -> Self {
        self = self.add_validator(FieldValidator {
            field_name: field_name.to_string(),
            validation_rule: ValidationRuleType::NotZero,
            value: Some(FieldValue::Nat(value)),
        });
        self
    }

    pub fn greater_than(mut self, field_name: &str, min: Nat, value: Nat) -> Self {
        self = self.add_validator(FieldValidator {
            field_name: field_name.to_string(),
            validation_rule: ValidationRuleType::GreaterThan { min },
            value: Some(FieldValue::Nat(value)),
        });
        self
    }

    pub fn greater_than_or_equal(mut self, field_name: &str, min: Nat, value: Nat) -> Self {
        self = self.add_validator(FieldValidator {
            field_name: field_name.to_string(),
            validation_rule: ValidationRuleType::GreaterThanOrEqual { min },
            value: Some(FieldValue::Nat(value)),
        });
        self
    }

    pub fn less_than(mut self, field_name: &str, max: Nat, value: Nat) -> Self {
        self = self.add_validator(FieldValidator {
            field_name: field_name.to_string(),
            validation_rule: ValidationRuleType::LessThan { max },
            value: Some(FieldValue::Nat(value)),
        });
        self
    }

    pub fn less_than_or_equal(mut self, field_name: &str, max: Nat, value: Nat) -> Self {
        self = self.add_validator(FieldValidator {
            field_name: field_name.to_string(),
            validation_rule: ValidationRuleType::LessThanOrEqual { max },
            value: Some(FieldValue::Nat(value)),
        });
        self
    }

    pub fn validate(self) -> Result<(), Vec<InternalError>> {
        let mut errors = Vec::new();

        for validator in self.validators {
            if let Err(error) = validator.validate() {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}