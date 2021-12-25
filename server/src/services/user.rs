use reqwest::Client;

use crate::models::auth::*;
use crate::models::error::{get_service_error, ServiceError};
use crate::models::user::*;
use crate::models::user_key::UserKeyRepository;
use crate::utils::env_util::RECAPTCHA_SECRET_KEY;
use crate::utils::password_util;

pub struct UserService {
    sign_up_token_repository: Option<SignUpTokenRepository>,
    password_token_repository: Option<PasswordTokenRepository>,
    user_key_repository: Option<UserKeyRepository>,
    user_repository: Option<UserRepository>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            sign_up_token_repository: None,
            password_token_repository: None,
            user_key_repository: None,
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

    fn user_key_repository(
        &mut self,
        new_repository: Option<UserKeyRepository>,
    ) -> &UserKeyRepository {
        match new_repository {
            Some(_) => {
                self.user_key_repository = new_repository;
                self.user_key_repository.as_ref().unwrap()
            }
            None => self.user_key_repository.as_ref().unwrap(),
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

    /// Finds a user by id.
    pub fn get_one(&mut self, id: u64) -> Result<UserDTO, ServiceError> {
        let user = {
            let fallback_repository =
                some_if_true!(self.user_repository.is_none() => UserRepository::new());
            self.user_repository(fallback_repository).find_by_id(id)?
        };

        let user_key = {
            let fallback_repository =
                some_if_true!(self.user_key_repository.is_none() => UserKeyRepository::new());
            self.user_key_repository(fallback_repository)
                .find_by_user_id(id)?
        };

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
    pub fn get_list(&mut self) -> Result<Vec<UserDTO>, ServiceError> {
        let user_list = {
            let fallback_repository =
                some_if_true!(self.user_repository.is_none() => UserRepository::new());
            self.user_repository(fallback_repository).find_all()?
        };

        Ok(user_list
            .iter()
            .map(|user| -> UserDTO {
                UserDTO {
                    id: user.id,
                    name: user.name.clone(),
                    email: user.email.clone(),
                    avatar_url: user.avatar_url.clone(),
                    public_key: None,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                }
            })
            .collect())
    }

    /// Verifies reCAPTCHA.
    async fn verify_recaptcha(&self, token: &str) -> Result<bool, ServiceError> {
        let form = reqwest::multipart::Form::new()
            .text("secret", &*RECAPTCHA_SECRET_KEY)
            .text("response", token.to_string());

        let response = Client::new()
            .post("https://www.google.com/recaptcha/api/siteverify")
            .multipart(form)
            .send()
            .await;

        match response {
            Ok(response) => {
                let recaptcha_response = response.json::<ReCaptchaResponse>().await;
                match recaptcha_response {
                    Ok(recaptcha_response) => Ok(recaptcha_response.success),
                    Err(_) => Err(ServiceError::InternalServerError),
                }
            }
            Err(_) => Err(ServiceError::InternalServerError),
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
    ) -> Result<bool, ServiceError> {
        let has_recaptcha_verified = self.verify_recaptcha(&recaptcha_token).await;
        match has_recaptcha_verified {
            Ok(has_recaptcha_verified) => {
                if has_recaptcha_verified {
                    let token: SignUpToken = {
                        let fallback_repository = some_if_true!(self.sign_up_token_repository.is_none() => SignUpTokenRepository::new());

                        let serialized_token = self
                            .sign_up_token_repository(fallback_repository)
                            .find(token_key)?;

                        let deserialized_token: SignUpToken = if let Ok(deserialized_token) =
                            serde_json::from_str(&serialized_token)
                        {
                            deserialized_token
                        } else {
                            return Err(get_service_error(ServiceError::InvalidFormat));
                        };

                        if token_pin == deserialized_token.pin {
                            let _ = self.sign_up_token_repository(None).delete(token_key)?;
                            deserialized_token
                        } else {
                            return Err(get_service_error(ServiceError::Unauthorized));
                        }
                    };

                    let user = {
                        let fallback_repository =
                            some_if_true!(self.user_repository.is_none() => UserRepository::new());
                        let user_repository = self.user_repository(fallback_repository);

                        user_repository.create(
                            &token.name,
                            &token.email,
                            &token.password,
                            &token.avatar_url,
                        )?;

                        user_repository.find_by_email(&token.email)?
                    };

                    let fallback_repository = some_if_true!(self.user_key_repository.is_none() => UserKeyRepository::new());
                    self.user_key_repository(fallback_repository)
                        .create(user.id, user_public_key)
                } else {
                    Err(ServiceError::Unauthorized)
                }
            }
            Err(error) => Err(error),
        }
    }

    /// Deletes a user.
    pub fn delete(&mut self, id: u64) -> Result<bool, ServiceError> {
        let fallback_repository =
            some_if_true!(self.user_repository.is_none() => UserRepository::new());
        self.user_repository(fallback_repository).delete(id)
    }

    /// Updates a new user.
    pub fn update(
        &mut self,
        id: u64,
        name: &Option<String>,
        password: &Option<String>,
        avatar_url: &Option<String>,
    ) -> Result<bool, ServiceError> {
        if name.is_none() && password.is_none() && avatar_url.is_none() {
            return Err(get_service_error(ServiceError::InvalidArgument));
        }

        if let (Some(name), Some(password), Some(avatar_url)) = (name, password, avatar_url) {
            if name.trim().is_empty() || password.trim().is_empty() || avatar_url.trim().is_empty()
            {
                return Err(get_service_error(ServiceError::InvalidArgument));
            }
        }

        let hashed_password = if let Some(password) = password {
            Some(password_util::get_hashed_password(&password))
        } else {
            None
        };

        let fallback_repository =
            some_if_true!(self.user_repository.is_none() => UserRepository::new());
        self.user_repository(fallback_repository)
            .update(id, name, &hashed_password, avatar_url)
    }

    // Reset the password.
    pub fn reset_password(
        &mut self,
        email: &str,
        token_id: &str,
        temporary_password: &str,
        new_password: &str,
    ) -> Result<bool, ServiceError> {
        let fallback_repository =
            some_if_true!(self.user_repository.is_none() => UserRepository::new());
        let user = self
            .user_repository(fallback_repository)
            .find_by_email(email)?;

        let fallback_repository = some_if_true!(self.password_token_repository.is_none() => PasswordTokenRepository::new(user.id));
        let token: PasswordToken = {
            let serialized_token = self.password_token_repository(fallback_repository).find()?;
            if let Ok(deserialized_token) = serde_json::from_str(&serialized_token) {
                deserialized_token
            } else {
                return Err(get_service_error(ServiceError::InvalidFormat));
            }
        };

        if token.id == token_id && token.password == temporary_password {
            let hashed_password = password_util::get_hashed_password(new_password);
            self.user_repository(None)
                .update(user.id, &None, &Some(hashed_password), &None)?;
            self.password_token_repository(None).delete()
        } else {
            Err(get_service_error(ServiceError::UserNotFound(
                email.to_string(),
            )))
        }
    }
}

impl Default for UserService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl UserService {
        pub fn new_with_repository(
            sign_up_token_repository: SignUpTokenRepository,
            password_token_repository: PasswordTokenRepository,
            user_key_repository: UserKeyRepository,
            user_repository: UserRepository,
        ) -> Self {
            Self {
                sign_up_token_repository: Some(sign_up_token_repository),
                password_token_repository: Some(password_token_repository),
                user_key_repository: Some(user_key_repository),
                user_repository: Some(user_repository),
            }
        }
    }
}
