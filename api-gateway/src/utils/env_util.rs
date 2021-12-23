use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref ENV: String = env::var("ENV").expect("ENV not found");
    pub static ref HOST: String = env::var("HOST").expect("HOST not found");
    pub static ref PORT: String = env::var("PORT").expect("PORT not found");
    pub static ref TLS_CERT_FILE_PATH: String =
        env::var("TLS_CERT_FILE_PATH").expect("TLS_CERT_FILE_PATH not found");
    pub static ref TLS_KEY_FILE_PATH: String =
        env::var("TLS_KEY_FILE_PATH").expect("TLS_KEY_FILE_PATH not found");
    pub static ref CLIENT_ADDRESS: String =
        env::var("CLIENT_ADDRESS").expect("CLIENT_ADDRESS not found");
    pub static ref BACK_END_SERVICE_ADDRESS: String =
        env::var("BACK_END_SERVICE_ADDRESS").expect("BACK_END_SERVICE_ADDRESS not found");
    pub static ref JWT_REFRESH_SECRET: String =
        env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET not found");
    pub static ref JWT_ACCESS_SECRET: String =
        env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET not found");
    pub static ref JWT_COOKIE_KEY: String = "jwt-refresh".to_string();
}
