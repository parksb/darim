use chrono::Utc;
use http::StatusCode;
use thiserror::Error;

use crate::utils::env_util::{Profile, PROFILE};

/// Errors using in api gateway.
#[derive(Error, Debug)]
pub enum Error {
    #[error("unauthorized")]
    Unauthorized,

    #[error("internal server error")]
    InternalServerError,

    #[error("failed to parse structure from service response")]
    ServiceResponseParsingFailure { reason: String, to: String },

    #[error("failed to set jwt refresh token")]
    JwtRefreshTokenSettingFailure,

    #[error("failed to set jwt access token")]
    JwtAccessTokenSettingFailure,

    #[error("invalid jwt access token")]
    InvalidJwtAccessToken,

    #[error("expired jwt access token")]
    ExpiredJwtAccessToken,

    #[error("failed to handle internal http request")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },

    #[error("failed to ")]
    JsonWebToken {
        #[from]
        source: jsonwebtoken::errors::Error,
    },
}

impl Error {
    /// Logs and returns api gateway error passed by parameter.
    pub fn message(&self) -> String {
        if *PROFILE == Profile::DEV {
            match &self {
                Error::Reqwest { source } => {
                    println!("[{}] {} <{}>", Utc::now(), &self, source)
                }
                Error::JsonWebToken { source } => {
                    println!("[{}] {} <{}>", Utc::now(), &self, source)
                }
                Error::ServiceResponseParsingFailure { reason, to } => {
                    println!(
                        "[{}] {} <reason: `{}` to: `{}`>",
                        Utc::now(),
                        &self,
                        reason,
                        to
                    )
                }
                _ => println!("[{}] {}", Utc::now(), &self),
            };
        }

        format!("{}", &self)
    }

    pub fn to_http_status(&self) -> StatusCode {
        match self {
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ServiceResponseParsingFailure { reason: _, to: _ } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
