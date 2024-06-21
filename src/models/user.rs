use crate::{schema::users, utils::full_file_url};
use diesel::prelude::*;
use serde::Serialize;

/// User Modal
#[derive(Queryable, Selectable, Serialize, Debug)]
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
  #[serde(serialize_with = "full_file_url")]
  pub avatar: Option<String>,
  pub created_at: Option<chrono::NaiveDateTime>,
  pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Creating new user payload
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserPayload<'a> {
  pub uid: &'a str,
  pub username: &'a str,
  pub first_name: &'a str,
  pub display_name: &'a str,
  pub email: Option<&'a str>,
}

/// Update user payload
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UpdateUserPayload<'a> {
  pub first_name: &'a str,
  pub last_name: Option<&'a str>,
  pub middle_name: Option<&'a str>,
  pub display_name: Option<&'a str>,
}
