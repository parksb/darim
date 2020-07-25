//! [![Server CI](https://github.com/ParkSB/darim/workflows/Server%20CI/badge.svg)](https://github.com/ParkSB/darim/actions?query=workflow%3A%22Server+CI%22)
//!
//! ```text
//! +-------------------+--------------------+
//! |  Server (main.rs)                      |
//! +-------------------+--------------------+
//!                     |
//! +-------------------+--------------------+
//! |  Routes                                |
//! +--------+--------+--------+-------------+
//! |  auth  |  post  |  user  |     ...     |
//! +----+---+----+---+----+---+------+------+
//!      |        |        |          |
//! +----+--------+--------+----------+------+
//! |  Services                              |
//! +-------------------+--------------------+
//!                     |           
//! +-------------------+--------------------+
//! |  Models                                |
//! +--------+--------+--------+-------------+
//! |  auth  |  post  |  user  |     ...     |
//! +----+---+----+---+----+---+------+------+
//!      |        |        |          |
//! +----+--------+--------+----------+------+
//! |  Database                              |
//! +----------------------------------------+
//! ```
//!
//! * `main.rs` - An entry point of the application. It runs a http server.
//! * Routes - A presentation layer that makes API public and passes request/response data to other layers.
//! * Services - A business layer that processes the transaction.
//! * Models - A data layer that can access the database and define data structures.

/// A data layer that can access the database and define data structures.
pub mod models {
    /// Model related to authentication.
    pub mod auth;
    /// Model related to Database connection.
    pub mod connection;
    /// Model related to error.
    pub mod error;
    /// Model related to post.
    pub mod post;
    /// Model related to user.
    pub mod user;
    /// Model related to user key.
    pub mod user_key;
}

/// A presentation layer that makes API public and passes request/response data to other layers.
pub mod routes {
    /// API related to authentication.
    pub mod auth;
    /// API related to post.
    pub mod post;
    /// API related to user.
    pub mod user;
}

/// A business layer that processes the transaction.
pub mod services {
    /// Service related to authentication.
    pub mod auth;
    /// Service related to post.
    pub mod post;
    /// Service related to user.
    pub mod user;
}

/// Reusable functions for multiple modules.
pub mod utils {
    /// Utilities related to HTTP.
    pub mod http_util;
    /// Utilities related to password.
    pub mod password_util;
    /// Utilities related to session.
    pub mod session_util;
}

/// A database schema.
pub mod schema;

#[macro_use]
extern crate diesel;
