use actix_session::Session;
use actix_web::{delete, get, patch, post, web, Responder};

use crate::models::error::*;
use crate::models::post::*;
use crate::services::post;
use crate::utils::{http_util, session_util};

/// Get a post written by logged-in user
///
/// # Request
///
/// ```text
/// GET /posts/:id
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": [
///         {
///             "id": 1,
///             "title": "Lorem ipsum",
///             "content": "Lorem ipsum dolor sit amet",
///             "date": "2020-04-12T07:43:03",
///             "created_at": "2020-04-13T16:31:09",
///             "updated_at": null
///         },
///     ],
///     "error": null
/// }
/// ```
pub fn get_post(session: Session, id: web::Path<u64>) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        post::get(user_session.user_id, id.into_inner())
    } else {
        Err(ServiceError::Unauthorized)
    };

    http_util::get_response::<PostDTO>(response)
}

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
///             "title": "Lorem ipsum",
///             "content": "Lorem ipsum dolor sit amet",
///             "date": "2020-04-12T07:43:03",
///             "created_at": "2020-04-13T16:31:09",
///             "updated_at": null
///         },
///         {
///             "id": 2,
///             "title": "Lorem ipsum",
///             "content": "Lorem ipsum dolor sit amet",
///             "date": "2020-04-10T07:43:03",
///             "created_at": "2020-05-07T07:43:03",
///             "updated_at": "2020-05-09T16:07:41"
///         },
///     ],
///     "error": null
/// }
/// ```
pub fn get_posts(session: Session) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        post::get_list(user_session.user_id)
    } else {
        Err(ServiceError::Unauthorized)
    };

    http_util::get_response::<Vec<PostDTO>>(response)
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
///     "title": "Lorem ipsum"
///     "content": "Lorem ipsum dolor sit amet"
///     "date": "2020-06-07T07:43:03",
/// }
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": 1,
///     "error": null
/// }
/// ```
pub fn create_post(session: Session, post: web::Json<CreateArgs>) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        post::create(user_session.user_id, post.into_inner())
    } else {
        Err(ServiceError::Unauthorized)
    };

    http_util::get_response::<u64>(response)
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
///     "data": true,
///     "error": null
/// }
/// ```
pub fn delete_post(session: Session, id: web::Path<u64>) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        post::delete(id.into_inner(), user_session.user_id)
    } else {
        Err(ServiceError::Unauthorized)
    };

    http_util::get_response::<bool>(response)
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
///     "data": true,
///     "error": null
/// }
/// ```
pub fn update_post(
    session: Session,
    id: web::Path<u64>,
    args: web::Json<UpdateArgs>,
) -> impl Responder {
    let response = if let Some(user_session) = session_util::get_session(&session) {
        post::update(id.into_inner(), user_session.user_id, args.into_inner())
    } else {
        Err(ServiceError::Unauthorized)
    };

    http_util::get_response::<bool>(response)
}

#[get("/posts/{id}")]
pub async fn get_post_route(session: Session, id: web::Path<u64>) -> impl Responder {
    get_post(session, id)
}

#[get("/posts")]
pub async fn get_posts_route(session: Session) -> impl Responder {
    get_posts(session)
}

#[post("/posts")]
pub async fn create_post_route(session: Session, post: web::Json<CreateArgs>) -> impl Responder {
    create_post(session, post)
}

#[delete("/posts/{id}")]
pub async fn delete_post_route(session: Session, id: web::Path<u64>) -> impl Responder {
    delete_post(session, id)
}

#[patch("/posts/{id}")]
pub async fn update_post_route(
    session: Session,
    id: web::Path<u64>,
    args: web::Json<UpdateArgs>,
) -> impl Responder {
    update_post(session, id, args)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_post_route);
    cfg.service(get_posts_route);
    cfg.service(create_post_route);
    cfg.service(delete_post_route);
    cfg.service(update_post_route);
}
