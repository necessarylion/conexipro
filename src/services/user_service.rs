use std::sync::Arc;

use super::StorageService;
use crate::{
  app::get_file_url,
  models::{user_info::NewUserInfoPayload, UpdateUserPayload, User},
  repository::{UserInfoRepo, UserRepo},
  requests::{ChangeUsernameRequest, UserInfoUpdateRequest, UserUpdateRequest},
  response::{ChangeAvatarResponse, ChangeUsernameResponse, UserDetailResponse},
  utils::{
    db::{get_db_conn, DbConn},
    validate_file, SomeStr, StrWithDefault,
  },
  Auth, DbPool, IError,
};
use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Data;

pub struct UserService {
  pub pool: Data<DbPool>,
  pub auth: Arc<Auth>,
}

impl UserService {
  /// update user data such as names
  pub async fn update_user_data(&mut self, payload: UserUpdateRequest) -> Result<User, IError> {
    let auth = &self.auth;
    let id = auth.id();
    let conn: &mut DbConn = &mut get_db_conn(&self.pool).await?;
    let data = UpdateUserPayload {
      first_name: StrWithDefault(payload.first_name.as_ref(), ""),
      last_name: SomeStr(payload.last_name.as_ref()),
      middle_name: SomeStr(payload.middle_name.as_ref()),
      display_name: SomeStr(payload.display_name.as_ref()),
    };
    UserRepo::update_user_by_id(conn, id, data).await
  }

  /// update avatar
  pub async fn change_avatar(&mut self, avatar: &TempFile) -> Result<ChangeAvatarResponse, IError> {
    validate_file("avatar", 2.0, avatar)?;

    let file = &mut avatar.file.as_file();
    let content_type = avatar.content_type.as_ref().unwrap().to_string();

    let storage = StorageService::new();
    let avatar = storage.put(file, content_type).await?;

    let conn: &mut DbConn = &mut get_db_conn(&self.pool).await?;
    let auth = &self.auth;
    let user = auth.user(&self.pool).await?;

    // delete old avatar
    let old_file = user.avatar;
    if old_file.is_some() {
      storage.delete(old_file.as_ref().unwrap()).await?;
    }

    UserRepo::update_avatar_by_id(conn, auth.id(), &avatar).await?;
    Ok(ChangeAvatarResponse {
      avatar: get_file_url(&avatar),
    })
  }

  /// change username
  pub async fn change_user_name(
    &mut self,
    payload: ChangeUsernameRequest,
  ) -> Result<ChangeUsernameResponse, IError> {
    let auth = &self.auth;

    let username = payload.username.as_ref().unwrap();

    // get db connection
    let conn: &mut DbConn = &mut get_db_conn(&self.pool).await?;
    let user = UserRepo::get_user_by_username(conn, username).await;

    let success = ChangeUsernameResponse {
      success: true,
      message: String::from("username updated successfully"),
    };

    // if record do not found, the function will return error.
    // which mean username is not taken and able to change.
    if user.is_err() {
      UserRepo::update_username_by_id(conn, auth.id(), username).await?;
      return Ok(success);
    }

    let user = user?;

    // if found user and auth user is the same,
    // no need to do since it is the same username
    if &user.id == auth.id() {
      return Ok(success);
    }

    // if record found and not match with auth user,
    // it mean username is already taken.
    Ok(ChangeUsernameResponse {
      success: false,
      message: format!("username {} is already taken", username),
    })
  }

  /// delte and insert user infos
  pub async fn update_user_info(
    &mut self,
    payload: UserInfoUpdateRequest,
  ) -> Result<UserDetailResponse, IError> {
    let user_id = self.auth.id();
    let conn: &mut DbConn = &mut get_db_conn(&self.pool).await?;

    // create payload to insert with array into db
    let mut records: Vec<NewUserInfoPayload> = Vec::new();
    // this line is important to prvent borrow error
    let infos = payload.infos.unwrap_or(Vec::new());

    for info in infos.iter() {
      let payload: NewUserInfoPayload = NewUserInfoPayload {
        user_id,
        info_key: StrWithDefault(info.info_key.as_ref(), ""),
        info_value: SomeStr(info.info_value.as_ref()),
        info_type: StrWithDefault(info.info_type.as_ref(), "contact"),
      };
      records.push(payload);
    }

    // delete old infos
    UserInfoRepo::delete_infos_by_user_id(conn, user_id).await?;

    // insert new infos
    UserInfoRepo::insert_infos(conn, &records).await?;

    // get user detail and infos
    let user = self.auth.user(&self.pool).await?;
    let infos = user.infos(conn).await?;
    Ok(UserDetailResponse { user, infos })
  }
}
