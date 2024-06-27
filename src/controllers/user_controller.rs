use crate::{
  requests::{
    ChangeAvatarRequest, ChangeUsernameRequest, ExtraRequests, UserInfoUpdateRequest,
    UserUpdateRequest,
  },
  services::UserService,
  DbPool, IError,
};
use actix_multipart::form::MultipartForm;
use actix_web::{
  post, put,
  web::{self, Json},
  HttpRequest, Responder,
};
use validator::Validate;

/// update user information
#[utoipa::path(
  tag = "User",
  responses(
    (status = 200, description = "success", body = User),
  ),
  security(
    ("bearer_token" = [])
  )
)]
#[post("/auth/user")]
pub async fn update_user(
  req: HttpRequest,
  pool: web::Data<DbPool>,
  payload: Json<UserUpdateRequest>,
) -> Result<impl Responder, IError> {
  // validate request
  payload.validate().map_err(IError::ValidationError)?;
  // get auth
  let auth = req.auth()?;
  let mut user_service = UserService { pool, auth };
  let res = user_service.update_user_data(payload.into_inner()).await?;
  Ok(Json(res))
}

/// change username
#[utoipa::path(
  tag = "User",
  responses(
    (status = 200, description = "success", body = ChangeUsernameResponse),
  ),
  security(
    ("bearer_token" = [])
  )
)]
#[put("/auth/user/username")]
pub async fn change_username(
  req: HttpRequest,
  pool: web::Data<DbPool>,
  payload: Json<ChangeUsernameRequest>,
) -> Result<impl Responder, IError> {
  // validate request
  payload.validate().map_err(IError::ValidationError)?;
  // get auth
  let auth = req.auth()?;
  let mut user_service = UserService { pool, auth };
  let res = user_service.change_user_name(payload.into_inner()).await?;
  return Ok(Json(res));
}

/// change user avatar image
#[utoipa::path(
  tag = "User",
  request_body(content = ChangeAvatarRequest, content_type = "multipart/form-data"),
  responses(
    (status = 200, description = "success", body = ChangeAvatarResponse),
  ),
  security(
    ("bearer_token" = [])
  )
)]
#[put("/auth/user/avatar")]
pub async fn change_avatar(
  req: HttpRequest,
  pool: web::Data<DbPool>,
  MultipartForm(form): MultipartForm<ChangeAvatarRequest>,
) -> Result<impl Responder, IError> {
  let auth = req.auth()?;
  let avatar = form.avatar;
  let mut user_service = UserService { pool, auth };
  let res = user_service.change_avatar(&avatar).await?;
  Ok(Json(res))
}

/// update user informations
#[utoipa::path(
  tag = "User",
  responses(
    (status = 200, description = "success", body = UserDetailResponse),
  ),
  security(
    ("bearer_token" = [])
  )
)]
#[post("/auth/user/infos")]
pub async fn update_user_infos(
  req: HttpRequest,
  pool: web::Data<DbPool>,
  payload: Json<UserInfoUpdateRequest>,
) -> Result<impl Responder, IError> {
  // validate request
  payload.validate().map_err(IError::ValidationError)?;
  let auth = req.auth()?;
  let infos = payload.into_inner();
  let mut user_service = UserService { pool, auth };
  let res = user_service.update_user_info(infos).await?;
  Ok(Json(res))
}
