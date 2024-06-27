use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};
use validator_derive::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct Info {
  #[validate(
    required(message = "info_key is required"),
    length(min = 1, message = "info_key is required")
  )]
  pub info_key: Option<String>,

  #[validate(
    required(message = "info_value is required"),
    length(min = 1, message = "info_value is required")
  )]
  pub info_value: Option<String>,

  pub info_type: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserInfoUpdateRequest {
  #[validate(
    required(message = "infos is required"),
    length(min = 1, message = "infos is required"),
    custom(function = "validate_infos_array")
  )]
  pub infos: Option<Vec<Info>>,
}

fn validate_infos_array(infos: &Vec<Info>) -> Result<(), ValidationError> {
  let mut index = 0;
  for info in infos {
    let res = info.validate();
    if res.is_err() {
      let errs = res.unwrap_err();
      for (_, errors) in errs.field_errors() {
        for error in errors {
          let mut err = error.clone();
          let msg = err.message.unwrap_or_default();
          err.message = Some(format!("{}.{}", index, msg).into());
          return Err(err);
        }
      }
    }
    index = index + 1
  }
  Ok(())
}
