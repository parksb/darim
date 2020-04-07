use actix_web::{get, post, delete, patch, Responder, HttpResponse, web};
use serde_json::json;

use crate::services::post;
use crate::models::post::*;
use crate::models::error::*;

/// List posts
#[get("/posts")]
pub async fn posts() -> impl Responder {
    let response = post::get_list();
    match response {
        Ok(result) => HttpResponse::Ok().json(json!({ "data": result })),
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

/// Create a post
#[post("/posts")]
pub async fn create_post(post: web::Json<CreateArgs>) -> impl Responder {
    let response = post::create(post.into_inner());
    match response {
        Ok(result) => HttpResponse::Ok().json(json!({ "data": result })),
        Err(ServiceError::InvalidArgument) => HttpResponse::BadRequest()
            .body(format!("{}", ServiceError::InvalidArgument)),
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

/// Delete a post
#[delete("/posts/{id}")]
pub async fn delete_post(id: web::Path<u64>) -> impl Responder {
    let response = post::delete(id.into_inner());
    match response {
        Ok(result) => HttpResponse::Ok().json(json!({ "data": result })),
        Err(ServiceError::NotFound(key)) => HttpResponse::NotFound()
            .body(format!("{}", ServiceError::NotFound(key))),
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

/// Update a post
#[patch("/posts/{id}")]
pub async fn update_post(id: web::Path<u64>, post: web::Json<UpdateArgs>) -> impl Responder {
    let response = post::update(
        id.into_inner(),
        post.into_inner(),
    );
    match response {
        Ok(result) => HttpResponse::Ok().json(json!({ "data": result })),
        Err(ServiceError::InvalidArgument) => HttpResponse::BadRequest()
            .body(format!("{}", ServiceError::InvalidArgument)),
        Err(ServiceError::NotFound(key)) => HttpResponse::NotFound()
            .body(format!("{}", ServiceError::NotFound(key))),
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}
