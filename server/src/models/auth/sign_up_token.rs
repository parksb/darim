use mockall::automock;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use redis::Commands;
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::models::connection::RedisConnection;
use crate::models::error::Result;

/// Sign up token that represents data in redis.
/// The token has information of the user used for sign up.
/// It can be referenced by unique `pin` as key.
#[derive(Serialize, Deserialize)]
pub struct SignUpToken {
    pub pin: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

/// A core data repository for token.
pub struct SignUpTokenRepository<'a> {
    redis: &'a mut RedisConnection,
}

#[automock]
pub trait SignUpTokenRepositoryTrait {
    fn find(&mut self, key: &str) -> Result<String>;
    fn delete(&mut self, key: &str) -> Result<bool>;
    fn save(&mut self, serialized_token: &str) -> Result<bool>;
}

impl<'a> SignUpTokenRepository<'a> {
    /// Creates a new token repository.
    pub fn new(conn: &'a mut RedisConnection) -> Self {
        Self { redis: conn }
    }

    /// Finds a token by key.
    pub fn find(&mut self, key: &str) -> Result<String> {
        let token = self.redis.get::<&str, String>(key)?;
        Ok(token)
    }

    /// Deletes a token by key.
    pub fn delete(&mut self, key: &str) -> Result<bool> {
        let _ = self.redis.del::<&str, _>(key)?;
        Ok(true)
    }

    /// Creates a new token and returns key.
    pub fn save(&mut self, serialized_token: &str) -> Result<String> {
        let key: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        let ttl_seconds = Duration::minutes(3).whole_seconds() as usize;

        let _ = self
            .redis
            .set::<&str, &str, bool>(&key, &serialized_token)?;
        let _ = self.redis.expire::<&str, bool>(&key, ttl_seconds)?;

        Ok(key)
    }
}
