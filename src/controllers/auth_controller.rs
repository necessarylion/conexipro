use crate::{
  errors::handler::IError,
  models::user::NewUserPayload,
  repository::user_repo::UserRepo,
  requests::{extra_requests::ExtraRequests, user_register_request::UserRegisterRequest},
  utils::{db, firebase::FireAuth, jwt, to_str},
};
use actix_web::{get, web::Json, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use validator::Validate;

/// user registration.
/// require firebase token to login
pub async fn login_or_register(
  req: HttpRequest,
  info: Json<UserRegisterRequest>,
) -> Result<impl Responder, IError> {
  info.validate().map_err(IError::ValidationError)?;

  // verify firebase id token and get user info
  let id_token = info.token.as_ref();
  let auth = FireAuth::new();
  let firebase_user = auth.get_user_info(id_token).await?;
  let uid = firebase_user.local_id.as_ref().unwrap();

  // get db connection
  let conn: &mut db::DbConn = &mut req.db_conn()?;
  // check if user exists with uid
  let mut user = UserRepo::get_user_by_uid(conn, uid);

  // if not exist create new one
  if user.is_err() {
    let display_name = firebase_user.display_name.as_ref().unwrap();
    let email = firebase_user.email.as_ref();
    let email = to_str(email);
    // prepare new user payload
    let new_user = NewUserPayload {
      uid: uid,
      username: uid,
      first_name: display_name,
      display_name: display_name,
      email: email,
    };
    user = UserRepo::create_user(conn, new_user);
  }

  let token = jwt::create(uid)?;

  let response = json!({
    "token": token,
    "success": true,
    "user": user?,
  });

  Ok(
    HttpResponse::Ok()
      .insert_header(("Authorization", token.token))
      .json(response),
  )
}

/// fetch user information
#[get("/auth/user")]
pub async fn fetch(req: HttpRequest) -> Result<impl Responder, IError> {
  let auth = req.auth()?;
  Ok(Json(auth.user))
}

/// refresh user new token
#[get("/auth/refresh")]
pub async fn refresh(req: HttpRequest) -> Result<impl Responder, IError> {
  let auth = req.auth()?;

  let token = jwt::create(auth.uid())?;

  let response = json!({
    "token": token,
    "success": true,
    "user": auth.user,
  });

  Ok(
    HttpResponse::Ok()
      .insert_header(("Authorization", token.token))
      .json(response),
  )
}
