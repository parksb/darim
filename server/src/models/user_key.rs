use chrono::NaiveDateTime;
use diesel::prelude::*;
use mockall::automock;
use serde::{Deserialize, Serialize};

use crate::models::connection;
use crate::models::error::{Error, Result};
use crate::schema::{user_keys, user_keys::dsl};

/// User key representing `user_keys` table.
/// One user must have only one public key.
/// This key is known to both client and server.
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserKey {
    pub id: u64,
    pub user_id: u64,
    pub public_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

/// User DAO using between models layer and RDB.
#[derive(Insertable, AsChangeset)]
#[table_name = "user_keys"]
struct UserKeyDAO {
    pub user_id: u64,
    pub public_key: String,
    pub updated_at: Option<NaiveDateTime>,
}

/// A core data repository for user key.
pub struct UserKeyRepository {
    conn: MysqlConnection,
}

#[automock]
pub trait UserKeyRepositoryTrait {
    fn find_by_user_id(&self, user_id: u64) -> Result<UserKey>;
    fn create(&self, user_id: u64, public_key: &str) -> Result<bool>;
}

impl UserKeyRepository {
    /// Creates a new user key repository.
    pub fn new() -> Self {
        Self {
            conn: connection::connect_rdb(),
        }
    }

    /// Finds a user key by user id.
    pub fn find_by_user_id(&self, user_id: u64) -> Result<UserKey> {
        let user_key = dsl::user_keys
            .filter(dsl::user_id.eq(user_id))
            .get_result::<UserKey>(&self.conn)?;

        Ok(user_key)
    }

    /// Creates a new user key.
    pub fn create(&self, user_id: u64, public_key: &str) -> Result<bool> {
        let user_key_to_create = UserKeyDAO {
            user_id,
            public_key: public_key.to_string(),
            updated_at: None,
        };

        let count = diesel::insert_into(dsl::user_keys)
            .values(user_key_to_create)
            .execute(&self.conn)?;

        if count > 0 {
            Ok(true)
        } else {
            Err(Error::QueryExecutionFailure)
        }
    }
}

impl Default for UserKeyRepository {
    fn default() -> Self {
        Self::new()
    }
}
