use actix_web::{delete, get, patch, post, web, Responder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::post::*;
use crate::services::post::PostService;
use crate::utils::http_util;

/// Arguments for `POST /posts` API.
#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub user_id: u64,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
}

/// Arguments for `PATCH /posts/:id` API.
#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub user_id: u64,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDateTime>,
}

/// Responds a post written by logged-in user
#[get("/posts/{user_id}")]
pub async fn get_posts(user_id: web::Path<u64>) -> impl Responder {
    let posts = PostService::new().get_list(user_id.into_inner());
    http_util::get_response::<Vec<PostDTO>>(posts)
}

/// Lists posts written by logged-in user
#[get("/posts/{user_id}/{id}")]
pub async fn get_post(web::Path((user_id, id)): web::Path<(u64, u64)>) -> impl Responder {
    let post = PostService::new().get(user_id, id);
    http_util::get_response::<PostDTO>(post)
}

/// Creates a new post
#[post("/posts")]
pub async fn create_post(args: web::Json<CreateArgs>) -> impl Responder {
    let CreateArgs {
        user_id,
        title,
        content,
        date,
    } = args.into_inner();
    let result = PostService::new().create(user_id, &title, &content, &date);
    http_util::get_response::<u64>(result)
}

/// Deletes a post
#[delete("/posts/{user_id}/{id}")]
pub async fn delete_post(web::Path((user_id, id)): web::Path<(u64, u64)>) -> impl Responder {
    let result = PostService::new().delete(id, user_id);
    http_util::get_response::<bool>(result)
}

/// Updates a post
#[patch("/posts/{id}")]
pub async fn update_post(id: web::Path<u64>, args: web::Json<UpdateArgs>) -> impl Responder {
    let UpdateArgs {
        user_id,
        title,
        content,
        date,
    } = args.into_inner();
    let result = PostService::new().update(id.into_inner(), user_id, &title, &content, &date);
    http_util::get_response::<bool>(result)
}

/// Initializes the post routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_post);
    cfg.service(get_posts);
    cfg.service(create_post);
    cfg.service(delete_post);
    cfg.service(update_post);
}
