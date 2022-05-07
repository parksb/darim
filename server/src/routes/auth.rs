use actix_web::{delete, get, post, web, Responder};
use serde::{Deserialize, Serialize};

use crate::models::connection::{RdbPool, RedisPool};
use crate::services::auth::jwt_refresh::{JwtRefreshService, SetJwtRefreshDTO};
use crate::services::auth::password::PasswordService;
use crate::services::auth::sign_up_token::SignUpTokenService;
use crate::services::auth::user_session::{UserSessionDTO, UserSessionService};
use crate::utils::http_util;

/// Arguments for `GET /auth` API.
#[derive(Serialize, Deserialize)]
pub struct LoginArgs {
    pub email: String,
    pub password: String,
    pub user_agent: Option<String>,
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
    pub token_uuid: String,
    pub jwt_refresh: String,
    pub user_agent: Option<String>,
}

/// Arguments for `POST /auth/token/access/:id` API.
#[derive(Serialize, Deserialize)]
pub struct ValidateJwtAccessArgs {
    pub token_uuid: String,
    pub jwt_access: String,
}

/// Arguments for `DELETE /auth/token/refresh/:id` API.
#[derive(Serialize, Deserialize)]
pub struct RemoveJwtRefreshArgs {
    pub token_uuid: String,
}

/// Sets token for creating user.
#[post("/auth/token/sign_up")]
pub async fn set_sign_up_token(
    redis_pool: web::Data<RedisPool>,
    args: web::Json<SetSignUpTokenArgs>,
) -> impl Responder {
    let SetSignUpTokenArgs {
        name,
        email,
        password,
        avatar_url,
    } = args.into_inner();
    let mut redis_conn = redis_pool.get().unwrap();
    let result =
        SignUpTokenService::new(&mut redis_conn).set(&name, &email, &password, &avatar_url);
    http_util::response::<String>(result)
}

/// Sets token for resetting password.
#[post("/auth/token/password")]
pub async fn set_password_token(
    rdb_pool: web::Data<RdbPool>,
    redis_pool: web::Data<RedisPool>,
    args: web::Json<SetPasswordTokenArgs>,
) -> impl Responder {
    let SetPasswordTokenArgs { email } = args.into_inner();
    let rdb_conn = rdb_pool.get().unwrap();
    let mut redis_conn = redis_pool.get().unwrap();
    let result = PasswordService::new(&rdb_conn, &mut redis_conn).set(&email);
    http_util::response::<bool>(result)
}

/// Sets a JWT refresh token.
#[post("/auth/token/refresh")]
pub async fn set_jwt_refresh(
    rdb_pool: web::Data<RdbPool>,
    redis_pool: web::Data<RedisPool>,
    args: web::Json<LoginArgs>,
) -> impl Responder {
    let LoginArgs {
        email,
        password,
        user_agent,
    } = args.into_inner();
    let rdb_conn = rdb_pool.get().unwrap();
    let mut redis_conn = redis_pool.get().unwrap();
    let result =
        JwtRefreshService::new(&rdb_conn, &mut redis_conn).set(&email, &password, user_agent);
    http_util::response::<SetJwtRefreshDTO>(result)
}

/// Validates JWT refresh token.
#[post("/auth/token/refresh/{id}")]
pub async fn validate_jwt_refresh(
    rdb_pool: web::Data<RdbPool>,
    redis_pool: web::Data<RedisPool>,
    id: web::Path<u64>,
    args: web::Json<ValidateJwtRefreshArgs>,
) -> impl Responder {
    let ValidateJwtRefreshArgs {
        token_uuid,
        jwt_refresh,
        user_agent,
    } = args.into_inner();
    let user_id = id.into_inner();
    let rdb_conn = rdb_pool.get().unwrap();
    let mut redis_conn = redis_pool.get().unwrap();
    let result = JwtRefreshService::new(&rdb_conn, &mut redis_conn).validate(
        user_id,
        &token_uuid,
        &jwt_refresh,
        user_agent,
    );
    http_util::response::<bool>(result)
}

/// Removes JWT refresh and access tokens.
#[delete("/auth/token/refresh/{id}")]
pub async fn remove_jwt_refresh(
    rdb_pool: web::Data<RdbPool>,
    redis_pool: web::Data<RedisPool>,
    id: web::Path<u64>,
    args: web::Json<RemoveJwtRefreshArgs>,
) -> impl Responder {
    let RemoveJwtRefreshArgs { token_uuid } = args.into_inner();
    let user_id = id.into_inner();
    let rdb_conn = rdb_pool.get().unwrap();
    let mut redis_conn = redis_pool.get().unwrap();
    let result = JwtRefreshService::new(&rdb_conn, &mut redis_conn).remove(user_id, &token_uuid);
    http_util::response::<bool>(result)
}

/// Get active tokens as session.
#[get("/auth/token/{id}")]
pub async fn get_session_list(
    redis_pool: web::Data<RedisPool>,
    id: web::Path<u64>,
) -> impl Responder {
    let user_id = id.into_inner();
    let mut redis_conn = redis_pool.get().unwrap();
    let result = UserSessionService::new(&mut redis_conn).get_all(user_id);
    http_util::response::<Vec<UserSessionDTO>>(result)
}

/// Initializes the auth routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(set_sign_up_token);
    cfg.service(set_password_token);
    cfg.service(set_jwt_refresh);
    cfg.service(validate_jwt_refresh);
    cfg.service(remove_jwt_refresh);
    cfg.service(get_session_list);
}
