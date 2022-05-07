use chrono::Utc;
use diesel::MysqlConnection;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::auth::jwt_claims::Claims;
use crate::models::auth::jwt_refresh::*;
use crate::models::connection::RedisConnection;
use crate::models::error::{Error, Result};
use crate::models::user::UserRepository;
use crate::utils::env_util::JWT_REFRESH_SECRET;
use crate::utils::{argon2_password_util, scrypt_password_util};

/// Session containing information of the logged-in user.
#[derive(Serialize, Deserialize)]
pub struct SetJwtRefreshDTO {
    pub token_uuid: String,
    pub user_id: u64,
    pub jwt_refresh: String,
}

pub struct JwtRefreshService<'a> {
    refresh_token_repository: RefreshTokenRepository<'a>,
    user_repository: UserRepository<'a>,
}

impl<'a> JwtRefreshService<'a> {
    pub fn new(rdb_conn: &'a MysqlConnection, redis_conn: &'a mut RedisConnection) -> Self {
        Self {
            refresh_token_repository: RefreshTokenRepository::new(redis_conn),
            user_repository: UserRepository::new(rdb_conn),
        }
    }

    /// Signs in to set user session.
    pub fn set(
        &mut self,
        email: &str,
        password: &str,
        user_agent: Option<String>,
    ) -> Result<SetJwtRefreshDTO> {
        fn encode_jwt_refresh(user_id: u64) -> Result<String> {
            Ok(encode(
                &Header::default(),
                &Claims::new(user_id),
                &EncodingKey::from_secret(JWT_REFRESH_SECRET.as_ref()),
            )?)
        }

        let user = {
            let found_password = self.user_repository.find_password_by_email(email)?;
            if found_password.starts_with("$argon2i") {
                if argon2_password_util::verify_hashed_password(&found_password, &password)? {
                    self.user_repository.find_by_email(email)?
                } else {
                    return Err(Error::Unauthorized);
                }
            } else if scrypt_password_util::check_password(password, &found_password) {
                self.user_repository.find_by_email(email)?
            } else {
                return Err(Error::Unauthorized);
            }
        };

        let jwt_refresh = encode_jwt_refresh(user.id)?;

        let token_uuid = Uuid::new_v4().to_string();
        let _ = self.refresh_token_repository.save(
            user.id,
            &token_uuid,
            &UserSession {
                jwt_refresh: jwt_refresh.clone(),
                user_agent,
                last_accessed_at: Utc::now().timestamp_millis(),
            },
        )?;

        Ok(SetJwtRefreshDTO {
            token_uuid,
            user_id: user.id,
            jwt_refresh,
        })
    }

    pub fn validate(
        &mut self,
        user_id: u64,
        token_uuid: &str,
        token: &str,
        user_agent: Option<String>,
    ) -> Result<bool> {
        let is_exist = self
            .refresh_token_repository
            .is_exist(user_id, token_uuid, token)?;

        if is_exist {
            let _ = self.refresh_token_repository.save(
                user_id,
                token_uuid,
                &UserSession {
                    jwt_refresh: token.to_string(),
                    user_agent,
                    last_accessed_at: Utc::now().timestamp_millis(),
                },
            )?;

            Ok(Claims::from_token(token).is_ok())
        } else {
            Ok(false)
        }
    }

    pub fn remove(&mut self, user_id: u64, token_uuid: &str) -> Result<bool> {
        Ok(self.refresh_token_repository.delete(user_id, token_uuid)?)
    }
}
