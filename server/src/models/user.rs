use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct UserToResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserToCreate {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserToUpdate {
    pub name: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub user_id: String,
    pub name: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
}
