use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct PostToShow {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct PostToCreate {
    pub user_id: u64,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct PostToUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDateTime>,
}
