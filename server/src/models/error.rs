use thiserror::Error;
use diesel::result;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("data store disconnected")]
    DataStoreDisconnect(#[from] result::Error),

    #[error("data for id `{0}` not found")]
    NotFound(u64),

    #[error("query execution failure")]
    QueryExecutionFailure,
}
