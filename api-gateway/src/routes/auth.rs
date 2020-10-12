use actix_session::Session;
use actix_web::{get, post, web, Responder};
use http::StatusCode;
use reqwest::Client;

use crate::models::auth::*;
use crate::models::error::{get_api_error_message, ApiGatewayError};
use crate::models::user::UserDTO;
use crate::utils::{http_util, session_util};

/// Responds auth information as user session.
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
///     },
///     "error": null
/// }
/// ```
#[get("/auth")]
pub async fn get_auth(session: Session) -> impl Responder {
    let user_session = session_util::get_session(&session);

    if let Some(user_session) = user_session {
        http_util::get_ok_response::<UserSession>(user_session)
    } else {
        http_util::get_err_response::<UserSession>(
            StatusCode::UNAUTHORIZED,
            &get_api_error_message(ApiGatewayError::Unauthorized),
        )
    }
}

/// Refresh auth information as user session.
///
/// # Request
///
/// ```text
/// POST /auth
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": {
///         "user_id": 0,
//          "user_email": "park@email.com"
///         "user_name": "park",
///         "user_avatar_url": "avatar.jpg"
///     },
///     "error": null
/// }
/// ```
#[post("/auth")]
pub async fn refresh_session(mut session: Session) -> impl Responder {
    let user_session = session_util::get_session(&session);
    if let Some(user_session) = user_session {
        let response = reqwest::get(&http_util::get_url(&format!(
            "/users/{}",
            user_session.user_id
        )))
        .await
        .unwrap();

        let result = http_util::parse_data_from_service_response::<UserDTO>(response).await;
        if let Ok(user) = result {
            if let Some(user) = user {
                session_util::set_session(
                    &mut session,
                    user_session.user_id,
                    &user_session.user_email,
                    &user.name,
                    &user_session.user_public_key,
                    &user.avatar_url,
                );

                if let Some(refreshed_user_session) = session_util::get_session(&session) {
                    http_util::get_ok_response::<UserSession>(refreshed_user_session)
                } else {
                    http_util::get_err_response::<UserSession>(
                        StatusCode::UNAUTHORIZED,
                        &get_api_error_message(ApiGatewayError::Unauthorized),
                    )
                }
            } else {
                http_util::get_err_response::<UserSession>(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &get_api_error_message(ApiGatewayError::ServiceResponseParsingFailure),
                )
            }
        } else {
            http_util::get_err_response::<UserSession>(
                StatusCode::UNAUTHORIZED,
                &get_api_error_message(ApiGatewayError::Unauthorized),
            )
        }
    } else {
        http_util::get_err_response::<UserSession>(
            StatusCode::UNAUTHORIZED,
            &get_api_error_message(ApiGatewayError::Unauthorized),
        )
    }
}

/// Sets token for creating user.
///
/// # Request
///
/// ```text
/// POST /auth/token/sign_up
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
///     "data": "a1lam9cBko",
///     "error": null
/// }
/// ```
#[post("/auth/token/sign_up")]
pub async fn set_sign_up_token(args: web::Json<SetSignUpTokenArgs>) -> impl Responder {
    let args: SetSignUpTokenArgs = args.into_inner();
    let response = Client::new()
        .post(&http_util::get_url("/auth/token/sign_up"))
        .json(&args)
        .send()
        .await;
    http_util::pass_response::<String>(response).await
}

/// Sets token for resetting password.
///
/// # Request
///
/// ```text
/// POST /auth/token/password
/// ```
///
/// ## Parameters
///
/// * email - A unique email of the user.
///
/// ```json
/// {
///     "email": "park@email.com",
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
#[post("/auth/token/password")]
pub async fn set_password_token(args: web::Json<SetPasswordTokenArgs>) -> impl Responder {
    let args: SetPasswordTokenArgs = args.into_inner();
    let response = Client::new()
        .post(&http_util::get_url("/auth/token/password"))
        .json(&args)
        .send()
        .await;
    http_util::pass_response::<bool>(response).await
}

/// Signs in to set user session.
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
///     },
///     "error": null
/// }
/// ```
#[post("/auth/login")]
pub async fn login(mut session: Session, args: web::Json<LoginArgs>) -> impl Responder {
    let args: LoginArgs = args.into_inner();
    let response = Client::new()
        .post(&http_util::get_url("/auth/login"))
        .json(&args)
        .send()
        .await;

    if let Ok(response) = response {
        let user_session =
            http_util::parse_data_from_service_response::<UserSession>(response).await;
        if let Ok(user_session) = user_session {
            if let Some(user_session) = user_session {
                session_util::set_session(
                    &mut session,
                    user_session.user_id,
                    &user_session.user_email,
                    &user_session.user_name,
                    &user_session.user_public_key,
                    &user_session.user_avatar_url,
                );
                http_util::get_ok_response::<UserSession>(user_session)
            } else {
                http_util::get_err_response::<UserSession>(
                    StatusCode::UNAUTHORIZED,
                    &format!("{}", ApiGatewayError::Unauthorized),
                )
            }
        } else {
            http_util::get_err_response::<UserSession>(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("{}", ApiGatewayError::ServiceResponseParsingFailure),
            )
        }
    } else {
        http_util::pass_response::<UserSession>(response).await
    }
}

/// Signs out to unset user session.
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
///     "data": true,
///     "error": null
/// }
/// ```
#[post("/auth/logout")]
pub async fn logout(mut session: Session) -> impl Responder {
    let is_logged_in = session_util::get_session(&session);
    if is_logged_in.is_some() {
        session_util::unset_session(&mut session);
        http_util::get_ok_response::<bool>(true)
    } else {
        http_util::get_err_response::<bool>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("{}", ApiGatewayError::InternalServerError),
        )
    }
}

/// Initializes the auth routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_auth);
    cfg.service(refresh_session);
    cfg.service(set_sign_up_token);
    cfg.service(set_password_token);
    cfg.service(login);
    cfg.service(logout);
}
