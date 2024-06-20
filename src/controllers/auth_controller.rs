use crate::{
  errors::handler::IError,
  models::user,
  requests::user_register_request::UserRegisterRequest,
  utils::{app::AppState, db, firebase::FireAuth, jwt},
};
use actix_web::{
  delete, get,
  web::{Data, Json},
  HttpResponse, Responder,
};
use serde_json::json;
use validator::Validate;

/**
 * user registration
 * require firebase token to login
 */
pub async fn login_or_register(
  info: Json<UserRegisterRequest>,
  state: Data<AppState>,
) -> Result<impl Responder, IError> {
  info.validate().map_err(IError::ValidationError)?;

  // verify firebase id token and get user info
  let id_token = info.token.as_ref();
  let auth = FireAuth::new();
  let firebase_user = auth.get_user_info(id_token).await?;
  let uid = firebase_user.local_id.as_ref().unwrap();

  // get db connection
  let conn: &mut db::DbConn = &mut db::get_db_conn(&state.db_pool)?;
  // check if user exists with uid
  let mut user = user::get_user_by_uid(conn, uid);

  // if not exist create new one
  if user.is_err() {
    let display_name = firebase_user.display_name.as_ref().unwrap();
    let email = firebase_user.email.as_ref();
    let email = if email == None { "" } else { email.unwrap() };
    // prepare new user payload
    let new_user = user::NewUser {
      uid: uid,
      username: uid,
      first_name: display_name,
      display_name: display_name,
      email: Some(email),
    };
    user = user::create_user(conn, new_user);
  }

  let token = jwt::create_token(uid)?;

  let response = json!({
    "token": token,
    "success": true,
    "user": user?,
  });

  Ok(
    HttpResponse::Ok()
      .insert_header(("Authorization", token))
      .json(response),
  )
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
