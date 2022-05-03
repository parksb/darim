use actix_web::cookie::{CookieBuilder, SameSite};
use actix_web::http::Cookie;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::Client;
use time::Duration;

use crate::models::auth::*;
use crate::models::error::Error;
use crate::utils::env_util::{
    Profile, JWT_ACCESS_SECRET, JWT_COOKIE_KEY, JWT_UUID_COOKIE_KEY, PROFILE,
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

    async fn resolve(args: LoginArgs) -> Result<(String, String, String), Error> {
        let response = Client::new()
            .post(&http_util::get_url("/auth/token/refresh"))
            .json(&args)
            .send()
            .await?;

        let result = http_util::parse_data_from_service_response::<SetJwtRefreshDTO>(response)
            .await?
            .ok_or(Error::Unauthorized)?;

        let jwt_access = encode(
            &Header::default(),
            &Claims::new(result.user_id, JwtType::ACCESS),
            &EncodingKey::from_secret(JWT_ACCESS_SECRET.as_ref()),
        )?;

        Ok((jwt_access, result.jwt_refresh, result.token_uuid))
    }

    match resolve(args).await {
        Ok((jwt_access, jwt_refresh, token_uuid)) => {
            let mut response = http_util::get_ok_response::<String>(jwt_access);
            let _ = response.add_cookie(&cookie(&*JWT_COOKIE_KEY, &jwt_refresh).finish());
            let _ = response.add_cookie(&cookie(&*JWT_UUID_COOKIE_KEY, &token_uuid).finish());
            response
        }
        Err(res) => http_util::get_err_response::<String>((&res).to_http_status(), &res.message()),
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
///     "error": null }
/// ```
#[delete("/auth/token/me")]
pub async fn remove_jwt_token(request: HttpRequest) -> impl Responder {
    async fn resolve(request: HttpRequest) -> Result<bool, Error> {
        let claims = Claims::from_cookie_by_refresh(&request)?;
        let jwt_uuid_cookie = request
            .cookie(&*JWT_UUID_COOKIE_KEY)
            .ok_or(Error::Unauthorized)?;
        let response = Client::new()
            .delete(&http_util::get_url(&format!(
                "/auth/token/refresh/{}",
                claims.user_id
            )))
            .json(&RemoveJwtRefreshArgs {
                token_uuid: jwt_uuid_cookie.value().to_string(),
            })
            .send()
            .await?;

        let result = http_util::parse_data_from_service_response::<bool>(response)
            .await?
            .ok_or(Error::Unauthorized)?;

        Ok(result)
    }

    match resolve(request).await {
        Ok(res) => {
            let mut response = http_util::get_ok_response::<bool>(res);
            let _ = response.add_cookie(
                &cookie(&*JWT_COOKIE_KEY, "deleted")
                    .max_age(Duration::seconds(0))
                    .finish(),
            );
            let _ = response.add_cookie(
                &cookie(&*JWT_UUID_COOKIE_KEY, "deleted")
                    .max_age(Duration::seconds(0))
                    .finish(),
            );
            response
        }
        Err(res) => http_util::get_err_response::<String>((&res).to_http_status(), &res.message()),
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
    async fn resolve(uuid: &str, request: HttpRequest) -> Result<bool, Error> {
        let claims = Claims::from_cookie_by_refresh(&request)?;
        let response = Client::new()
            .delete(&http_util::get_url(&format!(
                "/auth/token/refresh/{}",
                claims.user_id,
            )))
            .json(&RemoveJwtRefreshArgs {
                token_uuid: uuid.to_string(),
            })
            .send()
            .await?;

        let result = http_util::parse_data_from_service_response::<bool>(response)
            .await?
            .ok_or(Error::Unauthorized)?;

        Ok(result)
    }

    match resolve(&uuid.into_inner(), request).await {
        Ok(res) => http_util::get_ok_response::<bool>(res),
        Err(res) => http_util::get_err_response::<String>((&res).to_http_status(), &res.message()),
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
    async fn resolve(request: HttpRequest) -> Result<String, Error> {
        let claims = Claims::from_cookie_by_refresh(&request)?;
        let jwt_cookie = request
            .cookie(&*JWT_COOKIE_KEY)
            .ok_or(Error::Unauthorized)?;
        let jwt_uuid_cookie = request
            .cookie(&*JWT_UUID_COOKIE_KEY)
            .ok_or(Error::Unauthorized)?;

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
            .await?;

        if http_util::parse_data_from_service_response::<bool>(response)
            .await?
            .unwrap_or(false)
        {
            let access_token = encode(
                &Header::default(),
                &Claims::new(claims.user_id, JwtType::ACCESS),
                &EncodingKey::from_secret(JWT_ACCESS_SECRET.as_ref()),
            )?;
            Ok(access_token)
        } else {
            Err(Error::Unauthorized)
        }
    }

    match resolve(request).await {
        Ok(res) => http_util::get_ok_response(res),
        Err(res) => http_util::get_err_response::<String>((&res).to_http_status(), &res.message()),
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
    async fn resolve(request: HttpRequest) -> Result<Vec<ActiveUserSessionDTO>, Error> {
        let claims = Claims::from_header_by_access(&request)?;
        let jwt_uuid = request
            .cookie(&*JWT_UUID_COOKIE_KEY)
            .ok_or(Error::Unauthorized)?
            .value()
            .to_string();
        let response = reqwest::get(&http_util::get_url(&format!(
            "/auth/token/{}",
            claims.user_id
        )))
        .await?;

        Ok(
            http_util::parse_data_from_service_response::<Vec<UserSessionDTO>>(response)
                .await?
                .unwrap_or_default()
                .into_iter()
                .map(|session| -> ActiveUserSessionDTO {
                    ActiveUserSessionDTO {
                        is_mine: jwt_uuid == session.token_uuid,
                        token_uuid: session.token_uuid,
                        user_agent: session.user_agent,
                        last_accessed_at: session.last_accessed_at,
                    }
                })
                .collect(),
        )
    }

    match resolve(request).await {
        Ok(res) => http_util::get_ok_response(res),
        Err(res) => http_util::get_err_response::<String>((&res).to_http_status(), &res.message()),
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

fn cookie<'a>(key: &'a str, value: &'a str) -> CookieBuilder<'a> {
    let builder = Cookie::build(key, value)
        .http_only(true)
        .same_site(SameSite::None);
    match *PROFILE {
        Profile::PRODUCTION => builder.secure(true),
        Profile::DEV => builder.secure(false),
    }
}
