use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use uuid::Uuid;

use crate::models::auth::*;
use crate::models::error::{get_service_error, ServiceError};
use crate::models::user::UserRepository;
use crate::utils::env_util::{CLIENT_ADDRESS, JWT_REFRESH_SECRET};
use crate::utils::{argon2_password_util, email_util, scrypt_password_util};

pub struct AuthService {
    sign_up_token_repository: Option<SignUpTokenRepository>,
    password_token_repository: Option<PasswordTokenRepository>,
    refresh_token_repository: Option<RefreshTokenRepository>,
    user_repository: Option<UserRepository>,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            sign_up_token_repository: None,
            password_token_repository: None,
            refresh_token_repository: None,
            user_repository: None,
        }
    }

    fn sign_up_token_repository(
        &mut self,
        new_repository: Option<SignUpTokenRepository>,
    ) -> &mut SignUpTokenRepository {
        match new_repository {
            Some(_) => {
                self.sign_up_token_repository = new_repository;
                self.sign_up_token_repository.as_mut().unwrap()
            }
            None => self.sign_up_token_repository.as_mut().unwrap(),
        }
    }

    fn password_token_repository(
        &mut self,
        new_repository: Option<PasswordTokenRepository>,
    ) -> &mut PasswordTokenRepository {
        match new_repository {
            Some(_) => {
                self.password_token_repository = new_repository;
                self.password_token_repository.as_mut().unwrap()
            }
            None => self.password_token_repository.as_mut().unwrap(),
        }
    }

    fn refresh_token_repository(
        &mut self,
        new_repository: Option<RefreshTokenRepository>,
    ) -> &mut RefreshTokenRepository {
        match new_repository {
            Some(_) => {
                self.refresh_token_repository = new_repository;
                self.refresh_token_repository.as_mut().unwrap()
            }
            None => self.refresh_token_repository.as_mut().unwrap(),
        }
    }

    fn user_repository(&mut self, new_repository: Option<UserRepository>) -> &UserRepository {
        match new_repository {
            Some(_) => {
                self.user_repository = new_repository;
                self.user_repository.as_ref().unwrap()
            }
            None => self.user_repository.as_ref().unwrap(),
        }
    }

    /// Signs in to set user session.
    pub fn set_jwt_refresh(
        &mut self,
        email: &str,
        password: &str,
        user_agent: Option<String>,
    ) -> Result<SetJwtRefreshDTO, ServiceError> {
        let user = {
            let fallback_repository =
                some_if_true!(self.user_repository.is_none() => UserRepository::new());
            let found_password = self
                .user_repository(fallback_repository)
                .find_password_by_email(email)?;

            if found_password.starts_with("$argon2i") {
                if argon2_password_util::verify_hashed_password(&found_password, &password) {
                    self.user_repository(None).find_by_email(email)?
                } else {
                    return Err(ServiceError::Unauthorized);
                }
            } else if scrypt_password_util::check_password(password, &found_password) {
                self.user_repository(None).find_by_email(email)?
            } else {
                return Err(ServiceError::Unauthorized);
            }
        };

        let jwt_refresh = {
            let encoded_token = encode(
                &Header::default(),
                &Claims::new(user.id),
                &EncodingKey::from_secret(JWT_REFRESH_SECRET.as_ref()),
            );

            if let Ok(encoded_token) = encoded_token {
                encoded_token
            } else {
                return Err(ServiceError::InternalServerError);
            }
        };

        let token_uuid = Uuid::new_v4().to_string();
        let _ = {
            let fallback_repository = some_if_true!(self.refresh_token_repository.is_none() => RefreshTokenRepository::new());
            self.refresh_token_repository(fallback_repository).save(
                user.id,
                &token_uuid,
                &UserSession {
                    jwt_refresh: jwt_refresh.clone(),
                    user_agent,
                    last_accessed_at: Utc::now().timestamp_millis(),
                },
            )?
        };

        Ok(SetJwtRefreshDTO {
            token_uuid,
            user_id: user.id,
            jwt_refresh,
        })
    }

    pub fn validate_jwt_refresh(
        &mut self,
        user_id: u64,
        token_uuid: &str,
        token: &str,
        user_agent: Option<String>,
    ) -> bool {
        let is_exist = {
            let fallback_repository = some_if_true!(self.refresh_token_repository.is_none() => RefreshTokenRepository::new());
            self.refresh_token_repository(fallback_repository).is_exist(user_id, token_uuid, token)
        }.is_ok();

        if is_exist {
            let _ = {
                let fallback_repository = some_if_true!(self.refresh_token_repository.is_none() => RefreshTokenRepository::new());
                self.refresh_token_repository(fallback_repository).save(
                    user_id,
                    token_uuid,
                    &UserSession {
                        jwt_refresh: token.to_string(),
                        user_agent,
                        last_accessed_at: Utc::now().timestamp_millis(),
                    },
                )
            };

            Claims::from_token(token).is_ok()
        } else {
            false
        }
    }

    pub fn remove_jwt_refresh(&mut self, user_id: u64, token_uuid: &str) -> bool {
        let fallback_repository =
            some_if_true!(self.refresh_token_repository.is_none() => RefreshTokenRepository::new());
        self.refresh_token_repository(fallback_repository)
            .delete(user_id, token_uuid)
            .is_ok()
    }

    pub fn get_session_list(&mut self, user_id: u64) -> Result<Vec<UserSessionDTO>, ServiceError> {
        let fallback_repository =
            some_if_true!(self.refresh_token_repository.is_none() => RefreshTokenRepository::new());
        self.refresh_token_repository(fallback_repository)
            .find_all_by_user_id(user_id)
    }

    /// Sets token for sign up process.
    ///
    /// 1. Generates a random string called pin.
    /// 2. Creates a new token containing the pin and information of the user from arguments.
    /// 3. Serializes the token and inserts it to redis.
    pub fn set_sign_up_token(
        &mut self,
        name: &str,
        email: &str,
        password: &str,
        avatar_url: &Option<String>,
    ) -> Result<String, ServiceError> {
        if name.trim().is_empty() || email.trim().is_empty() || password.trim().is_empty() {
            return Err(get_service_error(ServiceError::InvalidArgument));
        }

        let pin: String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
        let password_salt: String = argon2_password_util::generate_password_salt();
        let hashed_password = argon2_password_util::hash_password(password, &password_salt);

        let token = SignUpToken {
            pin,
            name: name.to_string(),
            email: email.to_string(),
            password: hashed_password,
            avatar_url: avatar_url.clone(),
        };

        let serialized_token = serde_json::to_string(&token);
        let serialized_token = if let Ok(serialized_token) = serialized_token {
            serialized_token
        } else {
            return Err(get_service_error(ServiceError::InvalidFormat));
        };

        let result = {
            let fallback_repository = some_if_true!(self.sign_up_token_repository.is_none() => SignUpTokenRepository::new());
            self.sign_up_token_repository(fallback_repository)
                .save(&serialized_token)?
        };

        let email_content = format!(
            "<h1>🏕 Welcome to Darim</h1>\
            <h2>Hello {} :)</h2>\
            You've joined Darim.<br/><br/>\
            Please copy the key below to finish the sign up process:<br/><br/>\
            <div style=\"background-color: #f0f0f0; padding: 10px; font-size: 20px; font-weight: bold\">{}</div>",
            token.name, token.pin,
        );

        let _ = email_util::send_email(
            &format!("{} <{}>", &token.name, &token.email),
            &String::from("Welcome to Darim 🎉"),
            &email_content,
        );

        Ok(result)
    }

    /// Sets token for temporary password deposition in password finding process.
    pub fn set_password_token(&mut self, email: &str) -> Result<bool, ServiceError> {
        let user = {
            let fallback_repository =
                some_if_true!(self.user_repository.is_none() => UserRepository::new());
            self.user_repository(fallback_repository)
                .find_by_email(email)?
        };

        let token = PasswordToken {
            id: thread_rng().sample_iter(&Alphanumeric).take(32).collect(),
            password: thread_rng().sample_iter(&Alphanumeric).take(512).collect(),
        };

        let serialized_token = serde_json::to_string(&token);
        let serialized_token = if let Ok(serialized_token) = serialized_token {
            serialized_token
        } else {
            return Err(get_service_error(ServiceError::InvalidFormat));
        };

        let result = {
            let fallback_repository = some_if_true!(self.password_token_repository.is_none() => PasswordTokenRepository::new(user.id));
            self.password_token_repository(fallback_repository)
                .save(&serialized_token)?
        };

        let email_content = format!(
            "Hello :)<br/><br/>\
            Please copy the temporary password:<br/><br/>\
            <div style=\"background-color: #f0f0f0; padding: 10px; font-weight: bold\">{}</div><br/><br/>\
            and visit the link to reset your password:<br/><br/>\
            <a href=\"{}/password_reset/{}\">{}/password_reset/{}</a>",
            token.password, *CLIENT_ADDRESS, token.id, *CLIENT_ADDRESS, token.id,
        );

        let _ = email_util::send_email(
            &format!("{} <{}>", user.name, email),
            &String::from("Please reset your password 🔒"),
            &email_content,
        );

        Ok(result)
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl AuthService {
        pub fn new_with_repository(
            sign_up_token_repository: SignUpTokenRepository,
            password_token_repository: PasswordTokenRepository,
            refresh_token_repository: RefreshTokenRepository,
            user_repository: UserRepository,
        ) -> Self {
            Self {
                sign_up_token_repository: Some(sign_up_token_repository),
                password_token_repository: Some(password_token_repository),
                refresh_token_repository: Some(refresh_token_repository),
                user_repository: Some(user_repository),
            }
        }
    }
}
