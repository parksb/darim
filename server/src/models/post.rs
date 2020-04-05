use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::Queryable;

use crate::schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: u64,
    pub author: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub author: String,
    pub content: String,
}
