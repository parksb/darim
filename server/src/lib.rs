//! patic server

/// models
pub mod models {
    pub mod auth;
    pub mod db_connection;
    pub mod error;
    pub mod post;
    pub mod user;
}

/// routes
pub mod routes {
    pub mod auth;
    pub mod post;
    pub mod user;
}

/// services
pub mod services {
    pub mod auth;
    pub mod post;
    pub mod user;
}

pub mod utils {
    pub mod password_util;
    pub mod session_util;
}

pub mod schema;

#[macro_use]
extern crate diesel;
