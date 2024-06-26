use std::sync::Arc;

use crate::{models::User, repository::UserRepo, DbPool, IError};

pub struct Auth {
  pub user_uid: String,
}

impl Auth {
  pub fn uid(&self) -> &String {
    &self.user_uid
  }

  pub async fn user(&self, pool: &Arc<DbPool>) -> Result<User, IError> {
    let pool = pool.clone();
    let conn = &mut pool.get().await.unwrap();
    UserRepo::get_user_by_uid(conn, &self.uid()).await
  }
}
