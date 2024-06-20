use std::env;

use crate::errors::handler::IError;

pub mod app;
pub mod db;
pub mod firebase;
pub mod jwt;

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
  if val == None {
    None
  } else {
    Some(val.unwrap())
  }
}
