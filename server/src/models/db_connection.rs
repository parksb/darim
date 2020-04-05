use std::env;
use diesel::{mysql::MysqlConnection, prelude::*};

pub fn connect() -> MysqlConnection {
    dotenv::dotenv().expect("Failed to read .env file");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    MysqlConnection::establish(&db_url).expect("Failed to establish a db connection")
}
