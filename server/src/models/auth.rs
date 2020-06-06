use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginArgs {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResult {
    pub user_id: u64,
    pub user_email: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: u64,
    pub user_email: String,
    pub user_name: String,
}
