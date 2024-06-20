use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::errors::handler::IError;

#[derive(Clone, Debug)]
pub struct FireAuth {
  pub api_key: String, // web api key
}

impl FireAuth {
  /// A constructor for the FireAuth struct
  pub fn new() -> Self {
    Self {
      api_key: std::env::var("FIREBASE_WEB_API_KEY").expect("FIREBASE_API_KEY must be set"),
    }
  }
}

impl FireAuth {
  pub async fn get_user_info(&self, id_token: Option<&String>) -> Result<FirebaseUser, IError> {
    let url = format!(
      "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
      self.api_key,
    );

    let client = reqwest::Client::new();
    let resp = client
      .post(url)
      .header("Content-Type", "application/json")
      .json(&json!({"idToken": id_token}))
      .send()
      .await
      .map_err(|e| IError::ServerError(e.to_string()))?;

    if resp.status() != 200 {
      return Err(IError::Unauthorized(String::from(
        "Invalid firebase id token token",
      )));
    }

    let body = resp.json::<UserInfoResponse>().await.map_err(|e| {
      println!("{}", e);
      IError::ServerError(e.to_string())
    })?;

    Ok(body.users[0].clone())
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserInfoResponse {
  // kind: String,
  users: Vec<FirebaseUser>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseUser {
  pub local_id: Option<String>,
  pub email: Option<String>,
  pub password_hash: Option<String>,
  pub email_verified: Option<bool>,
  pub password_updated_at: Option<u64>,
  pub provider_user_info: Vec<ProviderUserInfo>,
  pub valid_since: Option<String>,
  pub last_login_at: Option<String>,
  pub created_at: Option<String>,
  pub last_refresh_at: Option<String>,
  pub display_name: Option<String>,
}

// Provider User Info
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUserInfo {
  pub provider_id: Option<String>,
  pub federated_id: Option<String>,
  pub email: Option<String>,
  pub raw_id: Option<String>,
}
