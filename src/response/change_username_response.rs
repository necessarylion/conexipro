use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ChangeUsernameResponse {
  #[schema(example = "true")]
  pub success: bool,
  #[schema(example = "username updated successfully")]
  pub message: String,
}
