use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use crate::{errors::error_handler::MyError, requests::info_request::InfoRequest};
use validator::Validate;

#[get("/")]
pub async fn home(info: web::Json<InfoRequest>) -> Result<impl Responder, MyError> {
  info.validate().map_err(MyError::ValidationError)?;

  let obj = json!({
    "name": info.name,
    "email": info.email
  });

  Ok(web::Json(obj))
}

#[get("/api/post")]
pub async fn api() -> impl Responder {
  HttpResponse::Ok().body("conexipro api v1")
}
