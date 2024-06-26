use crate::IError;
use std::env;

/// get env variable
/// ```
/// let val: String = get_env("APP_KEY")?
/// ```
pub fn get_env(key: &str) -> Result<String, IError> {
  let val =
    env::var(key).map_err(|e| IError::ServerError(format!("{}: {}", key, e.to_string())))?;
  Ok(val)
}

/// convert Option<&String> to Option<&str>
/// ```
/// let val = to_str(payload.last_name.as_ref())
/// ```
pub fn to_str(val: Option<&String>) -> Option<&str> {
  if val.is_none() {
    None
  } else {
    Some(val.unwrap())
  }
}

// convert bytes to mb
pub fn bytes_to_mb(size: usize) -> f64 {
  size as f64 / (1024.0 * 1024.0)
}
