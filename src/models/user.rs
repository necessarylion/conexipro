use crate::{
  errors::handler::IError,
  schema::users::{self, dsl::users as users_dsl},
};
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

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
  pub first_name: &'a str,
  pub last_name: Option<&'a str>,
  pub middle_name: Option<&'a str>,
  pub display_name: Option<&'a str>,
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

pub fn get_user_by_username(conn: &mut MysqlConnection, username: &String) -> Result<User, IError> {
  users::table
    .filter(users::username.eq(username))
    .select(User::as_select())
    .first(conn)
    .map_err(|err| IError::ServerError(err.to_string()))
}

pub fn update_user_by_uid(
  conn: &mut MysqlConnection,
  uid: &String,
  payload: UpdateUser,
) -> Result<User, IError> {
  conn
    .transaction(|conn| {
      diesel::update(users_dsl.filter(users::uid.eq(uid)))
        .set((
          users::first_name.eq(payload.first_name),
          users::last_name.eq(payload.last_name),
          users::middle_name.eq(payload.middle_name),
          users::display_name.eq(payload.display_name),
        ))
        .execute(conn)?;

      users::table
        .filter(users::uid.eq(uid))
        .select(User::as_select())
        .first(conn)
    })
    .map_err(|err: diesel::result::Error| IError::ServerError(err.to_string()))
}

pub fn update_username_by_uid(
  conn: &mut MysqlConnection,
  uid: &String,
  username: &String,
) -> Result<(), IError> {
  conn
    .transaction(|conn| {
      diesel::update(users_dsl.filter(users::uid.eq(uid)))
        .set(users::username.eq(username))
        .execute(conn)?;
      Ok(())
    })
    .map_err(|err: diesel::result::Error| IError::ServerError(err.to_string()))
}
