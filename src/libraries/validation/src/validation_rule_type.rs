use candid::Nat;
use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::validation_rules::greater_than::GreaterThan;
use crate::validation_rules::greater_than_or_equal::GreaterThanOrEqual;
use crate::validation_rules::less_than::LessThan;
use crate::validation_rules::less_than_or_equal::LessThanOrEqual;
use crate::validation_rules::not_zero::NotZero;
use crate::validation_rules::positive::Positive;
use crate::validation_rules::required::Required;
use crate::validation_rule::ValidationRule;
use crate::field_validator::FieldValue;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    GreaterThan { min: Nat },
    GreaterThanOrEqual { min: Nat },
    LessThan { max: Nat },
    LessThanOrEqual { max: Nat },
    NotZero,
    Positive,
    Required,
}

impl ValidationRuleType {

    pub fn as_rule(&self) -> Box<dyn ValidationRule> {
        match self {
            ValidationRuleType::GreaterThan { min } => Box::new(GreaterThan::new(min.clone())),
            ValidationRuleType::GreaterThanOrEqual { min } => Box::new(GreaterThanOrEqual::new(min.clone())),
            ValidationRuleType::LessThan { max } => Box::new(LessThan::new(max.clone())),
            ValidationRuleType::LessThanOrEqual { max } => Box::new(LessThanOrEqual::new(max.clone())),
            ValidationRuleType::NotZero => Box::new(NotZero::new()),
            ValidationRuleType::Positive => Box::new(Positive::new()),
            ValidationRuleType::Required => Box::new(Required::new()),
        }
    }

    pub fn validate(&self, field_name: &str, value: Option<&FieldValue>) -> Result<(), String> {
        let rule = self.as_rule();
        match value {
            Some(value) if rule.check_condition(value) => Ok(()),
            Some(_value) => Err(rule.error_message(field_name)),
            None => Err(format!("Field '{}' is missing", field_name)),
        }
    }
} 
