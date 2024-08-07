use crate::{
  models::{NewUserPayload, UpdateUserPayload, User},
  schema::users::{self, dsl::users as users_dsl},
  IError,
};
use diesel::prelude::*;
use diesel_async::AsyncMysqlConnection;
use diesel_async::RunQueryDsl;

pub struct UserRepo {}

impl UserRepo {
  /// create a new user
  pub async fn create_user<'a>(
    conn: &mut AsyncMysqlConnection,
    new_user: NewUserPayload<'a>,
  ) -> Result<User, IError> {
    diesel::insert_into(users::table)
      .values(&new_user)
      .execute(conn)
      .await
      .map_err(|err| IError::ServerError(err.to_string()))?;
    UserRepo::get_user_by_uid(conn, &new_user.uid.to_string()).await
  }

  /// get user by id
  pub async fn get_user_by_id(conn: &mut AsyncMysqlConnection, id: &u32) -> Result<User, IError> {
    users::table
      .filter(users::id.eq(id))
      .select(User::as_select())
      .first(conn)
      .await
      .map_err(|err| IError::ServerError(err.to_string()))
  }

  /// get user by uid
  pub async fn get_user_by_uid(
    conn: &mut AsyncMysqlConnection,
    uid: &String,
  ) -> Result<User, IError> {
    users::table
      .filter(users::uid.eq(uid))
      .select(User::as_select())
      .first(conn)
      .await
      .map_err(|err| IError::ServerError(err.to_string()))
  }

  /// get user by username
  pub async fn get_user_by_username(
    conn: &mut AsyncMysqlConnection,
    username: &String,
  ) -> Result<User, IError> {
    users::table
      .filter(users::username.eq(username))
      .select(User::as_select())
      .first(conn)
      .await
      .map_err(|err| IError::ServerError(err.to_string()))
  }

  /// update user data by id
  pub async fn update_user_by_id<'a>(
    conn: &mut AsyncMysqlConnection,
    id: &u32,
    payload: UpdateUserPayload<'a>,
  ) -> Result<User, IError> {
    diesel::update(users_dsl.filter(users::id.eq(id)))
      .set((
        users::first_name.eq(payload.first_name),
        users::last_name.eq(payload.last_name),
        users::middle_name.eq(payload.middle_name),
        users::display_name.eq(payload.display_name),
      ))
      .execute(conn)
      .await
      .map_err(|err| IError::ServerError(err.to_string()))?;

    users::table
      .filter(users::id.eq(id))
      .select(User::as_select())
      .first(conn)
      .await
      .map_err(|err: diesel::result::Error| IError::ServerError(err.to_string()))
  }

  /// update username by id
  pub async fn update_username_by_id(
    conn: &mut AsyncMysqlConnection,
    id: &u32,
    username: &String,
  ) -> Result<(), IError> {
    let res: Result<usize, diesel::result::Error> =
      diesel::update(users_dsl.filter(users::id.eq(id)))
        .set(users::username.eq(username))
        .execute(conn)
        .await;
    if res.is_err() {
      return Err(IError::ServerError(res.err().unwrap().to_string()));
    }
    Ok(())
  }

  /// update user avatar by id
  pub async fn update_avatar_by_id(
    conn: &mut AsyncMysqlConnection,
    id: &u32,
    avatar: &String,
  ) -> Result<(), IError> {
    let res: Result<usize, diesel::result::Error> =
      diesel::update(users_dsl.filter(users::id.eq(id)))
        .set(users::avatar.eq(avatar))
        .execute(conn)
        .await;
    if res.is_err() {
      return Err(IError::ServerError(res.err().unwrap().to_string()));
    }
    Ok(())
  }
}
