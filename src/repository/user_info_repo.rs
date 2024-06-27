use crate::models::user_info::NewUserInfoPayload;
use crate::schema::user_infos::{self, dsl::user_infos as info_dsl};
use crate::IError;
use diesel::prelude::*;
use diesel_async::AsyncMysqlConnection;
use diesel_async::RunQueryDsl;

pub struct UserInfoRepo {}

impl UserInfoRepo {
  /// get user by uid
  pub async fn delete_infos_by_user_id(
    conn: &mut AsyncMysqlConnection,
    user_id: &u32,
  ) -> Result<(), IError> {
    let _ = diesel::delete(info_dsl.filter(user_infos::user_id.eq(user_id)))
      .execute(conn)
      .await
      .map_err(|err| IError::ServerError(err.to_string()))?;
    Ok(())
  }

  /// insert users
  pub async fn insert_infos<'a>(
    conn: &mut AsyncMysqlConnection,
    infos: &Vec<NewUserInfoPayload<'a>>,
  ) -> Result<(), IError> {
    diesel::insert_into(user_infos::table)
      .values(infos)
      .execute(conn)
      .await
      .map_err(|err| IError::ServerError(err.to_string()))?;
    Ok(())
  }
}
