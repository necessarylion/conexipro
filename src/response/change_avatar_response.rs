use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ChangeAvatarResponse {
  pub avatar: String,
}
