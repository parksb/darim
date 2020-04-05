use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i32>,
    pub author: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Post {
    pub fn new(
        author: String,
        content: String,
    ) -> Self {
        Self {
            id: None,
            author,
            content,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostArgs {
    pub author: String,
    pub content: String,
}
