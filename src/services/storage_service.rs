use crate::{utils::get_env, IError};
use aws_sdk_s3::Client;
use std::{fs::File, io::Read};
use uuid::Uuid;

pub struct StorageService {
  pub bucket: String,
  pub folder: String,
}

impl StorageService {
  pub fn new() -> Self {
    let bucket = get_env("AWS_BUCKET").unwrap();
    let folder = get_env("AWS_FOLDER").unwrap();
    StorageService { bucket, folder }
  }

  async fn get_client(&self) -> Client {
    let config = aws_config::load_from_env().await;
    aws_sdk_s3::Client::new(&config)
  }

  pub async fn put(&self, file: &mut &File, content_type: String) -> Result<String, IError> {
    let name = Uuid::new_v4().to_string();
    let ext = content_type.split("/").last().unwrap();
    let key = format!("{}/{}.{}", self.folder, name, ext);
    let client = self.get_client().await;

    let mut buffer = Vec::new();
    file
      .read_to_end(&mut buffer)
      .map_err(|err| IError::ServerError(err.to_string()))?;

    client
      .put_object()
      .bucket(&self.bucket)
      .key(&key)
      .body(buffer.into())
      .content_type(content_type)
      .send()
      .await
      .map_err(|err| IError::ServerError(err.to_string()))?;

    Ok(key)
  }
}
