use crate::{controllers::auth_controller, middleware::auth_middleware};
use actix_web::{web, Scope};
use actix_web_lab::middleware::from_fn;

pub fn get_api_services() -> Scope {
  web::scope("api")
    // login or register do not required middleware to check token
    .route(
      "/auth/login",
      web::post().to(auth_controller::login_or_register),
    )
    .service(vec![
      // auth routes which required token to check
      web::scope("")
        .wrap(from_fn(auth_middleware::handler))
        .service(auth_controller::refresh)
        .service(auth_controller::fetch)
        .service(auth_controller::logout),
    ])
}
