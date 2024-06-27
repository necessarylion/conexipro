use serde::Deserialize;
use utoipa::ToSchema;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserUpdateRequest {
  #[validate(
    required(message = "First name is required"),
    length(min = 1, message = "First name is required")
  )]
  #[schema(example = "Zin Kyaw", required = true)]
  pub first_name: Option<String>,

  #[schema(example = "Kyaw", required = false)]
  pub last_name: Option<String>,

  #[schema(example = "", required = false)]
  pub middle_name: Option<String>,

  #[schema(example = "AJ", required = false)]
  pub display_name: Option<String>,
}
