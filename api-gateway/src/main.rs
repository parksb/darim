use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use time::Duration;

/// A layer that defines data structure.
pub mod models {
    /// Model related to authentication.
    pub mod auth;
    /// Model related to error.
    pub mod error;
    /// Model related to post.
    pub mod post;
    /// Model related to user.
    pub mod user;
}

/// A presentation layer that makes API public and passes request to back-end service.
pub mod routes {
    /// API related to authentication.
    pub mod auth;
    /// API related to post.
    pub mod post;
    /// API related to user.
    pub mod user;
}

/// Reusable functions for multiple modules.
pub mod utils {
    /// Utilities related to HTTP.
    pub mod http_util;
    /// Utilities related to service.
    pub mod meta_util;
    /// Utilities related to session.
    pub mod session_util;
}

use utils::meta_util::{MetaInfo, ENV};

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

    let env = env::var("ENV").expect("ENV not found");
    let meta_info = MetaInfo::new(ENV::from_string(&env));

    let host = env::var("HOST").expect("HOST not found");
    let port = env::var("PORT").expect("PORT not found");
    let address = format!("{}:{}", host, port);

    let server = HttpServer::new(|| {
        let client_address = env::var("CLIENT_ADDRESS").expect("CLIENT_ADDRESS not found");
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&client_address)
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                        http::header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(
                CookieSession::signed(&[0; 64])
                    .secure(true)
                    .http_only(true)
                    .max_age_time(Duration::days(30)),
            )
            .service(health_check)
            .configure(routes::auth::init_routes)
            .configure(routes::post::init_routes)
            .configure(routes::user::init_routes)
    });

    println!("Server running at {}", address);

    if meta_info.is_production() {
        let cert_file_path = env::var("TLS_CERT_FILE_PATH").expect("TLS_CERT_FILE_PATH not found");
        let key_file_path = env::var("TLS_KEY_FILE_PATH").expect("TLS_KEY_FILE_PATH not found");

        let mut config = ServerConfig::new(NoClientAuth::new());
        let cert_file = &mut BufReader::new(File::open(cert_file_path).unwrap());
        let key_file = &mut BufReader::new(File::open(key_file_path).unwrap());
        let cert_chain = certs(cert_file).unwrap();
        let mut keys = pkcs8_private_keys(key_file).unwrap();

        config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
        server.bind_rustls(address, config)
    } else {
        server.bind(address)
    }?
    .run()
    .await
}
