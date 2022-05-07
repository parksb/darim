use mockall::automock;
use redis::Commands;
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::models::connection::RedisConnection;
use crate::models::error::Result;

#[derive(Serialize, Deserialize)]
pub struct UserSession {
    pub jwt_refresh: String,
    pub user_agent: Option<String>,
    pub last_accessed_at: i64,
}

/// A core data repository for refresh token.
pub struct RefreshTokenRepository<'a> {
    redis: &'a mut RedisConnection,
}

#[automock]
pub trait RefreshTokenRepositoryTrait {
    fn is_exist(&self, user_id: u64, token: &str) -> Result<bool>;
    fn find_all_by_user_id(&self, user_id: u64) -> Result<Vec<(String, UserSession)>>;
    fn delete(&self, user_id: u64, token: &str) -> Result<bool>;
    fn save(&self, user_id: u64, token: &str) -> Result<bool>;
}

impl<'a> RefreshTokenRepository<'a> {
    /// Creates a new token repository.
    pub fn new(conn: &'a mut RedisConnection) -> Self {
        Self { redis: conn }
    }

    /// Check a token is exist by user id and token.
    pub fn is_exist(&mut self, user_id: u64, uuid: &str, token: &str) -> Result<bool> {
        let session_json = self.redis.hget::<u64, &str, String>(user_id, uuid)?;
        let session = serde_json::from_str::<UserSession>(&session_json)?;
        Ok(session.jwt_refresh == token)
    }

    /// Finds a token by user id.
    pub fn find_all_by_user_id(&mut self, user_id: u64) -> Result<Vec<(String, UserSession)>> {
        let sessions_json = self.redis.hgetall::<u64, Vec<(String, String)>>(user_id)?;
        Ok(sessions_json
            .into_iter()
            .map(|(uuid, session_json)| {
                let session = serde_json::from_str::<UserSession>(&session_json)?;
                Ok((uuid, session))
            })
            .filter_map(|value: Result<(String, UserSession)>| value.ok())
            .collect())
    }

    /// Deletes a token by key.
    pub fn delete(&mut self, user_id: u64, uuid: &str) -> Result<bool> {
        let _ = self.redis.hdel::<u64, &str, _>(user_id, uuid)?;
        Ok(true)
    }

    /// Creates a new token and returns key.
    pub fn save(&mut self, user_id: u64, uuid: &str, user_session: &UserSession) -> Result<String> {
        let value = serde_json::to_string(user_session)?;
        let ttl_seconds = Duration::days(30).whole_seconds() as usize;

        let _ = self
            .redis
            .hset::<u64, &str, &str, bool>(user_id, uuid, &value)?;
        let _ = self.redis.expire::<u64, bool>(user_id, ttl_seconds)?;

        Ok(user_id.to_string())
    }
}
