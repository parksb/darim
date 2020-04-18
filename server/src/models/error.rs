use diesel::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("data store disconnected")]
    DataStoreDisconnect(#[from] result::Error),

    #[error("data for id `{0}` not found")]
    NotFound(u64),

    #[error("invalid argument supplied")]
    InvalidArgument,

    #[error("duplicated key")]
    DuplicatedKey,

    #[error("query execution failure")]
    QueryExecutionFailure,
}
