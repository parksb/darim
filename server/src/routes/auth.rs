use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;

use crate::models::{auth::*, error::*};
use crate::services::auth;
use crate::utils::session_util;

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
///     "data": true
/// }
/// ```
#[post("/auth/login")]
pub async fn login(session: Session, args: web::Json<LoginArgs>) -> impl Responder {
    let response = auth::login(args.into_inner());

    match response {
        Ok(user_session) => {
            let result = session_util::set_session(
                session,
                &user_session.user_id,
                &user_session.user_email,
                &user_session.user_name,
            );
            HttpResponse::Ok().json(json!({ "data": result }))
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
    cfg.service(login);
    cfg.service(logout);
}
