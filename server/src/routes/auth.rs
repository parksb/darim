use actix_web::{delete, post, web, Responder};
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

/// Arguments for `POST /auth/token/refresh/:id` API.
#[derive(Serialize, Deserialize)]
pub struct ValidateJwtRefreshArgs {
    pub jwt_refresh: String,
}

/// Arguments for `DELETE /auth/token/refresh/:id` API.
#[derive(Serialize, Deserialize)]
pub struct RemoveJwtRefreshArgs {
    pub jwt_refresh: String,
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

/// Sets a JWT refresh token.
#[post("/auth/token/refresh")]
pub async fn set_jwt_refresh(args: web::Json<LoginArgs>) -> impl Responder {
    let LoginArgs { email, password } = args.into_inner();
    let result = AuthService::new().set_jwt_refresh(&email, &password);
    http_util::get_response::<SetJwtRefreshDTO>(result)
}

/// Validates JWT refresh token.
#[post("/auth/token/refresh/{id}")]
pub async fn validate_jwt_refresh(
    id: web::Path<u64>,
    args: web::Json<ValidateJwtRefreshArgs>,
) -> impl Responder {
    let ValidateJwtRefreshArgs { jwt_refresh } = args.into_inner();
    let user_id = id.into_inner();
    let result = AuthService::new().validate_jwt_refresh(user_id, &jwt_refresh);
    http_util::get_response::<bool>(Ok(result))
}

/// Removes JWT refresh and access tokens.
#[delete("/auth/token/refresh/{id}")]
pub async fn remove_jwt_refresh(
    id: web::Path<u64>,
    args: web::Json<RemoveJwtRefreshArgs>,
) -> impl Responder {
    let RemoveJwtRefreshArgs { jwt_refresh } = args.into_inner();
    let user_id = id.into_inner();
    let result = AuthService::new().remove_jwt_refresh(user_id, &jwt_refresh);
    http_util::get_response::<bool>(Ok(result))
}

/// Initializes the auth routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(set_sign_up_token);
    cfg.service(set_password_token);
    cfg.service(set_jwt_refresh);
    cfg.service(validate_jwt_refresh);
    cfg.service(remove_jwt_refresh);
}
