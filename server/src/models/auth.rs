use rand::{distributions::Alphanumeric, thread_rng, Rng};
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

use crate::models::connection;

/// Arguments for `GET /auth` API.
#[derive(Serialize, Deserialize)]
pub struct LoginArgs {
    pub email: String,
    pub password: String,
}

/// Arguments for `POST /auth/token` API.
#[derive(Serialize, Deserialize)]
pub struct SetSignUpTokenArgs {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

/// Session containing information of the logged-in user.
#[derive(Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: u64,
    pub user_email: String,
    pub user_name: String,
    pub user_public_key: String,
    pub user_avatar_url: Option<String>,
}

/// Token representing data in redis.
/// Token containing information of the user in progress to sign up.
/// It can be referenced by unique `pin` as key.
#[derive(Serialize, Deserialize)]
pub struct Token {
    pub pin: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

/// A core data repository for token.
pub struct TokenRepository {
    client: redis::Connection,
}

impl TokenRepository {
    /// Creates a new token repository.
    pub fn new() -> Self {
        Self {
            client: connection::connect_redis(),
        }
    }

    /// Finds a token by key.
    pub fn find(&mut self, key: &str) -> Result<String, RedisError> {
        self.client.get::<&str, String>(key)
    }

    /// Deletes a token by key.
    pub fn delete(&mut self, key: &str) -> Result<bool, RedisError> {
        self.client.del::<&str, _>(key)
    }

    /// Creates a new token.
    pub fn save(&mut self, serialized_token: &str) -> Result<bool, RedisError> {
        let key: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        let ttl_seconds = 180; // 3 min

        let _ = self.client.set::<&str, &str, _>(&key, &serialized_token)?;
        let _ = self.client.expire::<&str, _>(&key, ttl_seconds)?;

        Ok(true)
    }
}

impl Default for TokenRepository {
    fn default() -> Self {
        Self::new()
    }
}
