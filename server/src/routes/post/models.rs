use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Arguments for `POST /posts` API.
#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
}

/// Arguments for `PATCH /posts/:id` API.
#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDateTime>,
}
