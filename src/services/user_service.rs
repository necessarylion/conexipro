use std::sync::Arc;

use actix_multipart::form::tempfile::TempFile;
use serde_json::{json, Value};

use crate::{
  app::get_file_url,
  models::{UpdateUserPayload, User},
  repository::UserRepo,
  requests::{ChangeUsernameRequest, UserUpdateRequest},
  utils::{
    db::{get_db_conn, DbConn},
    to_str, validate_file,
  },
  Auth, DbPool, IError,
};

use super::StorageService;

pub struct UserService {
  pub pool: Arc<DbPool>,
  pub auth: Auth,
}

impl UserService {
  /// update user data such as names
  pub async fn update_user_data(&mut self, payload: UserUpdateRequest) -> Result<User, IError> {
    let auth = &self.auth;
    let uid = auth.uid();
    let conn: &mut DbConn = &mut get_db_conn(&self.pool).await?;
    let data = UpdateUserPayload {
      first_name: payload.first_name.as_ref().unwrap(),
      last_name: to_str(payload.last_name.as_ref()),
      middle_name: to_str(payload.middle_name.as_ref()),
      display_name: to_str(payload.display_name.as_ref()),
    };
    UserRepo::update_user_by_uid(conn, &uid, data).await
  }

  /// update avatar
  pub async fn change_avatar(&mut self, avatar: &TempFile) -> Result<Value, IError> {
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

    UserRepo::update_avatar_by_uid(conn, auth.uid(), &avatar).await?;
    Ok(json!({
      "avatar": get_file_url(&avatar)
    }))
  }

  /// change username
  pub async fn change_user_name(
    &mut self,
    payload: ChangeUsernameRequest,
  ) -> Result<Value, IError> {
    let auth = &self.auth;

    let username = payload.username.as_ref().unwrap();

    // get db connection
    let conn: &mut DbConn = &mut get_db_conn(&self.pool).await?;
    let user = UserRepo::get_user_by_username(conn, username).await;

    let success = json!({
      "success": true
    });

    // if record do not found, the function will return error.
    // which mean username is not taken and able to change.
    if user.is_err() {
      UserRepo::update_username_by_uid(conn, auth.uid(), username).await?;
      return Ok(success);
    }

    let user = user?;

    // if found user and auth user is the same,
    // no need to do since it is the same username
    if &user.uid == auth.uid() {
      return Ok(success);
    }

    // if record found and not match with auth user,
    // it mean username is already taken.
    Ok(json!({
      "success": false,
      "message": format!("username {} is already taken", username)
    }))
  }
}
