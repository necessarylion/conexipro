use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use utoipa::ToSchema;

#[derive(Debug, MultipartForm, ToSchema)]
pub struct ChangeAvatarRequest {
  pub avatar: TempFile,
}
