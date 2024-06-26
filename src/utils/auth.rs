use std::sync::Arc;

use super::db::get_db_conn;
use crate::{models::User, repository::UserRepo, DbConn, DbPool, IError};
use actix_web::web;

pub struct Auth {
  pub user_uid: String,
}

impl Auth {
  pub fn uid(&self) -> &String {
    &self.user_uid
  }

  pub async fn user(&self, pool: &Arc<DbPool>) -> Result<User, IError> {
    web::block({
      let uid = self.uid().clone();
      let pool = pool.clone();
      move || {
        let conn: &mut DbConn = &mut get_db_conn(&pool)?;
        UserRepo::get_user_by_uid(conn, &uid)
      }
    })
    .await?
  }
}
