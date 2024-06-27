use serde::Deserialize;
use utoipa::ToSchema;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
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
