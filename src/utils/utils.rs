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
/// this will return None value if val is none
/// ```
/// let val = to_str(payload.last_name.as_ref())
/// ```
pub fn some_str(val: Option<&String>) -> Option<&str> {
  if val.is_none() {
    None
  } else {
    Some(val.unwrap())
  }
}

/// convert Option<&'a String> to &'a str
/// this will return default value if val is none
/// ```
/// let val = str_default(payload.last_name.as_ref(), "default")
/// ```
pub fn str_default<'a>(val: Option<&'a String>, default: &'a str) -> &'a str {
  if val.is_none() {
    default
  } else {
    val.unwrap()
  }
}

// convert bytes to mb
pub fn bytes_to_mb(size: usize) -> f64 {
  size as f64 / (1024.0 * 1024.0)
}
