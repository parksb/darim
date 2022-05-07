use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;

use crate::utils::env_util::{DATABASE_URL, REDIS_URL};

pub type RdbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub type RedisPool = r2d2::Pool<redis::Client>;
pub type RedisConnection = r2d2::PooledConnection<redis::Client>;

/// Get established MySQL connection.
pub fn connect_rdb() -> RdbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(&*DATABASE_URL);
    diesel::r2d2::Pool::builder().build(manager).unwrap()
}

/// Get established redis connection.
pub fn connect_redis() -> RedisPool {
    let client = redis::Client::open(&**REDIS_URL).unwrap();
    r2d2::Pool::builder().build(client).unwrap()
}
