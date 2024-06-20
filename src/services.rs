use crate::controllers::auth_controller;
use actix_web::{web, Scope};

pub fn get_api_services() -> Scope {
  web::scope("api")
    // authentication services
    .service(auth_controller::login_or_register)
    .service(auth_controller::refresh)
    .service(auth_controller::fetch)
    .service(auth_controller::logout)
}
