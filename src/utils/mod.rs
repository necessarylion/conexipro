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
