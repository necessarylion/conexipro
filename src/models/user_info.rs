use crate::models::User;
use crate::schema::user_infos;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

/// User Modal
#[derive(
  Queryable, Selectable, Associations, Identifiable, Serialize, Debug, PartialEq, ToSchema,
)]
#[diesel(table_name = user_infos)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserInfo {
  #[schema(example = "1")]
  #[serde(skip)]
  pub id: u32,

  #[schema(example = "1")]
  #[serde(skip)]
  pub user_id: u32,

  #[schema(example = "phone")]
  pub info_key: String,

  #[schema(example = "+66620350322")]
  pub info_value: Option<String>,

  #[schema(example = "contact")]
  pub info_type: Option<String>,

  #[serde(skip)]
  pub created_at: Option<chrono::NaiveDateTime>,

  #[serde(skip)]
  pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Creating new user info payload
#[derive(Insertable)]
#[diesel(table_name = user_infos)]
pub struct NewUserInfoPayload<'a> {
  pub user_id: &'a u32,
  pub info_key: &'a str,
  pub info_value: Option<&'a str>,
  pub info_type: &'a str,
}
