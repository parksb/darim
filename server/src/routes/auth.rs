use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;

use crate::models::auth::*;
use crate::models::error::*;
use crate::services::auth;

fn set_session(session: Session, user_session: &UserSession) {
    session.set("user_email", &user_session.user_email);
    session.set("user_name", &user_session.user_name);
}

#[post("/auth/login")]
pub async fn login(session: Session, args: web::Json<LoginArgs>) -> impl Responder {
    let response = auth::login(args.into_inner());

    match response {
        Ok(result) => {
            set_session(session, &result);
            HttpResponse::Ok().json(json!({ "data": result }))
        }
        Err(ServiceError::NotFound(key)) => {
            HttpResponse::NotFound().body(format!("{}", ServiceError::NotFound(key)))
        }
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}
