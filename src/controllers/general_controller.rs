use actix_web::{web::Path, HttpResponse, Responder};

use crate::{services::StorageService, IError};

pub async fn render_file(filename: Path<String>) -> Result<impl Responder, IError> {
  let storage = StorageService::new();
  let file_exist = storage.exists(&filename).await?;
  if !file_exist {
    return Err(IError::NotFoundError("file not found".to_string()));
  }
  let image_bytes = storage.get(&filename).await?;
  let image_bytes = image_bytes.to_vec();
  let content_type = mime_guess::from_path(filename.to_string()).first();
  if content_type.is_none() {
    return Err(IError::NotFoundError("un supported file".to_string()));
  }
  let content_type = content_type.unwrap().to_string();
  Ok(
    HttpResponse::Ok()
      .append_header(("Content-Type", content_type))
      .append_header(("Cache-Control", "public, max-age=3600, immutable"))
      .body(image_bytes),
  )
}
