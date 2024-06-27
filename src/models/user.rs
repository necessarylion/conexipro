use crate::{schema::users, serializer::file_url};
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

/// User Modal
#[derive(Queryable, Selectable, Serialize, Debug, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
  #[schema(example = "1")]
  pub id: u32,

  #[schema(example = "UVsflAVCCSR1aaB1dzIh1TPdPG63")]
  pub uid: String,

  #[schema(example = "zinkyawkyaw")]
  pub username: String,

  #[schema(example = "aj.zinkyaw@gmail.com")]
  pub email: Option<String>,

  #[schema(example = "Zin Kyaw")]
  pub first_name: String,

  #[schema(example = "")]
  pub middle_name: Option<String>,

  #[schema(example = "Kyaw")]
  pub last_name: Option<String>,

  #[schema(example = "AJ")]
  pub display_name: Option<String>,

  #[serde(serialize_with = "file_url")]
  #[schema(
    example = "http://127.0.0.1:3335/files/conexipro-dev/72e1f1ea-7958-484b-97c2-b91d842e60c8.png"
  )]
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
