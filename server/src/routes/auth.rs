use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::models::{auth::*, error::*};
use crate::services::auth;
use crate::utils::session_util;

/// Get auth information as user session
///
/// # Request
///
/// ```text
/// GET /auth
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": {
///         "user_id": 0,
///         "user_email": "park@email.com"
///         "user_name": "park",
///         "user_avatar_url": "avatar.jpg"
///     }
/// }
/// ```
#[get("/auth")]
pub async fn get_auth(session: Session) -> impl Responder {
    let user_session = session_util::get_session(&session);

    if let Some(response) = user_session {
        HttpResponse::Ok().json(json!({ "data": response }))
    } else {
        HttpResponse::Unauthorized().body(format!("{}", ServiceError::Unauthorized))
    }
}

/// Set token to create user
///
/// # Request
///
/// ```text
/// POST /auth/token
/// ```
///
/// ## Parameters
///
/// * name - A name of the user.
/// * email - A unique email of the user.
/// * password - A password of the user.
/// * avatar_url - An avatar image url of the user.
///
/// ```json
/// {
///     "name": "park",
///     "email": "park@email.com",
///     "password": "Ir5c7y8dS3",
///     "avatar_url": "avatar.jpg"
/// }
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": true
/// }
/// ```
#[post("/auth/token")]
pub async fn set_sign_up_token(args: web::Json<SetSignUpTokenArgs>) -> impl Responder {
    let response = auth::set_sign_up_token(args.into_inner());
    match response {
        Ok(result) => HttpResponse::Ok().json(json!({ "data": result })),
        Err(ServiceError::InvalidArgument) => {
            HttpResponse::BadRequest().body(format!("{}", ServiceError::InvalidArgument))
        }
        Err(ServiceError::DuplicatedKey) => {
            HttpResponse::Conflict().body(format!("{}", ServiceError::DuplicatedKey))
        }
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

/// Login to set user session
///
/// # Request
///
/// ```text
/// POST /auth/login
/// ```
///
/// ## Parameters
///
/// * email - A unique email of the user.
/// * password - A password of the user.
///
/// ```json
/// {
///     "email": "park@email.com",
///     "password": "Ir5c7y8dS3",
/// }
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": {
///         "user_id": 0,
///         "user_email": "park@email.com"
///         "user_name": "park",
///     }
/// }
/// ```
#[post("/auth/login")]
pub async fn login(session: Session, args: web::Json<LoginArgs>) -> impl Responder {
    let user_session = auth::login(args.into_inner());

    match user_session {
        Ok(response) => {
            let is_succeed = session_util::set_session(
                session,
                response.user_id,
                &response.user_email,
                &response.user_name,
                &response.user_avatar_url,
            );

            if is_succeed {
                HttpResponse::Ok().json(json!({ "data": response }))
            } else {
                HttpResponse::InternalServerError().body("internal server error")
            }
        }
        Err(ServiceError::NotFound(key)) => {
            HttpResponse::NotFound().body(format!("{}", ServiceError::NotFound(key)))
        }
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

/// Logout to unset user session
///
/// # Request
///
/// ```text
/// POST /auth/logout
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": true
/// }
/// ```
#[post("/auth/logout")]
pub async fn logout(session: Session) -> impl Responder {
    let is_logged_in = session_util::get_session(&session);
    let result = if is_logged_in.is_some() {
        session_util::unset_session(session);
        true
    } else {
        false
    };

    HttpResponse::Ok().json(json!({ "data": result }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(set_sign_up_token);
    cfg.service(get_auth);
    cfg.service(login);
    cfg.service(logout);
}
