use actix_multipart::form::{tempfile::TempFile, MultipartForm};

#[derive(Debug, MultipartForm)]
pub struct ChangeAvatarRequest {
  pub avatar: TempFile,
}
