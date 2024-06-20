use actix_web::{
  body::MessageBody,
  dev::{ServiceRequest, ServiceResponse},
  Error,
};
use actix_web_lab::middleware::Next;

use crate::utils::jwt;

pub async fn handler(
  mut req: ServiceRequest,
  next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
  // pre-processing
  let headers = req.headers_mut();
  let token = headers.get("authorization");
  if token == None {
    return Err(actix_web::error::ErrorUnauthorized("required bearer token"));
  }
  let token = token.unwrap().to_str().unwrap().replace("Bearer ", "");
  let claims = jwt::verify(token);
  if claims.is_err() {
    Err(actix_web::error::ErrorUnauthorized(claims.err().unwrap()))
  } else {
    let claims = claims.unwrap();
    headers.insert("x-auth-id".parse().unwrap(), claims.sub.parse().unwrap());
    next.call(req).await
  }
}
