use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;

#[macro_use]
extern crate diesel;

/// A data layer that can access the database and define data structures.
pub mod models {
    /// Model related to authentication.
    pub mod auth {
        pub mod jwt_claims;
        pub mod jwt_refresh;
        pub mod password_token;
        pub mod sign_up_token;
    }
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
    pub mod auth {
        pub mod jwt_refresh;
        pub mod password_token;
        pub mod sign_up_token;
        pub mod user_session;
    }
    /// Service related to post.
    pub mod post;
    /// Service related to user.
    pub mod user;
}

/// Reusable functions for multiple modules.
pub mod utils {
    /// Utilities related to argon2 password.
    pub mod argon2_password_util;
    /// Utilities related to email.
    pub mod email_util;
    /// Utilities related to dotenv.
    pub mod env_util;
    /// Utilities related to HTTP.
    pub mod http_util;
    /// Utilities related to scrypt password.
    pub mod scrypt_password_util;
}

/// A database schema.
pub mod schema;

use crate::models::connection::connect_rdb;
use utils::env_util::{HOST, PORT};

/// Health check
#[get("/")]
async fn health_check() -> impl Responder {
    let mut response = HashMap::new();
    response.insert("version", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv().expect("Failed to read .env file");

    let address = format!("{}:{}", *HOST, *PORT);
    let rdb_pool = connect_rdb();

    println!("Server running at {}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(rdb_pool.clone()))
            .service(health_check)
            .configure(routes::post::init_routes)
            .configure(routes::user::init_routes)
            .configure(routes::auth::init_routes)
    })
    .bind(address)?
    .run()
    .await
}
