use actix_web::{HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use http::header::AUTHORIZATION;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::models::error::ApiGatewayError;
use crate::utils::env_util::{JWT_ACCESS_SECRET, JWT_COOKIE_KEY, JWT_REFRESH_SECRET};

/// Arguments for `GET /auth` API.
#[derive(Serialize, Deserialize)]
pub struct LoginArgs {
    pub email: String,
    pub password: String,
}

/// Arguments for `POST /auth/token` API.
#[derive(Serialize, Deserialize)]
pub struct SetSignUpTokenArgs {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

/// Arguments for `POST /auth/token/password` API.
#[derive(Serialize, Deserialize)]
pub struct SetPasswordTokenArgs {
    pub email: String,
}

/// Arguments for `POST /auth/token/refresh/:id` API.
#[derive(Serialize, Deserialize)]
pub struct ValidateJwtRefreshArgs {
    pub jwt_refresh: String,
}

/// Arguments for `DELETE /auth/token/refresh/:id` API.
#[derive(Serialize, Deserialize)]
pub struct RemoveJwtRefreshArgs {
    pub jwt_refresh: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetJwtRefreshDTO {
    pub user_id: u64,
    pub jwt_refresh: String,
}

pub enum JwtType {
    ACCESS,
    REFRESH,
}

/// JWT Claims
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub user_id: u64,
}

impl Claims {
    pub fn new(user_id: u64, token_type: JwtType) -> Self {
        let current_date_time = Utc::now();
        let exp = match token_type {
            JwtType::ACCESS => Duration::minutes(30),
            JwtType::REFRESH => Duration::days(30),
        };

        Self {
            user_id,
            exp: (current_date_time + exp).timestamp(),
            iat: current_date_time.timestamp(),
        }
    }

    pub fn from_header_by_access(request: HttpRequest) -> Result<Claims, ApiGatewayError> {
        const BEARER_STRING: &str = "bearer";
        if let Some(authorization_value) = request.headers().get(AUTHORIZATION) {
            if let Ok(authorization_value) = authorization_value.to_str() {
                if authorization_value
                    .to_lowercase()
                    .starts_with(BEARER_STRING)
                {
                    let token =
                        authorization_value[BEARER_STRING.len()..authorization_value.len()].trim();
                    Self::from_token(token, JwtType::ACCESS)
                } else {
                    Err(ApiGatewayError::Unauthorized)
                }
            } else {
                Err(ApiGatewayError::Unauthorized)
            }
        } else {
            Err(ApiGatewayError::Unauthorized)
        }
    }

    pub fn from_cookie_by_refresh(request: &HttpRequest) -> Result<Claims, ApiGatewayError> {
        if let Some(cookie) = request.cookie(&JWT_COOKIE_KEY) {
            Self::from_token(cookie.value(), JwtType::REFRESH)
        } else {
            Err(ApiGatewayError::Unauthorized)
        }
    }

    fn from_token(token: &str, token_type: JwtType) -> Result<Claims, ApiGatewayError> {
        let validation = Validation::default();
        let secret = match token_type {
            JwtType::REFRESH => JWT_REFRESH_SECRET.as_ref(),
            JwtType::ACCESS => JWT_ACCESS_SECRET.as_ref(),
        };

        match decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(error) => match error.kind() {
                ErrorKind::InvalidToken => Err(ApiGatewayError::InvalidJwtAccessToken),
                ErrorKind::ExpiredSignature => Err(ApiGatewayError::ExpiredJwtAccessToken),
                _ => Err(ApiGatewayError::InternalServerError),
            },
        }
    }
}
