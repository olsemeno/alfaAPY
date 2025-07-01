use candid::Nat;
use serde::{Deserialize, Serialize};
use candid::CandidType;

use validation::validation::Validation;
use validation::fields_validator::FieldsValidator;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PositionData {
    pub id: u64,
    pub amount0: Nat,
    pub amount1: Nat,
    pub usd_amount0: Nat,
    pub usd_amount1: Nat,
}

impl Validation for PositionData {
    fn define_validations(&self) -> FieldsValidator {
        FieldsValidator::new()
            .positive("amount0", self.amount0.clone())
            .positive("amount1", self.amount1.clone())
            .positive("usd_amount0", self.usd_amount0.clone())
            .positive("usd_amount1", self.usd_amount1.clone())
            .build()
    }
}