use actix_multipart::form::{tempfile::TempFile, MultipartForm};

#[derive(Debug, MultipartForm)]
pub struct ChangeAvatarRequest {
  #[multipart(limit = "2MB")]
  pub avatar: TempFile,
}
