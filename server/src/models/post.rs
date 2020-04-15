use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: u64,
    pub author: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct PostToCreate {
    pub author: String,
    pub content: String,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct PostToUpdate {
    pub author: Option<String>,
    pub content: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub author: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub author: Option<String>,
    pub content: Option<String>,
}
