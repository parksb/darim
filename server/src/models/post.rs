use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: u64,
    pub user_id: u64,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct PostToShow {
    pub id: u64,
    pub user_id: u64,
    pub user_name: String,
    pub user_avatar_url: Option<String>,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct PostToCreate {
    pub user_id: u64,
    pub content: String,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct PostToUpdate {
    pub content: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub user_id: u64,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub user_id: u64,
    pub content: Option<String>,
}
