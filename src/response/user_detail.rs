use crate::models::{User, UserInfo};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserDetail {
  #[serde(flatten)]
  pub user: User,
  #[schema(
    example = "[{ \"info_key\": \"facebook\", \"info_value\": \"https://facebook.com/zinkyaw\", \"info_type\": \"contact\" }]"
  )]
  pub infos: Vec<UserInfo>,
}
