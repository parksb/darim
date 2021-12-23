use actix_web::{delete, get, patch, post, web, HttpRequest, Responder};
use http::StatusCode;
use reqwest::Client;

use crate::models::auth::Claims;
use crate::models::error::*;
use crate::models::user::*;
use crate::utils::http_util;

/// Creates a new user
///
/// # Request
///
/// ```text
/// POST /users
/// ```
///
/// ## Parameters
///
/// * user_public_key - A user's public key
/// * token_key - A key for token search
/// * token_pin - A pin for verifying
///
/// ```json
/// {
///     "user_public_key": "d63ee429"
///     "token_key": "71I3Qz9u",
///     "token_pin": "P9d82Jc5"
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
#[post("/users")]
pub async fn create_user(args: web::Json<CreateArgs>) -> impl Responder {
    let response = Client::new()
        .post(&http_util::get_url("/users"))
        .json(&args.into_inner())
        .send()
        .await;

    http_util::pass_response::<bool>(response).await
}

/// Deletes a user
///
/// # Request
///
/// ```text
/// DELETE /users/:id
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
#[delete("/users/{id}")]
pub async fn delete_user(request: HttpRequest, id: web::Path<u64>) -> impl Responder {
    if let Ok(claims) = Claims::from_header_by_access(request) {
        let id_in_path = id.into_inner();
        if id_in_path == claims.user_id {
            let response = Client::new()
                .delete(&http_util::get_url(&format!("/users/{}", id_in_path)))
                .send()
                .await;

            http_util::pass_response::<bool>(response).await
        } else {
            http_util::get_err_response::<bool>(
                StatusCode::UNAUTHORIZED,
                &get_api_error_message(ApiGatewayError::Unauthorized),
            )
        }
    } else {
        http_util::get_err_response::<bool>(
            StatusCode::UNAUTHORIZED,
            &get_api_error_message(ApiGatewayError::Unauthorized),
        )
    }
}

/// Updates a user
///
/// # Request
///
/// ```text
/// PATCH /users/:id
/// ```
///
/// ## Parameters
///
/// * name - A name of the user.
/// * password - A password of the user.
/// * avatar_url - An avatar image url of the user.
///
/// ```json
/// {
///     "name": "park",
///     "password": "Ir5c7y8dS3",
///     "avatar_url": "avatar.jpg"
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
#[patch("/users/{id}")]
pub async fn update_user(
    request: HttpRequest,
    id: web::Path<u64>,
    args: web::Json<UpdateArgs>,
) -> impl Responder {
    if let Ok(claims) = Claims::from_header_by_access(request) {
        let id_in_path = id.into_inner();
        if id_in_path == claims.user_id {
            let response = Client::new()
                .patch(&http_util::get_url(&format!("/users/{}", id_in_path)))
                .json(&args.into_inner())
                .send()
                .await;

            http_util::pass_response::<bool>(response).await
        } else {
            http_util::get_err_response::<bool>(
                StatusCode::UNAUTHORIZED,
                &get_api_error_message(ApiGatewayError::Unauthorized),
            )
        }
    } else {
        http_util::get_err_response::<bool>(
            StatusCode::UNAUTHORIZED,
            &get_api_error_message(ApiGatewayError::Unauthorized),
        )
    }
}

/// Resets the password.
///
/// # Request
///
/// ```text
/// POST /users/password
/// ```
///
/// ## Parameters
///
/// * email - An email of the user.
/// * token_id - A password token ID.
/// * temporary_password - A temporary password.
/// * new_password - A new password.
///
/// ```json
/// {
///     "email": "park@email.com",
///     "token_id": "d63ee429",
///     "temporary_password": "P9d82Jc5",
///     "new_password": "71I3Qz9u"
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
#[post("/users/password")]
pub async fn reset_password(args: web::Json<ResetPasswordArgs>) -> impl Responder {
    let response = Client::new()
        .post(&http_util::get_url("/users/password"))
        .json(&args.into_inner())
        .send()
        .await;

    http_util::pass_response::<bool>(response).await
}

/// Initializes the user routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
    cfg.service(create_user);
    cfg.service(delete_user);
    cfg.service(update_user);
    cfg.service(reset_password);
}
