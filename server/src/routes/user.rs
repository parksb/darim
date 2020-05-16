use actix_session::Session;
use actix_web::{delete, patch, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::models::error::*;
use crate::models::user::*;
use crate::services::user;
use crate::utils::session_util;

/// Create a user
#[post("/users")]
pub async fn create_user(user: web::Json<CreateArgs>) -> impl Responder {
    let response = user::create(user.into_inner());
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

/// Delete a user
#[delete("/users/{id}")]
pub async fn delete_user(session: Session, id: web::Path<u64>) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        user::delete(id.into_inner(), user_session.user_id)
    } else {
        Err(ServiceError::Unauthorized)
    };

    match response {
        Ok(result) => HttpResponse::Ok().json(json!({ "data": result })),
        Err(ServiceError::Unauthorized) => {
            HttpResponse::Unauthorized().body(format!("{}", ServiceError::Unauthorized))
        }
        Err(ServiceError::NotFound(key)) => {
            HttpResponse::NotFound().body(format!("{}", ServiceError::NotFound(key)))
        }
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

/// Update a user
#[patch("/users/{id}")]
pub async fn update_user(
    session: Session,
    id: web::Path<u64>,
    user: web::Json<UpdateArgs>,
) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        user::update(id.into_inner(), user_session.user_id, user.into_inner())
    } else {
        Err(ServiceError::Unauthorized)
    };

    match response {
        Ok(result) => HttpResponse::Ok().json(json!({ "data": result })),
        Err(ServiceError::Unauthorized) => {
            HttpResponse::Unauthorized().body(format!("{}", ServiceError::Unauthorized))
        }
        Err(ServiceError::InvalidArgument) => {
            HttpResponse::BadRequest().body(format!("{}", ServiceError::InvalidArgument))
        }
        Err(ServiceError::NotFound(key)) => {
            HttpResponse::NotFound().body(format!("{}", ServiceError::NotFound(key)))
        }
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(delete_user);
    cfg.service(update_user);
}
