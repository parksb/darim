use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use std::collections::HashMap;

use patic::services;

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
  let response = services::post::get_list().unwrap();
  HttpResponse::Ok().json(response)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";

    HttpServer::new(|| App::new()
        .service(health_check)
        .service(posts)
    ).bind(address)?
        .run()
        .await
}
