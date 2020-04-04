use chrono::{NaiveDateTime, Utc};

#[derive(Debug)]
pub struct Post {
  id: Option<i32>,
  author: String,
  content: String,
  created_at: NaiveDateTime,
  updated_at: NaiveDateTime,
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
