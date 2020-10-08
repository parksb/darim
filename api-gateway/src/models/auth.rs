use serde::{Deserialize, Serialize};

/// Arguments for `GET /auth` API.
#[derive(Serialize, Deserialize)]
pub struct LoginArgs {
    pub email: String,
    pub password: String,
}

/// Arguments for `POST /auth/token` API.
#[derive(Serialize, Deserialize)]
pub struct SetSignUpTokenArgs {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

/// Arguments for `POST /auth/token/password` API.
#[derive(Serialize, Deserialize)]
pub struct SetPasswordTokenArgs {
    pub email: String,
}

/// Session containing information of the logged-in user.
#[derive(Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: u64,
    pub user_email: String,
    pub user_name: String,
    pub user_public_key: String,
    pub user_avatar_url: Option<String>,
}
