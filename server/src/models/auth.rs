use chrono::{Duration, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mockall::automock;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

use crate::models::connection;
use crate::models::error::{get_service_error, ServiceError};
use crate::utils::env_util::JWT_REFRESH_SECRET;

/// Session containing information of the logged-in user.
#[derive(Serialize, Deserialize)]
pub struct SetJwtRefreshDTO {
    pub user_id: u64,
    pub jwt_refresh: String,
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

#[derive(Serialize, Deserialize)]
pub struct UserSessionDTO {
    pub user_agent: Option<String>,
    pub last_accessed_at: i64,
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

/// JWT Claims
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub user_id: u64,
}

impl Claims {
    pub fn new(user_id: u64) -> Self {
        let current_date_time = Utc::now();
        Self {
            user_id,
            exp: (current_date_time + Duration::days(30)).timestamp(),
            iat: current_date_time.timestamp(),
        }
    }

    pub fn from_token(token: &str) -> Result<Claims, ServiceError> {
        let validation = Validation::default();
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_REFRESH_SECRET.as_ref()),
            &validation,
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(error) => match error.kind() {
                ErrorKind::InvalidToken => Err(ServiceError::InvalidToken),
                ErrorKind::ExpiredSignature => Err(ServiceError::ExpiredToken),
                _ => Err(ServiceError::InternalServerError),
            },
        }
    }
}

/// A core data repository for refresh token.
pub struct RefreshTokenRepository {
    client: redis::Connection,
}

#[automock]
pub trait RefreshTokenRepositoryTrait {
    fn is_exist(&mut self, user_id: u64, token: &str) -> Result<bool, ServiceError>;
    fn find_all_by_user_id(&mut self, user_id: u64) -> Result<Vec<UserSessionDTO>, ServiceError>;
    fn delete(&mut self, user_id: u64, token: &str) -> Result<bool, ServiceError>;
    fn save(&mut self, user_id: u64, token: &str) -> Result<bool, ServiceError>;
}

impl RefreshTokenRepository {
    /// Creates a new token repository.
    pub fn new() -> Self {
        Self {
            client: connection::connect_redis(),
        }
    }

    /// Check a token is exist by user id and token.
    pub fn is_exist(&mut self, user_id: u64, token: &str) -> Result<bool, ServiceError> {
        match self.client.hexists::<u64, &str, _>(user_id, &token) {
            Ok(is_exist) => Ok(is_exist),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Finds a token by user id.
    pub fn find_all_by_user_id(
        &mut self,
        user_id: u64,
    ) -> Result<Vec<UserSessionDTO>, ServiceError> {
        match self.client.hgetall::<u64, Vec<String>>(user_id) {
            Ok(value) => Ok(value
                .iter()
                .map(|value| serde_json::from_str(value))
                .filter_map(|value| value.ok())
                .collect()),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Deletes a token by key.
    pub fn delete(&mut self, user_id: u64, token: &str) -> Result<bool, ServiceError> {
        match self.client.hdel::<u64, &str, _>(user_id, &token) {
            Ok(result) => Ok(result),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Creates a new token and returns key.
    pub fn save(
        &mut self,
        user_id: u64,
        token: &str,
        user_session: &UserSessionDTO,
    ) -> Result<String, ServiceError> {
        let value = serde_json::to_string(user_session).unwrap();
        let ttl_seconds = 2592000; // 30 days

        let result: Result<bool, RedisError> = self
            .client
            .hset::<u64, &str, &str, _>(user_id, &token, &value);
        match result {
            Ok(_) => match self.client.expire::<u64, bool>(user_id, ttl_seconds) {
                Ok(_) => Ok(user_id.to_string()),
                Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
            },
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }
}

impl Default for RefreshTokenRepository {
    fn default() -> Self {
        Self::new()
    }
}
