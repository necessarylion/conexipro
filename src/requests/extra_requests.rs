use actix_web::HttpRequest;

use crate::{Auth, IError};

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

    let uid = auth_id.unwrap().to_str().unwrap().to_string();
    return Ok(Auth { user_uid: uid });
  }
}
