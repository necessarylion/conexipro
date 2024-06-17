use actix_cors::Cors;
use actix_http::{header, Method};

struct CorsConfig {
  pub headers: Vec<header::HeaderName>,
  pub methods: Vec<Method>,
  pub origins: Vec<String>,
}

pub fn get_cors_config() -> Cors {
  // cors configuration
  let cors_config = CorsConfig {
    headers: vec![header::AUTHORIZATION],
    methods: vec![
      Method::GET,
      Method::POST,
      Method::PUT,
      Method::DELETE,
      Method::HEAD,
      Method::OPTIONS,
    ],
    origins: vec![String::from("*")],
  };

  return Cors::default()
    .allowed_origin_fn(move |origin, _| {
      return cors_config.origins.iter().any(|o| o == "*" || o == origin);
    })
    .allowed_methods(cors_config.methods)
    .allowed_headers(cors_config.headers);
}
