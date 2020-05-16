use diesel::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("data store disconnected")]
    DataStoreDisconnect(#[from] result::Error),

    #[error("data for key `{0}` not found")]
    NotFound(String),

    #[error("invalid argument supplied")]
    InvalidArgument,

    #[error("duplicated key")]
    DuplicatedKey,

    #[error("query execution failure")]
    QueryExecutionFailure,

    #[error("unauthorized")]
    Unauthorized,
}
