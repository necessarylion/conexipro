use actix_cors::Cors;
use actix_web::http;

pub fn get_cors_config() -> Cors {
  return Cors::default()
    .allowed_origin("*")
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS"])
    .allowed_headers(vec![http::header::AUTHORIZATION]);
}
