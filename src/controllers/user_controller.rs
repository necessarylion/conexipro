use crate::{
  errors::handler::IError,
  models::user::{self, UpdateUser},
  requests::{auth_request::AuthRequest, user_update_request::UserUpdateRequest},
  utils::{app::AppState, db, to_str},
};
use actix_web::{
  post,
  web::{Data, Json},
  HttpRequest, Responder,
};
use validator::Validate;

/**
 * update user information
 */
#[post("/auth/user")]
pub async fn update(
  req: HttpRequest,
  payload: Json<UserUpdateRequest>,
  state: Data<AppState>,
) -> Result<impl Responder, IError> {
  payload.validate().map_err(IError::ValidationError)?;
  let auth = req.auth()?;

  // get db connection
  let conn: &mut db::DbConn = &mut db::get_db_conn(&state.db_pool)?;

  let data = UpdateUser {
    first_name: payload.first_name.as_ref().unwrap(),
    last_name: to_str(payload.last_name.as_ref()),
    middle_name: to_str(payload.middle_name.as_ref()),
    display_name: to_str(payload.display_name.as_ref()),
  };

  let user = user::update_user_by_uid(conn, auth.get_uid(), data)?;

  Ok(Json(user))
}
