use super::utils::bytes_to_mb;
use crate::IError;
use actix_multipart::form::tempfile::TempFile;
use std::{borrow::Cow, collections::HashMap};
use validator::{ValidationError, ValidationErrors};

/// validate file
pub fn validate_file(field: &'static str, max_mb: f64, input: &TempFile) -> Result<(), IError> {
  if input.size == 0 {
    let mut v_err = ValidationErrors::new();
    let err = ValidationError {
      code: Cow::from("required"),
      message: Some(Cow::from(format!("{} is required", field))),
      params: HashMap::new(),
    };
    v_err.add(field, err);
    return Err(IError::ValidationError(v_err));
  }

  if bytes_to_mb(input.size) >= max_mb {
    let mut v_err = ValidationErrors::new();
    let msg = format!("Max file size: {} mb", max_mb);
    let err = ValidationError {
      code: Cow::from("max_size"),
      message: Some(Cow::from(msg)),
      params: HashMap::new(),
    };
    v_err.add(field, err);
    return Err(IError::ValidationError(v_err));
  }

  Ok(())
}
