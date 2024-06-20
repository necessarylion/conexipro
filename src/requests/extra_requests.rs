use actix_web::{web::Data, HttpRequest};

use crate::{
  errors::handler::IError,
  models::user::User,
  repository::user_repo::UserRepo,
  utils::{
    app::AppState,
    db::{get_db_conn, DbConn},
  },
};

pub struct Auth {
  pub user: User,
}

impl Auth {
  pub fn uid(&self) -> &String {
    &self.user.uid
  }
}

pub trait ExtraRequests {
  fn auth(&self) -> Result<Auth, IError>;
  fn db_conn(&self) -> Result<DbConn, IError>;
}

impl ExtraRequests for HttpRequest {
  /// Get auth user from request
  /// ```
  /// let auth = req.auth()?;
  /// ```
  fn auth(&self) -> Result<Auth, IError> {
    let headers = self.headers();
    let auth_id = headers.get("x-auth-id");
    if auth_id == None {
      return Err(IError::Unauthorized(String::from(
        "x-auth-id not found in the header",
      )));
    }

    if let Some(app_state) = self.app_data::<Data<AppState>>() {
      let pool = &app_state.db_pool;
      let conn = &mut get_db_conn(pool)?;
      let uid = auth_id.unwrap().to_str().unwrap().to_string();
      let user = UserRepo::get_user_by_uid(conn, &uid)?;
      return Ok(Auth { user });
    }
    return Err(IError::ServerError(String::from("failed to get app state")));
  }

  /// get database connection from request
  /// ```
  /// let conn: &mut db::DbConn = &mut req.db_conn()?;
  /// ```
  fn db_conn(&self) -> Result<DbConn, IError> {
    if let Some(app_state) = self.app_data::<Data<AppState>>() {
      let pool = &app_state.db_pool;
      let conn: DbConn = get_db_conn(pool)?;
      return Ok(conn);
    }
    return Err(IError::ServerError(String::from("failed to get app state")));
  }
}
