use serde::Deserialize;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserRegisterRequest {
  #[validate(
    required(message = "Token is required"),
    length(min = 1, message = "Token is required")
  )]
  pub token: Option<String>,
}
