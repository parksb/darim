use actix_web::cookie::SameSite;
use actix_web::http::Cookie;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, Responder};
use http::StatusCode;
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::Client;
use time::Duration;

use crate::models::auth::*;
use crate::models::error::ApiGatewayError;
use crate::utils::env_util::{
    Profile, DOMAIN, JWT_ACCESS_SECRET, JWT_COOKIE_KEY, JWT_UUID_COOKIE_KEY, PROFILE,
};
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
pub async fn set_jwt_tokens(request: HttpRequest, args: web::Json<LoginArgs>) -> impl Responder {
    let args = LoginArgs {
        user_agent: http_util::extract_user_agent(&request),
        ..args.into_inner()
    };
    let response = Client::new()
        .post(&http_util::get_url("/auth/token/refresh"))
        .json(&args)
        .send()
        .await;

    if let Ok(response) = response {
        if let Ok(Some(result)) =
            http_util::parse_data_from_service_response::<SetJwtRefreshDTO>(response).await
        {
            let encoded_token = encode(
                &Header::default(),
                &Claims::new(result.user_id, JwtType::ACCESS),
                &EncodingKey::from_secret(JWT_ACCESS_SECRET.as_ref()),
            );

            if let Ok(jwt_access) = encoded_token {
                let mut response = http_util::get_ok_response::<String>(jwt_access);
                let _ = response.add_cookie({
                    let builder = Cookie::build(&*JWT_COOKIE_KEY, result.jwt_refresh)
                        .domain(&*DOMAIN)
                        .http_only(true);
                    &match *PROFILE {
                        Profile::PRODUCTION => builder.secure(true).same_site(SameSite::Lax),
                        Profile::DEV => builder.secure(false).same_site(SameSite::None),
                    }
                    .finish()
                });
                let _ = response.add_cookie({
                    let builder = Cookie::build(&*JWT_UUID_COOKIE_KEY, result.token_uuid)
                        .domain(&*DOMAIN)
                        .http_only(true);
                    &match *PROFILE {
                        Profile::PRODUCTION => builder.secure(true).same_site(SameSite::Lax),
                        Profile::DEV => builder.secure(false).same_site(SameSite::None),
                    }
                    .finish()
                });
                response
            } else {
                http_util::get_err_response::<String>(
                    StatusCode::UNAUTHORIZED,
                    &format!("{}", ApiGatewayError::JwtAccessTokenSettingFailure),
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
/// DELETE /auth/token/me
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
#[delete("/auth/token/me")]
pub async fn remove_jwt_token(request: HttpRequest) -> impl Responder {
    if let Ok(claims) = Claims::from_cookie_by_refresh(&request) {
        let jwt_uuid_cookie = request.cookie(&*JWT_UUID_COOKIE_KEY).unwrap();
        let response = Client::new()
            .delete(&http_util::get_url(&format!(
                "/auth/token/refresh/{}",
                claims.user_id
            )))
            .json(&RemoveJwtRefreshArgs {
                token_uuid: jwt_uuid_cookie.value().to_string(),
            })
            .send()
            .await;

        let mut response = http_util::pass_response::<bool>(response).await;
        let _ = response.add_cookie({
            let builder = Cookie::build(&*JWT_COOKIE_KEY, "deleted")
                .domain(&*DOMAIN)
                .http_only(true)
                .max_age(Duration::seconds(0));
            &match *PROFILE {
                Profile::PRODUCTION => builder.secure(true).same_site(SameSite::Lax),
                Profile::DEV => builder.secure(false).same_site(SameSite::None),
            }
            .finish()
        });

        let _ = response.add_cookie({
            let builder = Cookie::build(&*JWT_UUID_COOKIE_KEY, "deleted")
                .domain(&*DOMAIN)
                .http_only(true)
                .max_age(Duration::seconds(0));
            &match *PROFILE {
                Profile::PRODUCTION => builder.secure(true).same_site(SameSite::Lax),
                Profile::DEV => builder.secure(false).same_site(SameSite::None),
            }
            .finish()
        });

        response
    } else {
        http_util::get_err_response::<bool>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("{}", ApiGatewayError::Unauthorized),
        )
    }
}

/// Removes a JWT refresh token.
///
/// # Request
///
/// ```text
/// DELETE /auth/token/:uuid
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
#[delete("/auth/token/{uuid}")]
pub async fn remove_jwt_token_by_uuid(
    uuid: web::Path<String>,
    request: HttpRequest,
) -> impl Responder {
    if let Ok(claims) = Claims::from_cookie_by_refresh(&request) {
        let response = Client::new()
            .delete(&http_util::get_url(&format!(
                "/auth/token/refresh/{}",
                claims.user_id,
            )))
            .json(&RemoveJwtRefreshArgs {
                token_uuid: uuid.into_inner(),
            })
            .send()
            .await;

        http_util::pass_response::<bool>(response).await
    } else {
        http_util::get_err_response::<bool>(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("{}", ApiGatewayError::Unauthorized),
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
    if let Ok(claims) = Claims::from_cookie_by_refresh(&request) {
        let jwt_cookie = request.cookie(&*JWT_COOKIE_KEY).unwrap();
        let jwt_uuid_cookie = request.cookie(&*JWT_UUID_COOKIE_KEY).unwrap();

        let response = Client::new()
            .post(&http_util::get_url(&format!(
                "/auth/token/refresh/{}",
                claims.user_id,
            )))
            .json(&ValidateJwtRefreshArgs {
                token_uuid: jwt_uuid_cookie.value().to_string(),
                jwt_refresh: jwt_cookie.value().to_string(),
                user_agent: http_util::extract_user_agent(&request),
            })
            .send()
            .await;

        // FIXME: Resolve `if let` hell.
        if let Ok(response) = response {
            if let Ok(Some(is_valid_token)) =
                http_util::parse_data_from_service_response::<bool>(response).await
            {
                if is_valid_token {
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
            } else {
                http_util::get_err_response::<bool>(
                    StatusCode::UNAUTHORIZED,
                    &format!("{}", ApiGatewayError::InternalServerError),
                )
            }
        } else {
            http_util::get_err_response::<bool>(
                StatusCode::UNAUTHORIZED,
                &format!("{}", ApiGatewayError::InternalServerError),
            )
        }
    } else {
        http_util::get_err_response::<bool>(
            StatusCode::UNAUTHORIZED,
            &format!("{}", ApiGatewayError::InternalServerError),
        )
    }
}

/// Get active tokens as session.
///
/// # Request
///
/// ```text
/// GET /auth/token
/// ```
///
/// # Response
///
/// ```json
/// {
///     "data": [
///       {
///         "user_agent": "Mozilla/5.0",
///         "last_accessed_at": 1640425102089
///       }
///     ],
///     "error": null
/// }
/// ```
#[get("/auth/token")]
pub async fn get_session_list(request: HttpRequest) -> impl Responder {
    if let Ok(claims) = Claims::from_header_by_access(request) {
        let response = reqwest::get(&http_util::get_url(&format!(
            "/auth/token/{}",
            claims.user_id
        )))
        .await;
        http_util::pass_response::<Vec<UserSessionDTO>>(response).await
    } else {
        http_util::get_err_response::<Vec<UserSessionDTO>>(
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
    cfg.service(remove_jwt_token);
    cfg.service(remove_jwt_token_by_uuid);
    cfg.service(set_jwt_access_token);
    cfg.service(get_session_list);
}
