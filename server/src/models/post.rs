use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
  pub id: Option<i32>,
  pub author: String,
  pub content: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
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
      created_at: Utc::now().naive_utc(),
      updated_at: Utc::now().naive_utc(),
    }
  }
}
