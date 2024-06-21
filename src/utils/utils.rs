use std::env;

use serde::Serializer;

use crate::IError;

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

/// get app url
pub fn app_url() -> String {
  get_env("APP_URL").unwrap_or(String::from(""))
}

/// get file url
pub fn get_file_url(file_name: &str) -> String {
  format!("{}/files/{}", app_url(), file_name)
}

/// add full file url Serializer method
pub fn full_file_url<S>(value: &Option<String>, slz: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  match value {
    Some(ref val) => slz.serialize_some(&get_file_url(val)),
    None => slz.serialize_none(),
  }
}
