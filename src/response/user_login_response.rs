use crate::{models::User, utils::jwt::JwtToken};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserLoginResponse {
  pub token: JwtToken,
  pub success: bool,
  pub user: User,
}
