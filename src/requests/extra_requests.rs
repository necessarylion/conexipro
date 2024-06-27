use std::sync::Arc;

use crate::{Auth, IError};
use actix_web::HttpRequest;

pub trait ExtraRequests {
  fn auth(&self) -> Result<Arc<Auth>, IError>;
}

impl ExtraRequests for HttpRequest {
  /// Get auth user from request
  /// ```
  /// let auth = req.auth()?;
  /// ```
  fn auth(&self) -> Result<Arc<Auth>, IError> {
    let headers = self.headers();
    let auth_id = headers.get("x-auth-id");
    if auth_id.is_none() {
      return Err(IError::Unauthorized(String::from(
        "x-auth-id not found in the header",
      )));
    }
    let id: u32 = auth_id.unwrap().to_str().unwrap().parse::<u32>().unwrap();
    Ok(Arc::new(Auth { user_id: id }))
  }
}
