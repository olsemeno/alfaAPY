use errors::internal_error::error::InternalError;

use crate::fields_validator::FieldsValidator;

pub trait Validation {
    fn define_validations(&self) -> FieldsValidator;

    fn validate(&self) -> Result<(), Vec<InternalError>> {
        self.define_validations().validate()
    }
}
