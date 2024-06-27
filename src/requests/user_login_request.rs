use serde::Deserialize;
use utoipa::ToSchema;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserLoginRequest {
  #[validate(
    required(message = "Token is required"),
    length(min = 1, message = "Token is required")
  )]
  pub token: Option<String>,
}
