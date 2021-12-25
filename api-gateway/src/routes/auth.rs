use actix_web::cookie::SameSite;
use actix_web::http::Cookie;
use actix_web::{delete, post, web, HttpRequest, Responder};
use http::StatusCode;
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::Client;
use time::Duration;

use crate::models::auth::*;
use crate::models::error::ApiGatewayError;
use crate::utils::env_util::{JWT_ACCESS_SECRET, JWT_COOKIE_KEY, JWT_REFRESH_SECRET};
use crate::utils::http_util;

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

/// Generates JWT refresh and access tokens.
///
/// # Request
///
/// ```text
/// POST /auth/token/login
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
///     "password": "Ir5c7y8dS3"
/// }
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": "HsBZxiAicY",
///     "error": null
/// }
/// ```
#[post("/auth/token")]
pub async fn set_jwt_tokens(args: web::Json<LoginArgs>) -> impl Responder {
    let args: LoginArgs = args.into_inner();
    let response = Client::new()
        .post(&http_util::get_url("/auth/login"))
        .json(&args)
        .send()
        .await;

    if let Ok(response) = response {
        if let Ok(Some(user_session)) =
            http_util::parse_data_from_service_response::<UserSession>(response).await
        {
            let jwt_refresh = encode(
                &Header::default(),
                &Claims::new(user_session.user_id, JwtType::REFRESH),
                &EncodingKey::from_secret(JWT_REFRESH_SECRET.as_ref()),
            );

            let jwt_access = encode(
                &Header::default(),
                &Claims::new(user_session.user_id, JwtType::ACCESS),
                &EncodingKey::from_secret(JWT_ACCESS_SECRET.as_ref()),
            );

            if let Ok(jwt_refresh) = jwt_refresh {
                if let Ok(jwt_access) = jwt_access {
                    let mut response = http_util::get_ok_response::<String>(jwt_access);
                    let _ = response.add_cookie(
                        &Cookie::build(&*JWT_COOKIE_KEY, jwt_refresh)
                            .secure(true)
                            .http_only(true)
                            .same_site(SameSite::None)
                            .finish(),
                    );
                    response
                } else {
                    http_util::get_err_response::<String>(
                        StatusCode::UNAUTHORIZED,
                        &format!("{}", ApiGatewayError::JwtAccessTokenSettingFailure),
                    )
                }
            } else {
                http_util::get_err_response::<String>(
                    StatusCode::UNAUTHORIZED,
                    &format!("{}", ApiGatewayError::JwtRefreshTokenSettingFailure),
                )
            }
        } else {
            http_util::get_err_response::<String>(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("{}", ApiGatewayError::ServiceResponseParsingFailure),
            )
        }
    } else {
        http_util::pass_response::<String>(response).await
    }
}

/// Removes JWT refresh and access tokens.
///
/// # Request
///
/// ```text
/// DELETE /auth/token
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
#[delete("/auth/token")]
pub async fn remove_jwt_tokens(request: HttpRequest) -> impl Responder {
    if Claims::from_cookie_by_refresh(request).is_ok() {
        let mut response = http_util::get_ok_response::<bool>(true);
        let _ = response.add_cookie(
            &Cookie::build(&*JWT_COOKIE_KEY, "deleted")
                .secure(true)
                .http_only(true)
                .same_site(SameSite::None)
                .max_age(Duration::seconds(0))
                .finish(),
        );
        response
    } else {
        http_util::get_err_response::<bool>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("{}", ApiGatewayError::InternalServerError),
        )
    }
}

/// Generates a new access token.
///
/// # Request
///
/// ```text
/// POST /auth/token/access
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": "HsBZxiAicY",
///     "error": null
/// }
/// ```
#[post("/auth/token/access")]
pub async fn set_jwt_access_token(request: HttpRequest) -> impl Responder {
    if let Ok(claims) = Claims::from_cookie_by_refresh(request) {
        if let Ok(jwt_access) = encode(
            &Header::default(),
            &Claims::new(claims.user_id, JwtType::ACCESS),
            &EncodingKey::from_secret(JWT_ACCESS_SECRET.as_ref()),
        ) {
            http_util::get_ok_response::<String>(jwt_access)
        } else {
            http_util::get_err_response::<bool>(
                StatusCode::UNAUTHORIZED,
                &format!("{}", ApiGatewayError::Unauthorized),
            )
        }
    } else {
        http_util::get_err_response::<bool>(
            StatusCode::UNAUTHORIZED,
            &format!("{}", ApiGatewayError::Unauthorized),
        )
    }
}

/// Initializes the auth routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(set_sign_up_token);
    cfg.service(set_password_token);
    cfg.service(set_jwt_tokens);
    cfg.service(remove_jwt_tokens);
    cfg.service(set_jwt_access_token);
}
