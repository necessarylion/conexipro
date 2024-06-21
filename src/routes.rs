use crate::{
  controllers::{auth_controller, user_controller},
  middleware::auth_middleware,
};
use actix_web::{
  web::{post, scope},
  Scope,
};
use actix_web_lab::middleware::from_fn;

pub fn get_api_services() -> Scope {
  scope("api")
    // login or register do not required middleware to check token
    .route("/auth/login", post().to(auth_controller::login_or_register))
    .service(
      scope("")
        .wrap(from_fn(auth_middleware::handler))
        .service(auth_controller::refresh)
        .service(auth_controller::fetch)
        .service(user_controller::update)
        .service(user_controller::change_username)
        .service(user_controller::change_avatar),
    )
}
