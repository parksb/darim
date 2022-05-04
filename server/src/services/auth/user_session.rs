use serde::{Deserialize, Serialize};

use crate::models::auth::jwt_refresh::RefreshTokenRepository;
use crate::models::error::Result;

#[derive(Serialize, Deserialize)]
pub struct UserSessionDTO {
    pub token_uuid: String,
    pub user_agent: Option<String>,
    pub last_accessed_at: i64,
}

pub struct UserSessionService {
    refresh_token_repository: RefreshTokenRepository,
}

impl UserSessionService {
    pub fn new() -> Self {
        Self {
            refresh_token_repository: RefreshTokenRepository::new(),
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

impl Default for UserSessionService {
    fn default() -> Self {
        Self::new()
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
