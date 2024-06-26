use crate::{
  models::NewUserPayload,
  repository::UserRepo,
  requests::{ExtraRequests, UserRegisterRequest},
  utils::{firebase::FireAuth, jwt, to_str},
  DbPool, IError,
};
use actix_web::{
  get,
  web::{Data, Json},
  HttpRequest, HttpResponse, Responder,
};
use serde_json::json;
use validator::Validate;

/// user registration.
/// require firebase token to login
pub async fn login_or_register(
  pool: Data<DbPool>,
  info: Json<UserRegisterRequest>,
) -> Result<impl Responder, IError> {
  info.validate().map_err(IError::ValidationError)?;

  // verify firebase id token and get user info
  let id_token = info.token.as_ref();
  let auth = FireAuth::new();
  let firebase_user = auth.get_user_info(id_token).await?;
  let uid = firebase_user.get_uid();

  // get db connec
  let uid = uid.clone();
  let conn = &mut pool.get().await.unwrap();

  // check if user exists with uid
  let mut user = UserRepo::get_user_by_uid(conn, &uid).await;

  // if not exist create new one
  if user.is_err() {
    let display_name = firebase_user.display_name.as_ref().unwrap();
    let email = firebase_user.email.as_ref();
    let email = to_str(email);
    // prepare new user payload
    let new_user = NewUserPayload {
      uid: &uid,
      username: &uid,
      first_name: display_name,
      display_name,
      email,
    };
    user = UserRepo::create_user(conn, new_user).await;
  }

  let token = jwt::create(&uid)?;

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
pub async fn fetch(req: HttpRequest, pool: Data<DbPool>) -> Result<impl Responder, IError> {
  let auth = req.auth()?;
  let user = auth.user(&pool).await?;
  Ok(Json(user))
}

/// refresh user new token
#[get("/auth/refresh")]
pub async fn refresh(req: HttpRequest, pool: Data<DbPool>) -> Result<impl Responder, IError> {
  let auth = req.auth()?;
  let token = jwt::create(auth.uid())?;
  let user = auth.user(&pool).await?;
  let response = json!({
    "token": token,
    "success": true,
    "user": user,
  });

  Ok(
    HttpResponse::Ok()
      .insert_header(("Authorization", token.token))
      .json(response),
  )
}
