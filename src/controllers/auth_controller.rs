use crate::{
  errors::handler::IError,
  models::user::{create_user, NewUser},
  requests::user_register_request::UserRegisterRequest,
  utils::{app::AppState, firebase::FireAuth},
};
use actix_web::{
  delete, get, post,
  web::{Data, Json},
  HttpResponse, Responder,
};
use serde_json::json;
use validator::Validate;

/**
 * user registration
 * require firebase token to login
 */
#[post("/auth/login")]
pub async fn login_or_register(
  info: Json<UserRegisterRequest>,
  state: Data<AppState>,
) -> Result<impl Responder, IError> {
  info.validate().map_err(IError::ValidationError)?;

  // verify firebase id token and get user info
  let id_token = info.token.as_ref();
  let auth = FireAuth::new();
  let user = auth.get_user_info(id_token).await?;

  let u_id = user.local_id.as_ref().unwrap();
  let display_name = user.display_name.as_ref().unwrap();
  let email = user.display_name.as_ref();
  let email = if email == None { "" } else { email.unwrap() };

  let new_user = NewUser {
    uid: u_id,
    username: u_id,
    first_name: display_name,
    display_name: display_name,
    email: Some(email),
  };

  let conn = &mut state.db_pool.get().unwrap();
  let user = create_user(conn, new_user)?;

  Ok(Json(json!({
    "success": true,
    "data": user,
  })))
}

/**
 * fetch user information
 */
#[get("/auth/user")]
pub async fn fetch() -> impl Responder {
  HttpResponse::Ok().body("return user info")
}

/**
 * refresh user new token
 */
#[get("/auth/refresh")]
pub async fn refresh() -> impl Responder {
  HttpResponse::Ok().body("return new token")
}

/**
 * logout user
 */
#[delete("/auth/logout")]
pub async fn logout() -> impl Responder {
  HttpResponse::Ok().body("logout user")
}
