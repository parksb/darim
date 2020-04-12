use std::env;
use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use std::collections::HashMap;

use patic::routes;

/// Health Check
#[get("/")]
async fn health_check() -> impl Responder {
    let mut response = HashMap::new();
    response.insert("version", format!("{}", env!("CARGO_PKG_VERSION")));
    HttpResponse::Ok().json(response)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let address = env::var("ADDRESS").expect("ADDRESS not found");
    let client_address = env::var("CLIENT_ADDRESS").expect("CLIENT_ADDRESS not found");

    HttpServer::new(move || App::new()
        .wrap(
            Cors::new()
                .allowed_origin(&client_address)
                .finish()
        )
        .service(health_check)
        .service(routes::post::posts)
        .service(routes::post::create_post)
        .service(routes::post::delete_post)
        .service(routes::post::update_post)
    ).bind(address)?
        .run()
        .await
}
