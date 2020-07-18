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

/// Arguments for `POST /auth/token/password` API.
#[derive(Serialize, Deserialize)]
pub struct SetPasswordTokenArgs {
    pub email: String,
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
pub struct SignUpTokenRepository {
    client: redis::Connection,
}

impl SignUpTokenRepository {
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

impl Default for SignUpTokenRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// Password token that represents data in redis.
/// The token has temporary password used to reset the password.
#[derive(Serialize, Deserialize)]
pub struct PasswordToken {
    pub id: String,
    pub password: String,
}

/// A core data repository for password token.
pub struct PasswordTokenRepository {
    key: String,
    client: redis::Connection,
}

impl PasswordTokenRepository {
    /// Creates a new token repository.
    pub fn new(user_id: u64) -> Self {
        Self {
            key: format!("password_token:{}", user_id),
            client: connection::connect_redis(),
        }
    }

    /// Finds a token by key.
    pub fn find(&mut self) -> Result<String, RedisError> {
        self.client.get::<&str, String>(&self.key)
    }

    /// Deletes a token by key.
    pub fn delete(&mut self) -> Result<bool, RedisError> {
        self.client.del::<&str, _>(&self.key)
    }

    /// Creates a new token.
    pub fn save(&mut self, serialized_token: &str) -> Result<bool, RedisError> {
        let ttl_seconds = 180; // 3 min

        let _ = self.client.set::<&str, &str, _>(&self.key, &serialized_token)?;
        let _ = self.client.expire::<&str, _>(&self.key, ttl_seconds)?;

        Ok(true)
    }
}
