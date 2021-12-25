use diesel::{mysql::MysqlConnection, prelude::*};

use crate::utils::env_util::{DATABASE_URL, REDIS_URL};

/// Get established MySQL connection.
pub fn connect_rdb() -> MysqlConnection {
    MysqlConnection::establish(&*DATABASE_URL).expect("Failed to establish a db connection")
}

/// Get established redis connection.
pub fn connect_redis() -> redis::Connection {
    redis::Client::open(&**REDIS_URL)
        .expect("Failed to connect to redis")
        .get_connection()
        .expect("Failed to get redis connection")
}
