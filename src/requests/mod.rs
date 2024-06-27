pub mod change_avatar_request;
pub mod change_username_request;
pub mod extra_requests;
pub mod user_info_update_request;
pub mod user_login_request;
pub mod user_update_request;

// exports
pub use change_avatar_request::ChangeAvatarRequest;
pub use change_username_request::ChangeUsernameRequest;
pub use extra_requests::ExtraRequests;
pub use user_info_update_request::UserInfoUpdateRequest;
pub use user_login_request::UserLoginRequest;
pub use user_update_request::UserUpdateRequest;
