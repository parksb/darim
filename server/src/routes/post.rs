use actix_web::{get, post, delete, Responder, HttpResponse, web};
use std::collections::HashMap;

use crate::services;
use crate::models;

/// List posts
#[get("/posts")]
pub async fn posts() -> impl Responder {
    let mut response = HashMap::new();
    response.insert("data", services::post::get_list().unwrap_or(vec!()));
    HttpResponse::Ok().json(response)
}

/// Create a post
#[post("/posts")]
pub async fn create_post(post: web::Json<models::post::CreatePostArgs>) -> impl Responder {
    let mut response = HashMap::new();
    response.insert("data", services::post::create(post.into_inner()).unwrap_or(false));
    HttpResponse::Ok().json(response)
}

/// Delete a post
#[delete("/posts/{id}")]
pub async fn delete_post(id: web::Path<i32>) -> impl Responder {
    let mut response = HashMap::new();
    response.insert("data", services::post::delete(id.into_inner()).unwrap_or(false));
    HttpResponse::Ok().json(response)
}
