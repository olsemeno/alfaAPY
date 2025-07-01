use candid::Nat;
use serde::{Deserialize, Serialize};
use candid::CandidType;
use validation::fields_validator::FieldsValidator;
use validation::validation::Validation;

#[derive(CandidType, Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct PoolData {
    pub tvl: Nat,
    // pub balance0: Nat,
    // pub balance1: Nat,
    // pub lp_fee_0: Nat,
    // pub lp_fee_1: Nat,
}

impl Validation for PoolData {
    fn define_validations(&self) -> FieldsValidator {
        FieldsValidator::new()
            .positive("tvl", self.tvl.clone())
            .build()
    }
} 
