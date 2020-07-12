use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::env;

use chrono::Duration;
use darim::routes;

/// Health check
#[get("/")]
async fn health_check() -> impl Responder {
    let mut response = HashMap::new();
    response.insert("version", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().json(response)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let address = env::var("ADDRESS").expect("ADDRESS not found");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .wrap(
                CookieSession::signed(&[0; 64])
                    .secure(false) // It should be `true` in production.
                    .http_only(true)
                    .max_age_time(Duration::days(30)),
            )
            .service(health_check)
            .configure(routes::post::init_routes)
            .configure(routes::user::init_routes)
            .configure(routes::auth::init_routes)
    })
    .bind(address)?
    .run()
    .await
}
