use actix_web::{get, post, delete, App, HttpServer, Responder, HttpResponse, web};
use std::collections::HashMap;

use patic::services;
use patic::models;

/// Health Check
#[get("/")]
async fn health_check() -> impl Responder {
    let mut response = HashMap::new();
    response.insert("version", format!("{}", env!("CARGO_PKG_VERSION")));
    HttpResponse::Ok().json(response)
}

/// List posts
#[get("/posts")]
async fn posts() -> impl Responder {
    let mut response = HashMap::new();
    response.insert("data", services::post::get_list().unwrap_or(vec!()));
    HttpResponse::Ok().json(response)
}

/// Create a post
#[post("/posts")]
async fn create_post(post: web::Json<models::post::CreatePostArgs>) -> impl Responder {
    let mut response = HashMap::new();
    response.insert("data", services::post::create(post.into_inner()).unwrap_or(false));
    HttpResponse::Ok().json(response)
}

/// Delete a post
#[delete("/posts/{id}")]
async fn delete_post(id: web::Path<i32>) -> impl Responder {
    let mut response = HashMap::new();
    response.insert("data", services::post::delete(id.into_inner()).unwrap_or(false));
    HttpResponse::Ok().json(response)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";

    HttpServer::new(|| App::new()
        .service(health_check)
        .service(posts)
        .service(create_post)
        .service(delete_post)
    ).bind(address)?
        .run()
        .await
}
