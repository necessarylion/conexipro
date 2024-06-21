use crate::{utils::get_env, IError};
use aws_sdk_s3::{primitives::AggregatedBytes, Client};
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

  /// get aws client
  async fn get_client(&self) -> Client {
    let config = aws_config::load_from_env().await;
    aws_sdk_s3::Client::new(&config)
  }

  /// put file to aws
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

  /// get file from aws
  pub async fn get(&self, key: &String) -> Result<AggregatedBytes, IError> {
    let client = self.get_client().await;
    client
      .get_object()
      .bucket(&self.bucket)
      .key(key)
      .send()
      .await
      .map_err(|err| IError::ServerError(err.to_string()))?
      .body
      .collect()
      .await
      .map_err(|err| IError::ServerError(err.to_string()))
  }

  /// delete file from aws
  pub async fn delete(&self, key: &String) -> Result<(), IError> {
    let client = self.get_client().await;

    // check if file exists
    let val = client
      .head_object()
      .bucket(&self.bucket)
      .key(key)
      .send()
      .await
      .map_err(|err| IError::ServerError(err.to_string()));

    // if not exists, return
    if val.is_err() {
      return Ok(());
    }

    client
      .delete_object()
      .bucket(&self.bucket)
      .key(key)
      .send()
      .await
      .map_err(|err| IError::ServerError(err.to_string()))?;
    Ok(())
  }
}
