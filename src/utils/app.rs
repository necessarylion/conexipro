use super::get_env;

/// get app url
pub fn app_url() -> String {
  get_env("APP_URL").unwrap_or(String::from(""))
}

/// get file url
pub fn get_file_url(file_name: &str) -> String {
  format!("{}/files/{}", app_url(), file_name)
}
