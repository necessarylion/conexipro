use crate::{
  controllers::{auth_controller, general_controller, user_controller},
  middleware::auth_middleware,
};
use actix_web::{
  dev::{ServiceFactory, ServiceRequest, ServiceResponse},
  middleware::{self, TrailingSlash},
};
use actix_web::{
  web::{get, post, scope},
  Scope,
};
use actix_web_lab::middleware::from_fn;

pub fn get_api_services() -> Scope<
  impl ServiceFactory<
    ServiceRequest,
    Config = (),
    Response = ServiceResponse,
    Error = actix_web::Error,
    InitError = (),
  >,
> {
  scope("")
    .wrap(middleware::NormalizePath::new(TrailingSlash::default()))
    // login or register do not required middleware to check token
    .route("/api/auth/login", post().to(auth_controller::login))
    .route(
      "/api/user/{username}",
      get().to(user_controller::get_user_detail),
    )
    .route(
      "/files/{filename:.*}",
      get().to(general_controller::render_file),
    )
    .service(
      scope("api")
        .wrap(from_fn(auth_middleware::handler))
        .service(auth_controller::refresh_user_token)
        .service(auth_controller::fetch_user)
        .service(user_controller::update_user)
        .service(user_controller::change_username)
        .service(user_controller::change_avatar)
        .service(user_controller::update_user_infos),
    )
}
