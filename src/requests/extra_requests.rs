use actix_web::{web::Data, HttpRequest};

use crate::{repository::UserRepo, utils::db, Auth, DbConn, DbPool, IError};

pub trait ExtraRequests {
  fn auth(&self) -> Result<Auth, IError>;
}

impl ExtraRequests for HttpRequest {
  /// Get auth user from request
  /// ```
  /// let auth = req.auth()?;
  /// ```
  fn auth(&self) -> Result<Auth, IError> {
    let headers = self.headers();
    let auth_id = headers.get("x-auth-id");
    if auth_id.is_none() {
      return Err(IError::Unauthorized(String::from(
        "x-auth-id not found in the header",
      )));
    }

    if let Some(pool) = self.app_data::<Data<DbPool>>() {
      let conn: &mut DbConn = &mut db::get_db_conn(&pool)?;
      let uid = auth_id.unwrap().to_str().unwrap().to_string();
      let user = UserRepo::get_user_by_uid(conn, &uid)?;
      return Ok(Auth { user });
    }
    return Err(IError::ServerError(String::from("failed to get app state")));
  }
}
