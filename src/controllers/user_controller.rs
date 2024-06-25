use crate::{
  requests::{ChangeAvatarRequest, ChangeUsernameRequest, ExtraRequests, UserUpdateRequest},
  services::UserService,
  utils::db::get_db_conn,
  DbConn, DbPool, IError,
};
use actix_multipart::form::MultipartForm;
use actix_web::{
  post, put,
  web::{self, Json},
  HttpRequest, Responder,
};
use validator::Validate;

/// update user information
#[post("/auth/user")]
pub async fn update(
  req: HttpRequest,
  pool: web::Data<DbPool>,
  payload: Json<UserUpdateRequest>,
) -> Result<impl Responder, IError> {
  // validate request
  payload.validate().map_err(IError::ValidationError)?;
  // get auth
  let auth = req.auth()?;

  let res = web::block(move || {
    let conn: DbConn = get_db_conn(&pool)?;
    let mut user_service = UserService { conn, auth };
    user_service.update_user_data(payload.into_inner())
  })
  .await??;

  Ok(Json(res))
}

/// change username
#[put("/auth/user/username")]
pub async fn change_username(
  req: HttpRequest,
  pool: web::Data<DbPool>,
  payload: Json<ChangeUsernameRequest>,
) -> Result<impl Responder, IError> {
  // validate request
  payload.validate().map_err(IError::ValidationError)?;
  // get auth
  let auth = req.auth()?;

  let res = web::block(move || {
    let conn: DbConn = get_db_conn(&pool)?;
    let mut user_service = UserService { conn, auth };
    user_service.change_user_name(payload.into_inner())
  })
  .await??;

  return Ok(Json(res));
}

/// change user avatar image
#[put("/auth/user/avatar")]
pub async fn change_avatar(
  req: HttpRequest,
  pool: web::Data<DbPool>,
  MultipartForm(form): MultipartForm<ChangeAvatarRequest>,
) -> Result<impl Responder, IError> {
  let auth = req.auth()?;
  let conn: DbConn = get_db_conn(&pool)?;
  let avatar = form.avatar;
  let mut user_service = UserService { conn, auth };
  let res = user_service.change_avatar(&avatar).await?;
  Ok(Json(res))
}
