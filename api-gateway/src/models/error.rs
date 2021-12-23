use chrono::Utc;
use serde::Serialize;
use thiserror::Error;

/// Errors using in api gateway.
#[derive(Error, Debug, Serialize)]
pub enum ApiGatewayError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("internal server error")]
    InternalServerError,

    #[error("failed to parse structure from service response")]
    ServiceResponseParsingFailure,

    #[error("failed to set jwt refresh token")]
    JwtRefreshTokenSettingFailure,

    #[error("failed to set jwt access token")]
    JwtAccessTokenSettingFailure,

    #[error("invalid jwt access token")]
    InvalidJwtAccessToken,

    #[error("expired jwt access token")]
    ExpiredJwtAccessToken,
}

/// Logs and returns api gateway error passed by parameter.
pub fn get_api_error_message(error: ApiGatewayError) -> String {
    println!("[{}] {}", Utc::now(), error);
    format!("{}", error)
}
