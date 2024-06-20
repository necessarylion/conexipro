use serde::Deserialize;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserUpdateRequest {
  #[validate(
    required(message = "First name is required"),
    length(min = 1, message = "First name is required")
  )]
  pub first_name: Option<String>,

  pub last_name: Option<String>,

  pub middle_name: Option<String>,

  pub display_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangeUsernameRequest {
  #[validate(
    required(message = "First name is required"),
    length(min = 1, message = "First name is required")
  )]
  pub username: Option<String>,
}
