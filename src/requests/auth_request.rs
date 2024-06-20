use actix_web::{web::Data, HttpRequest};

use crate::{
  errors::handler::IError,
  models::user::{self, User},
  utils::{app::AppState, db::get_db_conn},
};

pub struct Auth {
  pub user: User,
}

impl Auth {
  pub fn get_uid(&self) -> &String {
    &self.user.uid
  }
}

// Define a trait with the custom function
pub trait AuthRequest {
  fn auth(&self) -> Result<Auth, IError>;
}

// Implement the trait for ServiceRequest
impl AuthRequest for HttpRequest {
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
      let user = user::get_user_by_uid(conn, &uid)?;
      return Ok(Auth { user });
    }
    return Err(IError::ServerError(String::from("failed to get app state")));
  }
}
