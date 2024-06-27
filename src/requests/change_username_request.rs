use serde::Deserialize;
use utoipa::ToSchema;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangeUsernameRequest {
  #[validate(
    required(message = "Username is required"),
    length(min = 1, message = "Username is required")
  )]
  pub username: Option<String>,
}
