//! patic server

/// models
pub mod models {
    pub mod db_connection;
    pub mod error;
    pub mod post;
    pub mod user;
}

/// routes
pub mod routes {
    pub mod post;
    pub mod user;
}

/// services
pub mod services {
    pub mod post;
    pub mod user;
}

pub mod schema;

#[macro_use]
extern crate diesel;
