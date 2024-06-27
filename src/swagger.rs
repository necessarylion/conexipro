use crate::models::{User, UserInfo};
use crate::requests::user_info_update_request::Info;
use crate::requests::{ChangeAvatarRequest, ChangeUsernameRequest, UserInfoUpdateRequest, UserLoginRequest, UserUpdateRequest};
use crate::response::{ChangeAvatarResponse, ChangeUsernameResponse, UserLoginResponse, UserDetailResponse};
use crate::controllers::{auth_controller, user_controller};
use crate::utils::jwt::JwtToken;
use utoipa::{
  openapi::{
    self,
    security::{Http, HttpAuthScheme, SecurityScheme},
  },
  Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
  servers(
    (url = "{BaseUrl}",
      variables(
        ("BaseUrl" = (default = "http://127.0.0.1:3335/api")),
      )
    )
  ),
  paths(
    auth_controller::login,
    auth_controller::fetch_user, 
    auth_controller::refresh_user_token,
    user_controller::update_user,
    user_controller::change_username,
    user_controller::change_avatar,
    user_controller::update_user_infos
  ),
  components(schemas(
    User, 
    Info,
    UserInfo,
    UserLoginRequest,
    JwtToken,
    UserLoginResponse,
    UserDetailResponse,
    UserInfoUpdateRequest,
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

/// crate swagger json file
pub fn crate_swagger_file() -> () {
  let val = ApiDoc::openapi().to_pretty_json().unwrap();
  std::fs::write("./fe/src/swaggers/api.swagger.json", val).unwrap();
}