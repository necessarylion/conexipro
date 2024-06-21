use crate::{
  controllers::{auth_controller, general_controller, user_controller},
  middleware::auth_middleware,
};
use actix_web::{
  web::{get, post, scope},
  Scope,
};
use actix_web_lab::middleware::from_fn;

pub fn get_api_services() -> Scope {
  scope("")
    // login or register do not required middleware to check token
    .route(
      "/api/auth/login",
      post().to(auth_controller::login_or_register),
    )
    .route(
      "/files/{filename:.*}",
      get().to(general_controller::render_file),
    )
    .service(
      scope("api")
        .wrap(from_fn(auth_middleware::handler))
        .service(auth_controller::refresh)
        .service(auth_controller::fetch)
        .service(user_controller::update)
        .service(user_controller::change_username)
        .service(user_controller::change_avatar),
    )
}
