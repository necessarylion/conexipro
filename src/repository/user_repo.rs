use crate::{
  models::{NewUserPayload, UpdateUserPayload, User},
  schema::users::{self, dsl::users as users_dsl},
  IError,
};
use diesel::prelude::*;

pub struct UserRepo {}

impl UserRepo {
  /// create a new user
  pub fn create_user(conn: &mut MysqlConnection, new_user: NewUserPayload) -> Result<User, IError> {
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

  /// get user by uid
  pub fn get_user_by_uid(conn: &mut MysqlConnection, uid: &String) -> Result<User, IError> {
    users::table
      .filter(users::uid.eq(uid))
      .select(User::as_select())
      .first(conn)
      .map_err(|err| IError::ServerError(err.to_string()))
  }

  /// get user by username
  pub fn get_user_by_username(
    conn: &mut MysqlConnection,
    username: &String,
  ) -> Result<User, IError> {
    users::table
      .filter(users::username.eq(username))
      .select(User::as_select())
      .first(conn)
      .map_err(|err| IError::ServerError(err.to_string()))
  }

  /// update user data by uid
  pub fn update_user_by_uid(
    conn: &mut MysqlConnection,
    uid: &String,
    payload: UpdateUserPayload,
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

  /// update username
  pub fn update_username_by_uid(
    conn: &mut MysqlConnection,
    uid: &String,
    username: &String,
  ) -> Result<(), IError> {
    let res: Result<usize, diesel::result::Error> =
      diesel::update(users_dsl.filter(users::uid.eq(uid)))
        .set(users::username.eq(username))
        .execute(conn);
    if res.is_err() {
      return Err(IError::ServerError(res.err().unwrap().to_string()));
    }
    Ok(())
  }

  /// update user avatar
  pub fn update_avatar_by_uid(
    conn: &mut MysqlConnection,
    uid: &String,
    avatar: &String,
  ) -> Result<(), IError> {
    let res: Result<usize, diesel::result::Error> =
      diesel::update(users_dsl.filter(users::uid.eq(uid)))
        .set(users::avatar.eq(avatar))
        .execute(conn);
    if res.is_err() {
      return Err(IError::ServerError(res.err().unwrap().to_string()));
    }
    Ok(())
  }
}
