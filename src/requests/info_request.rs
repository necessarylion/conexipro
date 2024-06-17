use serde::Deserialize;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct InfoRequest {
  #[validate(required(message = "Name is required"))]
  pub name: Option<String>,

  #[validate(
    required(message = "email is required"),
    email(message = "Invalid email format")
  )]
  pub email: Option<String>,
}
