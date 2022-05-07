use chrono::NaiveDateTime;
use diesel::MysqlConnection;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::models::auth::sign_up_token::{SignUpToken, SignUpTokenRepository};
use crate::models::connection::RedisConnection;
use crate::models::error::{Error, Result};
use crate::models::user::*;
use crate::models::user_key::UserKeyRepository;
use crate::utils::argon2_password_util;
use crate::utils::env_util::{Profile, PROFILE, RECAPTCHA_SECRET_KEY};

/// User DTO using between routes layer and service layer.
#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub public_key: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct UserService<'a> {
    sign_up_token_repository: SignUpTokenRepository<'a>,
    user_key_repository: UserKeyRepository<'a>,
    user_repository: UserRepository<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(rdb_conn: &'a MysqlConnection, redis_conn: &'a mut RedisConnection) -> Self {
        Self {
            sign_up_token_repository: SignUpTokenRepository::new(redis_conn),
            user_key_repository: UserKeyRepository::new(rdb_conn),
            user_repository: UserRepository::new(rdb_conn),
        }
    }

    /// Finds a user by id.
    pub fn get_one(&mut self, id: u64) -> Result<UserDTO> {
        let user = self.user_repository.find_by_id(id)?;
        let user_key = self.user_key_repository.find_by_user_id(id)?;

        Ok(UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar_url: user.avatar_url,
            public_key: Some(user_key.public_key),
            updated_at: user.updated_at,
            created_at: user.created_at,
        })
    }

    /// Finds all users.
    pub fn get_list(&mut self) -> Result<Vec<UserDTO>> {
        let user_list = self.user_repository.find_all()?;

        Ok(user_list
            .into_iter()
            .map(|user| -> UserDTO {
                UserDTO {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    avatar_url: user.avatar_url,
                    public_key: None,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                }
            })
            .collect())
    }

    /// Verifies reCAPTCHA.
    async fn verify_recaptcha(&self, token: &str) -> Result<bool> {
        match *PROFILE {
            Profile::PRODUCTION => {
                let form = reqwest::multipart::Form::new()
                    .text("secret", &*RECAPTCHA_SECRET_KEY)
                    .text("response", token.to_string());

                let response = Client::new()
                    .post("https://www.google.com/recaptcha/api/siteverify")
                    .multipart(form)
                    .send()
                    .await?;

                let recaptcha_response = response.json::<ReCaptchaResponse>().await?;
                Ok(recaptcha_response.success)
            }
            Profile::DEV => Ok(true),
        }
    }

    /// Creates a new user.
    ///
    /// 1. Finds serialized token by token key from arguments.
    /// 2. Deserializes the found token and compares pin from token and it from arguments.
    /// 3. If the pins are equal, deletes the token from redis and creates a new user.
    pub async fn create(
        &mut self,
        user_public_key: &str,
        token_key: &str,
        token_pin: &str,
        recaptcha_token: &str,
    ) -> Result<bool> {
        let has_recaptcha_verified = self.verify_recaptcha(&recaptcha_token).await?;
        if has_recaptcha_verified {
            let token: SignUpToken = {
                let serialized_token = self.sign_up_token_repository.find(token_key)?;
                let deserialized_token: SignUpToken = serde_json::from_str(&serialized_token)?;

                if token_pin == deserialized_token.pin {
                    let _ = self.sign_up_token_repository.delete(token_key)?;
                    deserialized_token
                } else {
                    return Err(Error::Unauthorized);
                }
            };

            let user = {
                let _ = self.user_repository.create(
                    &token.name,
                    &token.email,
                    &token.password,
                    &token.avatar_url,
                )?;

                self.user_repository.find_by_email(&token.email)?
            };

            self.user_key_repository.create(user.id, user_public_key)
        } else {
            Err(Error::Unauthorized)
        }
    }

    /// Deletes a user.
    pub fn delete(&mut self, id: u64) -> Result<bool> {
        self.user_repository.delete(id)
    }

    /// Updates a new user.
    pub fn update(
        &mut self,
        id: u64,
        name: &Option<String>,
        password: &Option<String>,
        avatar_url: &Option<String>,
    ) -> Result<bool> {
        if name.is_none() && password.is_none() && avatar_url.is_none() {
            return Err(Error::InvalidArgument);
        }

        if let (Some(name), Some(password), Some(avatar_url)) = (name, password, avatar_url) {
            if name.trim().is_empty() || password.trim().is_empty() || avatar_url.trim().is_empty()
            {
                return Err(Error::InvalidArgument);
            }
        }

        let hashed_password = if let Some(password) = password {
            let password_salt: String = argon2_password_util::generate_password_salt();
            Some(argon2_password_util::hash_password(
                &password,
                &password_salt,
            )?)
        } else {
            None
        };

        self.user_repository
            .update(id, name, &hashed_password, avatar_url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<'a> UserService<'a> {
        pub fn new_with_repository(
            sign_up_token_repository: SignUpTokenRepository<'a>,
            user_key_repository: UserKeyRepository<'a>,
            user_repository: UserRepository<'a>,
        ) -> Self {
            Self {
                sign_up_token_repository,
                user_key_repository,
                user_repository,
            }
        }
    }
}
