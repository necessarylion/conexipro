use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::errors::handler::IError;

use super::get_env;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  sub: String,
  exp: usize,
}

pub fn create_token(uid: &String) -> Result<String, IError> {
  let claims = Claims {
    sub: uid.to_string(),
    exp: 1000,
  };
  let secret = get_env("JWT_SECRET")?;
  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  )
  .map_err(|err| IError::ServerError(err.to_string()))
}
