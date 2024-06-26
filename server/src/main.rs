use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::env;

#[macro_use]
mod macros;

#[macro_use]
extern crate diesel;

/// A data layer that can access the database and define data structures.
pub mod models {
    /// Model related to authentication.
    pub mod auth;
    /// Model related to Database connection.
    pub mod connection;
    /// Model related to error.
    pub mod error;
    /// Model related to post.
    pub mod post;
    /// Model related to user.
    pub mod user;
    /// Model related to user key.
    pub mod user_key;
}

/// A presentation layer that makes API public and passes request/response data to other layers.
pub mod routes {
    /// API related to authentication.
    pub mod auth;
    /// API related to post.
    pub mod post;
    /// API related to user.
    pub mod user;
}

/// A business layer that processes the transaction.
pub mod services {
    /// Service related to authentication.
    pub mod auth;
    /// Service related to post.
    pub mod post;
    /// Service related to user.
    pub mod user;
}

/// Reusable functions for multiple modules.
pub mod utils {
    /// Utilities related to email.
    pub mod email_util;
    /// Utilities related to HTTP.
    pub mod http_util;
    /// Utilities related to password.
    pub mod password_util;
}

/// A database schema.
pub mod schema;

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

    let host = env::var("HOST").expect("HOST not found"); // 0.0.0.0
    let port = env!("PORT"); // 0000
    let address = format!("{}:{}", host, port);

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
