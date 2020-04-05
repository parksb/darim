use actix_web::{get, post, delete, patch, Responder, HttpResponse, web};
use serde_json::json;

use crate::services::post;
use crate::models::post::*;

/// List posts
#[get("/posts")]
pub async fn posts() -> impl Responder {
    let response = post::get_list().unwrap_or(vec!());
    HttpResponse::Ok().json(json!({ "data": response }))
}

/// Create a post
#[post("/posts")]
pub async fn create_post(post: web::Json<CreateArgs>) -> impl Responder {
   let response = post::create(post.into_inner()).unwrap_or(false);
    HttpResponse::Ok().json(json!({ "data": response }))
}

/// Delete a post
#[delete("/posts/{id}")]
pub async fn delete_post(id: web::Path<u64>) -> impl Responder {
    let response = post::delete(id.into_inner()).unwrap_or(false);
    HttpResponse::Ok().json(json!({ "data": response }))
}

/// Update a post
#[patch("/posts/{id}")]
pub async fn update_post(id: web::Path<u64>, post: web::Json<UpdateArgs>) -> impl Responder {
    let response = post::update(
        id.into_inner(),
        post.into_inner(),
    ).unwrap_or(false);
    HttpResponse::Ok().json(json!({ "data": response }))
}
