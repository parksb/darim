use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Arguments for `POST /posts` API.
#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
}

/// Arguments for `POST /posts` API of the service.
#[derive(Serialize, Deserialize)]
pub struct ServiceCreateArgs {
    pub user_id: u64,
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

/// Arguments for `PATCH /posts/:id` API of the service.
#[derive(Serialize, Deserialize)]
pub struct ServiceUpdateArgs {
    pub user_id: u64,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDateTime>,
}

/// Post DTO using between api gateway and the service.
#[derive(Serialize, Deserialize)]
pub struct PostDTO {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
