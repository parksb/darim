use chrono::Utc;
use diesel::result;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("data store disconnected")]
    DataStoreDisconnect(#[from] result::Error),
}

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
}

pub fn get_service_error(error: ServiceError) -> ServiceError {
    println!("[{}] {}", Utc::now(), ServiceError::QueryExecutionFailure);
    error
}
