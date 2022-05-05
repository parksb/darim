use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::utils::env_util::{DATABASE_URL, REDIS_URL};

pub type RdbPool = Pool<ConnectionManager<MysqlConnection>>;

/// Get established MySQL connection.
pub fn connect_rdb() -> RdbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(&*DATABASE_URL);
    Pool::builder().build(manager).unwrap()
}

/// Get established redis connection.
pub fn connect_redis() -> redis::Connection {
    redis::Client::open(&**REDIS_URL)
        .expect("Failed to connect to redis")
        .get_connection()
        .expect("Failed to get redis connection")
}
