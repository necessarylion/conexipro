use serde_json::{json, Value};

use crate::{
  models::{UpdateUserPayload, User},
  repository::UserRepo,
  requests::{ChangeUsernameRequest, UserUpdateRequest},
  utils::to_str,
  Auth, DbConn, IError,
};

pub struct UserService {
  pub conn: DbConn,
  pub auth: Auth,
}

impl UserService {
  /// update user data such as names
  pub fn update_user_data(&mut self, payload: UserUpdateRequest) -> Result<User, IError> {
    let conn: &mut DbConn = &mut self.conn;
    let auth = &self.auth;

    let data = UpdateUserPayload {
      first_name: payload.first_name.as_ref().unwrap(),
      last_name: to_str(payload.last_name.as_ref()),
      middle_name: to_str(payload.middle_name.as_ref()),
      display_name: to_str(payload.display_name.as_ref()),
    };

    UserRepo::update_user_by_uid(conn, auth.uid(), data)
  }

  /// update avatar
  pub fn change_avatar(&mut self, avatar: &String) -> Result<Value, IError> {
    let conn: &mut DbConn = &mut self.conn;
    let auth = &self.auth;
    UserRepo::update_avatar_by_uid(conn, auth.uid(), avatar)?;
    Ok(json!({
      "avatar": avatar
    }))
  }

  /// change username
  pub fn change_user_name(&mut self, payload: ChangeUsernameRequest) -> Result<Value, IError> {
    // get db connection
    let conn: &mut DbConn = &mut self.conn;
    let auth = &self.auth;

    let username = payload.username.as_ref().unwrap();

    let user = UserRepo::get_user_by_username(conn, username);

    let success = json!({
      "success": true
    });

    // if record do not found, the function will return error.
    // which mean username is not taken and able to change.
    if user.is_err() {
      UserRepo::update_username_by_uid(conn, auth.uid(), username)?;
      return Ok(success);
    }

    let user = user?;

    // if found user and auth user is the same,
    // no need to do since it is the same username
    if user.id == auth.user.id {
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
