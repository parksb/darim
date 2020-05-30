use actix_session::Session;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::models::error::*;
use crate::models::post::*;
use crate::services::post;
use crate::utils::session_util;

/// List posts written by logged-in user
///
/// # Request
///
/// ```text
/// GET /posts
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": [
///         {
///             "id": 1,
///             "content": "Lorem ipsum dolor sit amet",
///             "date": "2020-04-12T07:43:03",
///             "created_at": "2020-04-13T16:31:09",
///             "updated_at": null
///         },
///         {
///             "id": 2,
///             "content": "Lorem ipsum dolor sit amet",
///             "date": "2020-04-10T07:43:03",
///             "created_at": "2020-05-07T07:43:03",
///             "updated_at": "2020-05-09T16:07:41"
///         },
///     ]
/// }
/// ```
#[get("/posts")]
pub async fn posts(session: Session) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        post::get_list(user_session.user_id)
    } else {
        Err(ServiceError::Unauthorized)
    };

    match response {
        Ok(result) => HttpResponse::Ok().json(json!({ "data": result })),
        Err(ServiceError::Unauthorized) => {
            HttpResponse::Unauthorized().body(format!("{}", ServiceError::Unauthorized))
        }
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

/// Create a post
///
/// # Request
///
/// ```text
/// POST /posts
/// ```
///
/// ## Parameters
///
/// * content - A content of the post.
///
/// ```json
/// {
///     "content": "Lorem ipsum dolor sit amet"
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
#[post("/posts")]
pub async fn create_post(session: Session, post: web::Json<CreateArgs>) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        post::create(user_session.user_id, post.into_inner())
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
        _ => HttpResponse::InternalServerError().body("internal server error"),
    }
}

/// Delete a post
///
/// # Request
///
/// ```text
/// DELETE /posts/:id
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": true
/// }
/// ```
#[delete("/posts/{id}")]
pub async fn delete_post(session: Session, id: web::Path<u64>) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        post::delete(id.into_inner(), user_session.user_id)
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

/// Update a post
///
/// # Request
///
/// ```text
/// PATCH /posts/:id
/// ```
///
/// ## Parameters
///
/// * content - A content of the post.
///
/// ```json
/// {
///     "content": "Lorem ipsum dolor sit amet"
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
#[patch("/posts/{id}")]
pub async fn update_post(
    session: Session,
    id: web::Path<u64>,
    args: web::Json<UpdateArgs>,
) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        post::update(id.into_inner(), user_session.user_id, args.into_inner())
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
    cfg.service(posts);
    cfg.service(create_post);
    cfg.service(delete_post);
    cfg.service(update_post);
}
