use crate::models::User;
use crate::requests::{ChangeAvatarRequest, ChangeUsernameRequest, UserUpdateRequest};
use crate::response::{ChangeAvatarResponse, ChangeUsernameResponse, UserLoginResponse};
use crate::utils::jwt::JwtToken;
use crate::{controllers::{auth_controller, user_controller}, requests::UserLoginRequest};
use utoipa::{
  openapi::{
    self,
    security::{Http, HttpAuthScheme, SecurityScheme},
  },
  Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
  paths(
    auth_controller::fetch, 
    auth_controller::login_or_register,
    auth_controller::refresh,
    user_controller::update,
    user_controller::change_username,
    user_controller::change_avatar,
  ), 
  components(schemas(
    User, 
    UserLoginRequest,
    JwtToken,
    UserLoginResponse,
    UserUpdateRequest,
    ChangeUsernameRequest,
    ChangeUsernameResponse,
    ChangeAvatarRequest,
    ChangeAvatarResponse,
  )), 
  modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut openapi::OpenApi) {
    // NOTE: we can unwrap safely since there already is components registered.
    let components = openapi.components.as_mut().unwrap();
    components.add_security_scheme(
      "bearer_token",
      SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
    );
  }
}
