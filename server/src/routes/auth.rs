use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};

use crate::models::auth::*;
use crate::services::auth::AuthService;
use crate::utils::http_util;

/// Arguments for `GET /auth` API.
#[derive(Serialize, Deserialize)]
pub struct LoginArgs {
    pub email: String,
    pub password: String,
}

/// Arguments for `POST /auth/token` API.
#[derive(Serialize, Deserialize)]
pub struct SetSignUpTokenArgs {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

/// Arguments for `POST /auth/token/password` API.
#[derive(Serialize, Deserialize)]
pub struct SetPasswordTokenArgs {
    pub email: String,
}

/// Sets token for creating user.
#[post("/auth/token/sign_up")]
pub async fn set_sign_up_token(args: web::Json<SetSignUpTokenArgs>) -> impl Responder {
    let SetSignUpTokenArgs {
        name,
        email,
        password,
        avatar_url,
    } = args.into_inner();
    let result = AuthService::new().set_sign_up_token(&name, &email, &password, &avatar_url);
    http_util::get_response::<String>(result)
}

/// Sets token for resetting password.
#[post("/auth/token/password")]
pub async fn set_password_token(args: web::Json<SetPasswordTokenArgs>) -> impl Responder {
    let SetPasswordTokenArgs { email } = args.into_inner();
    let result = AuthService::new().set_password_token(&email);
    http_util::get_response::<bool>(result)
}

/// Signs in to set user session.
#[post("/auth/login")]
pub async fn login(args: web::Json<LoginArgs>) -> impl Responder {
    let LoginArgs { email, password } = args.into_inner();
    let result = AuthService::new().login(&email, &password);
    http_util::get_response::<UserSession>(result)
}

/// Initializes the auth routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(set_sign_up_token);
    cfg.service(set_password_token);
    cfg.service(login);
}
