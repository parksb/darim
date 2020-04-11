use seed::*;
use serde::Deserialize;

use crate::{Msg, NewPost, EditedPost};

const BASE_URL: &str = "http://localhost:8080";

#[derive(Clone, Deserialize)]
pub struct Response<T> {
    pub data: T,
}

pub async fn create(new_post: NewPost) -> Result<Msg, Msg> {
    fetch::Request::new(format!("{}/posts", BASE_URL))
        .method(Method::Post)
        .send_json(&new_post)
        .fetch_json_data(Msg::PostCreated)
        .await
}

pub async fn update(edited_post: EditedPost) -> Result<Msg, Msg> {
    fetch::Request::new(
        format!(
            "{}/posts/{}", BASE_URL, if let Some(id) = edited_post.id {
            id.to_string()
        } else {
            String::from("")
        })
    ).method(Method::Patch)
        .send_json(&edited_post)
        .fetch_json_data(Msg::PostUpdated)
        .await
}

pub async fn delete(id: u64) -> Result<Msg, Msg> {
    fetch::Request::new(format!("{}/posts/{}", BASE_URL, id))
        .method(Method::Delete)
        .fetch_json_data(Msg::PostDeleted)
        .await
}

pub async fn get_list() -> Result<Msg, Msg> {
    fetch::Request::new(format!("{}/posts", BASE_URL))
        .fetch_json_data(Msg::PostsFetched)
        .await
}
