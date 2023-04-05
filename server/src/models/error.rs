use crate::utils::env_util::{Profile, PROFILE};
use chrono::Utc;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data not found")]
    NotFound,

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

    #[error("invalid token")]
    InvalidToken,

    #[error("expired token")]
    ExpiredToken,

    #[error("mail error")]
    MailError,

    #[error("diesel error")]
    Diesel {
        #[from]
        source: diesel::result::Error,
    },

    #[error("redis error")]
    Redis {
        #[from]
        source: redis::RedisError,
    },

    #[error("reqwest error")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },

    #[error("json web token error")]
    JsonWebToken {
        #[from]
        source: jsonwebtoken::errors::Error,
    },

    #[error("serde json error")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },

    #[error("argon2 error")]
    Argon2 {
        #[from]
        source: argon2::Error,
    },

    #[error("scrypt check error")]
    Scrypt {
        #[from]
        source: scrypt::errors::CheckError,
    },

    #[error("rand error")]
    Rand {
        #[from]
        source: rand::Error,
    },
}

impl Error {
    /// Logs and returns server error passed by parameter.
    pub fn message(&self) -> String {
        if *PROFILE == Profile::DEV {
            match self {
                Error::Diesel { source } => self.print(source),
                Error::SerdeJson { source } => self.print(source),
                Error::Redis { source } => self.print(source),
                Error::Reqwest { source } => self.print(source),
                Error::JsonWebToken { source } => self.print(source),
                Error::Rand { source } => self.print(source),
                _ => println!("[{}] {}", Utc::now(), self),
            };
        }

        format!("{}", self)
    }

    fn print<T: Display>(&self, data: T) {
        println!("[{}] {} <{}>", Utc::now(), self, data);
    }
}

pub type Result<T> = std::result::Result<T, Error>;
