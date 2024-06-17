use actix_web::web;

use crate::controllers::home_controller::{api, home};

pub fn get_api_services() -> actix_web::Scope {
  web::scope("").service(home).service(api)
}
