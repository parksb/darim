use mockall::automock;
use redis::Commands;
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::models::connection::RedisConnection;
use crate::models::error::Result;

/// Password token that represents data in redis.
/// The token has temporary password used to reset the password.
#[derive(Serialize, Deserialize)]
pub struct PasswordToken {
    pub id: String,
    pub password: String,
}

/// A core data repository for password token.
pub struct PasswordTokenRepository<'a> {
    redis: &'a mut RedisConnection,
}

#[automock]
pub trait PasswordTokenRepositoryTrait {
    fn new() -> Self;
    fn find(&mut self) -> Result<String>;
    fn delete(&mut self) -> Result<bool>;
    fn save(&mut self, serialized_token: &str) -> Result<bool>;
}

impl<'a> PasswordTokenRepository<'a> {
    /// Creates a new token repository.
    pub fn new(conn: &'a mut RedisConnection) -> Self {
        Self { redis: conn }
    }

    /// Finds a token by key.
    pub fn find(&mut self, user_id: u64) -> Result<String> {
        let token = self.redis.get::<&str, String>(&key(user_id))?;
        Ok(token)
    }

    /// Creates a new token.
    pub fn save(&mut self, user_id: u64, serialized_token: &str) -> Result<bool> {
        let ttl_seconds = Duration::minutes(3).whole_seconds() as usize;
        let key = &key(user_id);

        let _ = self.redis.set::<&str, &str, bool>(key, &serialized_token)?;
        let _ = self.redis.expire::<&str, _>(key, ttl_seconds)?;

        Ok(true)
    }

    /// Deletes a token by key.
    pub fn delete(&mut self, user_id: u64) -> Result<bool> {
        let _ = self.redis.del::<&str, _>(&key(user_id))?;
        Ok(true)
    }
}

fn key(user_id: u64) -> String {
    format!("password_token:{}", user_id)
}
