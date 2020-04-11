use seed::*;
use serde::Deserialize;

use crate::Msg;

const BASE_URL: &str = "http://localhost:8080";

#[derive(Clone, Deserialize)]
pub struct Response<T> {
    pub data: T,
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
