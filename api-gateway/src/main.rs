use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;

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
    /// Utilities related to dotenv.
    pub mod env_util;
    /// Utilities related to HTTP.
    pub mod http_util;
    /// Utilities related to service.
    pub mod meta_util;
}

use utils::env_util::{CLIENT_ADDRESS, ENV, HOST, PORT, TLS_CERT_FILE_PATH, TLS_KEY_FILE_PATH};
use utils::meta_util::{MetaInfo, ENVIRONMENT};

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

    let meta_info = MetaInfo::new(ENVIRONMENT::from_string(&ENV));
    let address = format!("{}:{}", *HOST, *PORT);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&CLIENT_ADDRESS)
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                        http::header::CONTENT_TYPE,
                        http::header::AUTHORIZATION,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(health_check)
            .configure(routes::auth::init_routes)
            .configure(routes::post::init_routes)
            .configure(routes::user::init_routes)
    });

    println!("Server running at {}", address);

    if meta_info.is_production() {
        let mut config = ServerConfig::new(NoClientAuth::new());
        let cert_file = &mut BufReader::new(File::open(&*TLS_CERT_FILE_PATH).unwrap());
        let key_file = &mut BufReader::new(File::open(&*TLS_KEY_FILE_PATH).unwrap());
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
