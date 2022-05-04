use mockall::automock;
use redis::Commands;
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::models::connection;
use crate::models::error::Result;

/// Password token that represents data in redis.
/// The token has temporary password used to reset the password.
#[derive(Serialize, Deserialize)]
pub struct PasswordToken {
    pub id: String,
    pub password: String,
}

/// A core data repository for password token.
pub struct PasswordTokenRepository {
    redis: redis::Connection,
}

#[automock]
pub trait PasswordTokenRepositoryTrait {
    fn new() -> Self;
    fn find(&mut self) -> Result<String>;
    fn delete(&mut self) -> Result<bool>;
    fn save(&mut self, serialized_token: &str) -> Result<bool>;
}

impl PasswordTokenRepository {
    /// Creates a new token repository.
    pub fn new() -> Self {
        Self {
            redis: connection::connect_redis(),
        }
    }

    /// Finds a token by key.
    pub fn find(&mut self, user_id: u64) -> Result<String> {
        let token = self.redis.get::<&str, String>(&self.key(user_id))?;
        Ok(token)
    }

    /// Creates a new token.
    pub fn save(&mut self, user_id: u64, serialized_token: &str) -> Result<bool> {
        let ttl_seconds = Duration::minutes(3).whole_seconds() as usize;
        let key = &self.key(user_id);

        let _ = self.redis.set::<&str, &str, bool>(key, &serialized_token)?;
        let _ = self.redis.expire::<&str, _>(key, ttl_seconds)?;

        Ok(true)
    }

    /// Deletes a token by key.
    pub fn delete(&mut self, user_id: u64) -> Result<bool> {
        let _ = self.redis.del::<&str, _>(&self.key(user_id))?;
        Ok(true)
    }

    fn key(&self, user_id: u64) -> String {
        format!("password_token:{}", user_id)
    }
}

impl Default for PasswordTokenRepository {
    fn default() -> Self {
        Self::new()
    }
}
