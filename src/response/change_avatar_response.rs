use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ChangeAvatarResponse {
  #[schema(
    example = "http://127.0.0.1:3335/files/conexipro-dev/72e1f1ea-7958-484b-97c2-b91d842e60c8.png"
  )]
  pub avatar: String,
}
