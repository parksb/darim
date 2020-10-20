use actix_session::Session;
use actix_web::{delete, get, patch, post, web, Responder};
use http::StatusCode;
use reqwest::Client;

use crate::models::error::*;
use crate::models::post::*;
use crate::utils::{http_util, session_util};

/// Responds a post written by logged-in user
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
#[get("/posts/{id}")]
pub async fn get_post(session: Session, id: web::Path<u64>) -> impl Responder {
    if let Some(user_session) = session_util::get_session(&session) {
        let response = reqwest::get(&http_util::get_url(&format!(
            "/posts/{}/{}",
            user_session.user_id, id
        )))
        .await;
        http_util::pass_response::<PostDTO>(response).await
    } else {
        http_util::get_err_response::<PostDTO>(
            StatusCode::UNAUTHORIZED,
            &get_api_error_message(ApiGatewayError::Unauthorized),
        )
    }
}

/// Lists posts written by logged-in user
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
#[get("/posts")]
pub async fn get_posts(session: Session) -> impl Responder {
    if let Some(user_session) = session_util::get_session(&session) {
        let response = reqwest::get(&http_util::get_url(&format!(
            "/posts/{}",
            user_session.user_id
        )))
        .await;
        http_util::pass_response::<Vec<PostDTO>>(response).await
    } else {
        http_util::get_err_response::<Vec<PostDTO>>(
            StatusCode::UNAUTHORIZED,
            &get_api_error_message(ApiGatewayError::Unauthorized),
        )
    }
}

/// Creates a new post
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
#[post("/posts")]
pub async fn create_post(session: Session, args: web::Json<CreateArgs>) -> impl Responder {
    if let Some(user_session) = session_util::get_session(&session) {
        let args = {
            let CreateArgs {
                title,
                content,
                date,
            } = args.into_inner();
            ServiceCreateArgs {
                create_args: CreateArgs {
                    title,
                    content,
                    date,
                },
                user_id: user_session.user_id,
            }
        };

        let response = Client::new()
            .post(&http_util::get_url("/posts"))
            .json(&args)
            .send()
            .await;

        http_util::pass_response::<u64>(response).await
    } else {
        http_util::get_err_response::<u64>(
            StatusCode::UNAUTHORIZED,
            &get_api_error_message(ApiGatewayError::Unauthorized),
        )
    }
}

/// Deletes a post
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
#[delete("/posts/{id}")]
pub async fn delete_post(session: Session, id: web::Path<u64>) -> impl Responder {
    if let Some(user_session) = session_util::get_session(&session) {
        let response = Client::new()
            .delete(&http_util::get_url(&format!(
                "/posts/{}/{}",
                user_session.user_id, id
            )))
            .send()
            .await;
        http_util::pass_response::<bool>(response).await
    } else {
        http_util::get_err_response::<u64>(
            StatusCode::UNAUTHORIZED,
            &get_api_error_message(ApiGatewayError::Unauthorized),
        )
    }
}

/// Updates a post
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
#[patch("/posts/{id}")]
pub async fn update_post(
    session: Session,
    id: web::Path<u64>,
    args: web::Json<UpdateArgs>,
) -> impl Responder {
    if let Some(user_session) = session_util::get_session(&session) {
        let args = {
            let UpdateArgs {
                title,
                content,
                date,
            } = args.into_inner();
            ServiceUpdateArgs {
                update_args: UpdateArgs {
                    title,
                    content,
                    date,
                },
                user_id: user_session.user_id,
            }
        };

        let response = Client::new()
            .patch(&http_util::get_url(&format!("/posts/{}", id)))
            .json(&args)
            .send()
            .await;

        http_util::pass_response::<bool>(response).await
    } else {
        http_util::get_err_response::<bool>(
            StatusCode::UNAUTHORIZED,
            &get_api_error_message(ApiGatewayError::Unauthorized),
        )
    }
}

/// Initializes the post routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_post);
    cfg.service(get_posts);
    cfg.service(create_post);
    cfg.service(delete_post);
    cfg.service(update_post);
}
