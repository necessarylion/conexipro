use crate::{
  errors::handler::IError,
  models::user::{self, UpdateUser},
  requests::{
    extra_requests::ExtraRequests,
    user_update_request::{ChangeUsernameRequest, UserUpdateRequest},
  },
  utils::{db, to_str},
};
use actix_web::{post, put, web::Json, HttpRequest, Responder};
use serde_json::json;
use validator::Validate;

/// update user information
#[post("/auth/user")]
pub async fn update(
  req: HttpRequest,
  payload: Json<UserUpdateRequest>,
) -> Result<impl Responder, IError> {
  payload.validate().map_err(IError::ValidationError)?;
  let auth = req.auth()?;

  // get db connection
  let conn: &mut db::DbConn = &mut req.db_conn()?;

  let data = UpdateUser {
    first_name: payload.first_name.as_ref().unwrap(),
    last_name: to_str(payload.last_name.as_ref()),
    middle_name: to_str(payload.middle_name.as_ref()),
    display_name: to_str(payload.display_name.as_ref()),
  };

  let user = user::update_user_by_uid(conn, auth.uid(), data)?;

  Ok(Json(user))
}

/// change username
#[put("/auth/user/username")]
pub async fn change_username(
  req: HttpRequest,
  payload: Json<ChangeUsernameRequest>,
) -> Result<impl Responder, IError> {
  let auth = req.auth()?;

  // get db connection
  let conn: &mut db::DbConn = &mut req.db_conn()?;
  let username = payload.username.as_ref().unwrap();
  let user = user::get_user_by_username(conn, username);

  // if record do not found, the function will return error.
  // which mean username is not taken and able to change.
  if user.is_err() {
    // @todo call api to update username
    user::update_username_by_uid(conn, auth.uid(), username)?;
    return Ok(Json(json!({
      "success": true
    })));
  }

  let user = user?;

  // if found user and auth user is the same,
  // no need to do since it is the same username
  if user.id == auth.user.id {
    return Ok(Json(json!({
      "success": true
    })));
  }

  // if record found and not match with auth user,
  // it mean username is already taken.
  return Ok(Json(json!({
    "success": false,
    "message": "username is already taken"
  })));
}

/// change user avatar image
#[put("/auth/user/avatar")]
pub async fn change_avatar(req: HttpRequest) -> Result<impl Responder, IError> {
  let auth = req.auth()?;
  Ok(Json(auth.user))
}
