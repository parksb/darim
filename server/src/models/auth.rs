use mockall::automock;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

use crate::models::connection;
use crate::models::error::{get_service_error, ServiceError};

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

#[automock]
pub trait SignUpTokenRepositoryTrait {
    fn find(&mut self, key: &str) -> Result<String, ServiceError>;
    fn delete(&mut self, key: &str) -> Result<bool, ServiceError>;
    fn save(&mut self, serialized_token: &str) -> Result<bool, ServiceError>;
}

impl SignUpTokenRepository {
    /// Creates a new token repository.
    pub fn new() -> Self {
        Self {
            client: connection::connect_redis(),
        }
    }

    /// Finds a token by key.
    pub fn find(&mut self, key: &str) -> Result<String, ServiceError> {
        match self.client.get::<&str, String>(key) {
            Ok(token) => Ok(token),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Deletes a token by key.
    pub fn delete(&mut self, key: &str) -> Result<bool, ServiceError> {
        match self.client.del::<&str, _>(key) {
            Ok(result) => Ok(result),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Creates a new token and returns key.
    pub fn save(&mut self, serialized_token: &str) -> Result<String, ServiceError> {
        let key: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        let ttl_seconds = 180; // 3 min

        let result: Result<bool, RedisError> =
            self.client.set::<&str, &str, _>(&key, &serialized_token);
        match result {
            Ok(_) => match self.client.expire::<&str, bool>(&key, ttl_seconds) {
                Ok(_) => Ok(key),
                Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
            },
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
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

#[automock]
pub trait PasswordTokenRepositoryTrait {
    fn new(user_id: u64) -> Self;
    fn find(&mut self) -> Result<String, ServiceError>;
    fn delete(&mut self) -> Result<bool, ServiceError>;
    fn save(&mut self, serialized_token: &str) -> Result<bool, ServiceError>;
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
    pub fn find(&mut self) -> Result<String, ServiceError> {
        match self.client.get::<&str, String>(&self.key) {
            Ok(token) => Ok(token),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Creates a new token.
    pub fn save(&mut self, serialized_token: &str) -> Result<bool, ServiceError> {
        let ttl_seconds = 180; // 3 min

        let result: Result<bool, RedisError> = self
            .client
            .set::<&str, &str, _>(&self.key, &serialized_token);
        match result {
            Ok(_) => match self.client.expire::<&str, _>(&self.key, ttl_seconds) {
                Ok(result) => Ok(result),
                Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
            },
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Deletes a token by key.
    pub fn delete(&mut self) -> Result<bool, ServiceError> {
        match self.client.del::<&str, _>(&self.key) {
            Ok(result) => Ok(result),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }
}
