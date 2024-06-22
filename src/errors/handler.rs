use super::IValidationError;
use actix_http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde_json::json;
use std::{borrow::Cow, collections::HashMap};
use validator::{ValidationError, ValidationErrors};

#[derive(Debug, Display)]
pub enum IError {
  ValidationError(ValidationErrors),
  CustomValidatoinError(IValidationError),
  ServerError(String),
  NotFoundError(String),
  Unauthorized(String),
}

// Implement ResponseError trait, so that it is able to use as response
// in services
impl ResponseError for IError {
  fn error_response(&self) -> HttpResponse {
    match &self {
      IError::ValidationError(e) => handle_validation_error(&e),
      IError::ServerError(msg) => handle_server_error(&msg),
      IError::NotFoundError(msg) => handle_not_found_error(&msg),
      IError::Unauthorized(msg) => handle_unauthorized_error(&msg),
      IError::CustomValidatoinError(err) => handle_custom_validation_error(err),
    }
  }
}

// Handle validation errors
// and return as json
fn handle_validation_error(e: &ValidationErrors) -> HttpResponse {
  HttpResponse::BadRequest()
    .status(StatusCode::UNPROCESSABLE_ENTITY)
    .json(e)
}

// Handle server errors
fn handle_server_error(msg: &String) -> HttpResponse {
  HttpResponse::BadRequest()
    .status(StatusCode::INTERNAL_SERVER_ERROR)
    .json(json!({ "message": msg }))
}

// Handle auauthorized error
fn handle_unauthorized_error(msg: &String) -> HttpResponse {
  HttpResponse::BadRequest()
    .status(StatusCode::UNAUTHORIZED)
    .json(json!({ "message": msg }))
}

// handle 404 not found error
fn handle_not_found_error(msg: &String) -> HttpResponse {
  HttpResponse::BadRequest()
    .status(StatusCode::NOT_FOUND)
    .json(json!({ "message": msg }))
}

// handle custom validation error
fn handle_custom_validation_error(input: &IValidationError) -> HttpResponse {
  let mut v_err = ValidationErrors::new();
  let err = ValidationError {
    code: Cow::from(input.code.to_string()),
    message: Some(Cow::from(input.msg.to_string())),
    params: HashMap::new(),
  };
  v_err.add(input.field, err);
  HttpResponse::BadRequest()
    .status(StatusCode::UNPROCESSABLE_ENTITY)
    .json(v_err)
}
