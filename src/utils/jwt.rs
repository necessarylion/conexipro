use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::handler::IError;

use super::get_env;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
  pub token: String,
  pub token_type: String,
  pub expired_date: DateTime<Utc>,
}

pub fn create(uid: &String) -> Result<JwtToken, IError> {
  let exp_duration = Duration::days(5);

  // Get the current time and add the expiration duration
  let expired_date: DateTime<Utc> = Utc::now() + exp_duration;

  let claims = Claims {
    sub: uid.to_string(),
    exp: expired_date.timestamp() as usize,
  };
  let secret = get_env("JWT_SECRET")?;
  let token = encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  )
  .map_err(|err| IError::ServerError(err.to_string()))?;

  Ok(JwtToken {
    token,
    token_type: String::from("Bearer"),
    expired_date,
  })
}

pub fn verify(token: String) -> Result<Claims, IError> {
  let secret = get_env("JWT_SECRET")?;
  let decoded_result = decode::<Claims>(
    &token,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::default(),
  )
  .map_err(|_| IError::ServerError("Invalid Bearer Token".to_string()))?;
  Ok(decoded_result.claims)
}
