use std::env;

use crate::errors::handler::IError;

pub mod app;
pub mod db;
pub mod firebase;
pub mod jwt;

pub fn get_env(key: &str) -> Result<String, IError> {
  let val =
    env::var(key).map_err(|e| IError::ServerError(format!("{}: {}", key, e.to_string())))?;
  Ok(val)
}

pub fn to_str(val: Option<&String>) -> Option<&str> {
  if val == None {
    None
  } else {
    Some(val.unwrap())
  }
}
