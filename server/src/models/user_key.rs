use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::models::connection;
use crate::schema::{user_keys, user_keys::dsl};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserKey {
    pub id: u64,
    pub user_id: u64,
    pub public_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
#[derive(Insertable, AsChangeset)]
#[table_name = "user_keys"]
pub struct UserKeyDAO {
    pub user_id: u64,
    pub public_key: String,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct UserKeyRepository {
    conn: MysqlConnection,
}

impl UserKeyRepository {
    pub fn new() -> Self {
        Self {
            conn: connection::connect_rdb(),
        }
    }

    pub fn find_by_user_id(&self, user_id: u64) -> Result<UserKey, Error> {
        let user_key: UserKey = dsl::user_keys
            .filter(dsl::user_id.eq(user_id))
            .get_result::<UserKey>(&self.conn)?;
        Ok(user_key)
    }

    pub fn create(&self, user_id: u64, public_key: &str) -> Result<usize, Error> {
        let user_key_to_create = UserKeyDAO {
            user_id,
            public_key: public_key.to_string(),
            updated_at: None,
        };

        let count = diesel::insert_into(dsl::user_keys)
            .values(user_key_to_create)
            .execute(&self.conn)?;

        Ok(count)
    }
}

impl Default for UserKeyRepository {
    fn default() -> Self {
        Self::new()
    }
}
