use crate::{
  models::NewUserPayload,
  repository::UserRepo,
  requests::{ExtraRequests, UserLoginRequest},
  response::{UserDetail, UserLoginResponse},
  utils::{
    db::{get_db_conn, DbConn},
    firebase::FireAuth,
    jwt, SomeStr,
  },
  DbPool, IError,
};
use actix_web::{
  get,
  web::{Data, Json},
  HttpRequest, HttpResponse, Responder,
};
use validator::Validate;

/// User login or registraion using firebase token
#[utoipa::path(
  post,
  path = "/auth/login",
  tag = "Auth",
  responses(
    (status = 200, description = "success", body = UserLoginResponse),
  ),
)]
pub async fn login(
  pool: Data<DbPool>,
  info: Json<UserLoginRequest>,
) -> Result<impl Responder, IError> {
  info.validate().map_err(IError::ValidationError)?;

  // verify firebase id token and get user info
  let id_token = info.token.as_ref();
  let auth = FireAuth::new();
  let firebase_user = auth.get_user_info(id_token).await?;
  let uid = firebase_user.get_uid();

  // get db connec
  let uid = uid.clone();
  let conn: &mut DbConn = &mut get_db_conn(&pool).await?;

  // check if user exists with uid
  let mut user = UserRepo::get_user_by_uid(conn, &uid).await;

  // if not exist create new one
  if user.is_err() {
    let display_name = firebase_user.display_name.as_ref().unwrap();
    let email = firebase_user.email.as_ref();
    let email = SomeStr(email);
    // prepare new user payload
    let new_user = NewUserPayload {
      uid: &uid,
      username: &uid,
      first_name: display_name,
      display_name,
      email,
    };
    user = UserRepo::create_user(conn, new_user).await;
  }

  let user = user?;
  let token = jwt::create(&user.id)?;
  let token_string = token.token.to_string();

  let res = UserLoginResponse {
    token,
    success: true,
    user,
  };

  Ok(
    HttpResponse::Ok()
      .insert_header(("Authorization", token_string))
      .json(res),
  )
}

/// Fetch user information
#[utoipa::path(
  tag = "Auth",
  responses(
    (status = 200, description = "success", body = UserDetailResponse),
  ),
  security(
    ("bearer_token" = [])
  )
)]
#[get("/auth/user")]
pub async fn fetch_user(req: HttpRequest, pool: Data<DbPool>) -> Result<impl Responder, IError> {
  let auth = req.auth()?;
  let user = auth.user(&pool).await?;
  let conn = &mut get_db_conn(&pool).await?;
  let infos = user.infos(conn).await?;
  let res = UserDetail { user, infos };
  Ok(Json(res))
}

/// Refresh user new token
#[utoipa::path(
  tag = "Auth",
  responses(
    (status = 200, description = "success", body = UserLoginResponse),
  ),
  security(
    ("bearer_token" = [])
  )
)]
#[get("/auth/refresh")]
pub async fn refresh_user_token(
  req: HttpRequest,
  pool: Data<DbPool>,
) -> Result<impl Responder, IError> {
  let auth = req.auth()?;
  let token = jwt::create(auth.id())?;
  let token_string = token.token.to_string();
  let user = auth.user(&pool).await?;

  let res = UserLoginResponse {
    token,
    success: true,
    user,
  };
  Ok(
    HttpResponse::Ok()
      .insert_header(("Authorization", token_string))
      .json(res),
  )
}
