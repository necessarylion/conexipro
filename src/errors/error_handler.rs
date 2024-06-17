use actix_http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug, Display)]
pub enum MyError {
  ValidationError(ValidationErrors),
  ServerError(String),
}

impl ResponseError for MyError {
  fn error_response(&self) -> HttpResponse {
    self.response()
  }
}

impl MyError {
  fn response(&self) -> HttpResponse {
    match self {
      MyError::ValidationError(e) => handle_validation_error(e),
      MyError::ServerError(msg) => handle_server_error(msg),
    }
  }
}

fn handle_validation_error(e: &ValidationErrors) -> HttpResponse {
  HttpResponse::BadRequest()
    .status(StatusCode::UNPROCESSABLE_ENTITY)
    .json(e)
}

fn handle_server_error(msg: &String) -> HttpResponse {
  HttpResponse::BadRequest()
    .status(StatusCode::INTERNAL_SERVER_ERROR)
    .json(json!({ "message": msg }))
}
