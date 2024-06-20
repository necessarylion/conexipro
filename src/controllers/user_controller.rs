use crate::{
  requests::{ChangeUsernameRequest, ExtraRequests, UserUpdateRequest},
  services::UserService,
  IError,
};
use actix_web::{post, put, web::Json, HttpRequest, Responder};
use validator::Validate;

/// update user information
#[post("/auth/user")]
pub async fn update(
  req: HttpRequest,
  payload: Json<UserUpdateRequest>,
) -> Result<impl Responder, IError> {
  // validate request
  payload.validate().map_err(IError::ValidationError)?;
  // get auth
  let auth = req.auth()?;
  // get db connection
  let conn = req.db_conn()?;

  let mut user_service = UserService { conn, auth };
  let res = user_service.update_user_data(payload.into_inner())?;

  Ok(Json(res))
}

/// change username
#[put("/auth/user/username")]
pub async fn change_username(
  req: HttpRequest,
  payload: Json<ChangeUsernameRequest>,
) -> Result<impl Responder, IError> {
  // validate request
  payload.validate().map_err(IError::ValidationError)?;
  // get auth
  let auth = req.auth()?;
  // get db connection
  let conn = req.db_conn()?;

  let mut user_service = UserService { conn, auth };
  let res = user_service.change_user_name(payload.into_inner())?;

  return Ok(Json(res));
}

/// change user avatar image
#[put("/auth/user/avatar")]
pub async fn change_avatar(req: HttpRequest) -> Result<impl Responder, IError> {
  let auth = req.auth()?;
  Ok(Json(auth.user))
}
