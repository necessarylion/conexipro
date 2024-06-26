use crate::{models::User, repository::UserRepo, DbPool, IError};
use std::sync::Arc;
pub struct Auth {
  pub user_uid: String,
}

impl Auth {
  pub fn uid(&self) -> &String {
    &self.user_uid
  }

  pub async fn user(&self, pool: &Arc<DbPool>) -> Result<User, IError> {
    let conn = &mut pool
      .get()
      .await
      .map_err(|err| IError::ServerError(err.to_string()))?;
    UserRepo::get_user_by_uid(conn, &self.uid()).await
  }
}
