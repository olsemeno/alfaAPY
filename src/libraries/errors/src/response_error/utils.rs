use super::error::ResponseError;
use super::builder::ResponseErrorBuilder;

pub fn response_error_internal_error<T>(message: impl Into<String>) -> Result<T, ResponseError> {
    Err(ResponseErrorBuilder::internal_error().message(message).build())
}

pub fn response_error_not_found<T>(message: impl Into<String>) -> Result<T, ResponseError> {
    Err(ResponseErrorBuilder::not_found().message(message).build())
}
