use std::env;
use mysql::{Pool, PooledConn, Error};

pub fn connect() -> Result<PooledConn, Error> {
  dotenv::dotenv().expect("Failed to read .env file");

  let user = env::var("DB_USR").expect("db user not found");
  let password = env::var("DB_PASSWORD").expect("db password not found");
  let host = env::var("DB_HOST").expect("db host not found");
  let name = env::var("DB_NAME").expect("db name not found");
  let port = env::var("DB_PORT").expect("db port not found");

  let db_url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, name);
  let pool = Pool::new(db_url)?;
  let conn = pool.get_conn()?;

  Ok(conn)
}
