use diesel::{mysql::MysqlConnection, prelude::*};
use std::env;

/// Get established MySQL connection.
pub fn connect_rdb() -> MysqlConnection {
    dotenv::dotenv().expect("Failed to read .env file");
    let rdb_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    MysqlConnection::establish(&rdb_url).expect("Failed to establish a db connection")
}

/// Get established redis connection.
pub fn connect_redis() -> redis::Connection {
    dotenv::dotenv().expect("Failed to read .env file");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL not found");
    let client = redis::Client::open(redis_url).expect("Failed to connect to redis");
    client
        .get_connection()
        .expect("Failed to get redis connection")
}
