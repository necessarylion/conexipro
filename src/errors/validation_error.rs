use std::fmt;

#[derive(Debug)]
pub struct IValidationError {
  pub field: &'static str,
  pub code: String,
  pub msg: String,
}

impl fmt::Display for IValidationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "VError occurred")
  }
}
