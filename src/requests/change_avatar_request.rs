use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use utoipa::ToSchema;

#[derive(Debug, MultipartForm, ToSchema)]
pub struct ChangeAvatarRequest {
  #[schema(value_type = String, format = Binary)]
  #[multipart(limit = "2 MiB")]
  pub avatar: TempFile,
}
