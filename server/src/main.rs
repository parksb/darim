use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::env;

use darim::routes;

/// Health check
#[get("/")]
async fn health_check() -> impl Responder {
    let mut response = HashMap::new();
    response.insert("version", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let address = env::var("ADDRESS").expect("ADDRESS not found");

    println!("Server running at {}", address);

    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .configure(routes::post::init_routes)
            .configure(routes::user::init_routes)
            .configure(routes::auth::init_routes)
    })
    .bind(address)?
    .run()
    .await
}
