use crate::{errors::handler::IError, schema::users};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
  pub id: u32,
  pub uid: String,
  pub username: String,
  pub email: Option<String>,
  pub first_name: String,
  pub middle_name: Option<String>,
  pub last_name: Option<String>,
  pub display_name: Option<String>,
  pub avatar: Option<String>,
  pub created_at: Option<chrono::NaiveDateTime>,
  pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
  pub uid: &'a str,
  pub username: &'a str,
  pub first_name: &'a str,
  pub display_name: &'a str,
  pub email: Option<&'a str>,
}

pub fn create_user(conn: &mut MysqlConnection, new_user: NewUser) -> Result<User, IError> {
  conn
    .transaction(|conn| {
      diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

      users::table
        .order(users::id.desc())
        .select(User::as_select())
        .first(conn)
    })
    .map_err(|err| IError::ServerError(err.to_string()))
}

pub fn get_user_by_uid(conn: &mut MysqlConnection, uid: &String) -> Result<User, IError> {
  users::table
    .filter(users::uid.eq(uid))
    .select(User::as_select())
    .first(conn)
    .map_err(|err| IError::ServerError(err.to_string()))
}
