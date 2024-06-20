use crate::models::user::User;

pub struct Auth {
  pub user: User,
}

impl Auth {
  pub fn uid(&self) -> &String {
    &self.user.uid
  }
}
