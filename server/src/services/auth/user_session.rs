use serde::{Deserialize, Serialize};

use crate::models::auth::jwt_refresh::RefreshTokenRepository;
use crate::models::connection::RedisConnection;
use crate::models::error::Result;

#[derive(Serialize, Deserialize)]
pub struct UserSessionDTO {
    pub token_uuid: String,
    pub user_agent: Option<String>,
    pub last_accessed_at: i64,
}

pub struct UserSessionService<'a> {
    refresh_token_repository: RefreshTokenRepository<'a>,
}

impl<'a> UserSessionService<'a> {
    pub fn new(conn: &'a mut RedisConnection) -> Self {
        Self {
            refresh_token_repository: RefreshTokenRepository::new(conn),
        }
    }

    pub fn get_all(&mut self, user_id: u64) -> Result<Vec<UserSessionDTO>> {
        Ok(self
            .refresh_token_repository
            .find_all_by_user_id(user_id)?
            .into_iter()
            .map(|(uuid, session)| UserSessionDTO {
                token_uuid: uuid,
                user_agent: session.user_agent,
                last_accessed_at: session.last_accessed_at,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl UserSessionService {
        pub fn new_with_repository(refresh_token_repository: RefreshTokenRepository) -> Self {
            Self {
                refresh_token_repository,
            }
        }
    }
}
