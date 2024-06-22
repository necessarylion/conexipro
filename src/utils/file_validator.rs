use super::utils::bytes_to_mb;
use crate::{errors::IValidationError, IError};
use actix_multipart::form::tempfile::TempFile;

/// validate file
pub fn validate_file(field: &'static str, max_mb: f64, input: &TempFile) -> Result<(), IError> {
  if input.size == 0 {
    return Err(IError::CustomValidatoinError(IValidationError {
      field,
      code: String::from("required"),
      msg: format!("{} is required", field),
    }));
  }

  if bytes_to_mb(input.size) >= max_mb {
    return Err(IError::CustomValidatoinError(IValidationError {
      field,
      code: String::from("max_size"),
      msg: format!("Max file size: {} mb", max_mb),
    }));
  }
  Ok(())
}
