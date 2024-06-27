use crate::{models::User, repository::UserRepo, DbPool, IError};
use std::sync::Arc;
pub struct Auth {
  pub user_id: u32,
}

impl Auth {
  pub fn id(&self) -> &u32 {
    &self.user_id
  }

  pub async fn user(&self, pool: &Arc<DbPool>) -> Result<User, IError> {
    let conn = &mut pool
      .get()
      .await
      .map_err(|err| IError::ServerError(err.to_string()))?;
    UserRepo::get_user_by_id(conn, &self.id()).await
  }
}
