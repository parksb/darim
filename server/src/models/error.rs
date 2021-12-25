use chrono::Utc;
use diesel::result;
use serde::Serialize;
use thiserror::Error;

/// Errors using in model layer.
#[derive(Error, Debug)]
pub enum ModelError {
    #[error("data store disconnected")]
    DataStoreDisconnect(#[from] result::Error),
}

/// Errors using in service layer.
#[derive(Error, Debug, Serialize)]
pub enum ServiceError {
    #[error("data for key `{0}` not found")]
    NotFound(String),

    #[error("invalid argument supplied")]
    InvalidArgument,

    #[error("invalid format")]
    InvalidFormat,

    #[error("duplicated key")]
    DuplicatedKey,

    #[error("query execution failure")]
    QueryExecutionFailure,

    #[error("unauthorized")]
    Unauthorized,

    #[error("internal server error")]
    InternalServerError,

    #[error("user for id `{0}` not found")]
    UserNotFound(String),

    #[error("failed to send email to `{0}`")]
    EmailFailure(String),

    #[error("invalid token")]
    InvalidToken,

    #[error("expired token")]
    ExpiredToken,
}

/// Logs and returns service error passed by parameter.
pub fn get_service_error(error: ServiceError) -> ServiceError {
    println!("[{}] {}", Utc::now(), error);
    error
}
