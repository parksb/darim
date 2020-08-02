use serde::{Deserialize, Serialize};

/// Arguments for `POST /users` API.
#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub user_public_key: String,
    pub token_key: String,
    pub token_pin: String,
}

/// Arguments for `PATCH /users/:id` API.
#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub name: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
}

/// Arguments for `POST /users/password` API.
#[derive(Serialize, Deserialize)]
pub struct ResetPasswordArgs {
    pub email: String,
    pub token_id: String,
    pub temporary_password: String,
    pub new_password: String,
}
