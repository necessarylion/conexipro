use actix_web::{get, web::Json, HttpResponse, Responder};
use serde_json::json;

use crate::{errors::handler::IError, requests::info_request::InfoRequest};
use validator::Validate;

#[get("/")]
pub async fn home(info: Json<InfoRequest>) -> Result<impl Responder, IError> {
  info.validate().map_err(IError::ValidationError)?;

  let obj = json!({
    "name": info.name,
    "email": info.email
  });

  Ok(Json(obj))
}

#[get("/api/post")]
pub async fn api() -> impl Responder {
  HttpResponse::Ok().body("conexipro api v1")
}
