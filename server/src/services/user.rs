use diesel::result::Error;

use crate::models::auth::{
    PasswordToken, PasswordTokenRepository, SignUpToken, SignUpTokenRepository,
};
use crate::models::error::{get_service_error, ServiceError};
use crate::models::user::*;
use crate::models::user_key::UserKeyRepository;
use crate::utils::password_util;

pub struct UserService {}

impl UserService {
    /// Finds a user by id.
    pub fn get_one(id: u64) -> Result<UserDTO, ServiceError> {
        let user = {
            let user_repository = UserRepository::new();
            user_repository.find_by_id(id)
        };

        match user {
            Ok(user) => Ok(UserDTO {
                id: user.id,
                name: user.name,
                email: user.email,
                avatar_url: user.avatar_url,
                updated_at: user.updated_at,
                created_at: user.created_at,
            }),
            Err(error) => match error {
                Error::NotFound => Err(get_service_error(ServiceError::NotFound(id.to_string()))),
                _ => Err(get_service_error(ServiceError::QueryExecutionFailure)),
            },
        }
    }

    /// Finds all users.
    pub fn get_list() -> Result<Vec<UserDTO>, ServiceError> {
        let user_list = {
            let user_repository = UserRepository::new();
            user_repository.find_all()
        };

        if let Ok(user_list) = user_list {
            let user_to_show_list = user_list
                .iter()
                .map(|user| -> UserDTO {
                    UserDTO {
                        id: user.id,
                        name: user.name.clone(),
                        email: user.email.clone(),
                        avatar_url: user.avatar_url.clone(),
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    }
                })
                .collect();

            Ok(user_to_show_list)
        } else {
            Err(get_service_error(ServiceError::QueryExecutionFailure))
        }
    }

    /// Creates a new user.
    ///
    /// 1. Finds serialized token by token key from arguments.
    /// 1. Deserializes the found token and compares pin from token and it from arguments.
    /// 1. If the pins are equal, deletes the token from redis and creates a new user.
    pub fn create(
        user_public_key: &str,
        token_key: &str,
        token_pin: &str,
    ) -> Result<bool, ServiceError> {
        let token: SignUpToken = {
            let mut token_repository = SignUpTokenRepository::new();
            let serialized_token = if let Ok(serialized_token) = token_repository.find(token_key) {
                serialized_token
            } else {
                return Err(get_service_error(ServiceError::NotFound(
                    token_key.to_string(),
                )));
            };

            let deserialized_token: SignUpToken =
                if let Ok(deserialized_token) = serde_json::from_str(&serialized_token) {
                    deserialized_token
                } else {
                    return Err(get_service_error(ServiceError::InvalidFormat));
                };

            if token_pin == deserialized_token.pin {
                let _ = token_repository.delete(token_key);
                deserialized_token
            } else {
                return Err(get_service_error(ServiceError::Unauthorized));
            }
        };

        let (created_count, user) = {
            let user_repository = UserRepository::new();

            let created_count = user_repository.create(
                &token.name,
                &token.email,
                &token.password,
                &token.avatar_url,
            );
            let user = user_repository.find_by_email(&token.email);

            (created_count, user)
        };

        // FIXME: Improve error handling.
        if let (Ok(created_count), Ok(user)) = (created_count, user) {
            if created_count > 0 {
                let user_key_created_count = {
                    let user_key_repository = UserKeyRepository::new();
                    user_key_repository.create(user.id, user_public_key)
                };

                if let Ok(user_key_created_count) = user_key_created_count {
                    if user_key_created_count > 0 {
                        Ok(true)
                    } else {
                        Err(get_service_error(ServiceError::QueryExecutionFailure))
                    }
                } else {
                    Err(get_service_error(ServiceError::QueryExecutionFailure))
                }
            } else {
                Err(get_service_error(ServiceError::QueryExecutionFailure))
            }
        } else {
            Err(get_service_error(ServiceError::QueryExecutionFailure))
        }
    }

    /// Deletes a user.
    pub fn delete(id: u64) -> Result<bool, ServiceError> {
        let deleted_count = {
            let user_repository = UserRepository::new();
            user_repository.delete(id)
        };

        if let Ok(deleted_count) = deleted_count {
            if deleted_count > 0 {
                Ok(true)
            } else {
                Err(get_service_error(ServiceError::QueryExecutionFailure))
            }
        } else {
            Err(get_service_error(ServiceError::QueryExecutionFailure))
        }
    }

    /// Updates a new user.
    pub fn update(
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

        let updated_count = {
            let user_repository = UserRepository::new();

            let hashed_password = if let Some(password) = password {
                Some(password_util::get_hashed_password(&password))
            } else {
                None
            };

            user_repository.update(id, name, &hashed_password, avatar_url)
        };

        if let Ok(updated_count) = updated_count {
            if updated_count > 0 {
                Ok(true)
            } else {
                Err(get_service_error(ServiceError::QueryExecutionFailure))
            }
        } else {
            Err(get_service_error(ServiceError::QueryExecutionFailure))
        }
    }

    // Reset the password.
    pub fn reset_password(
        email: &str,
        token_id: &str,
        temporary_password: &str,
        new_password: &str,
    ) -> Result<bool, ServiceError> {
        let user_repository = UserRepository::new();
        let user = if let Ok(user) = user_repository.find_by_email(email) {
            user
        } else {
            return Err(get_service_error(ServiceError::UserNotFound(
                email.to_string(),
            )));
        };

        let mut token_repository = PasswordTokenRepository::new(user.id);
        let token: PasswordToken = {
            if let Ok(serialized_token) = token_repository.find() {
                if let Ok(deserialized_token) = serde_json::from_str(&serialized_token) {
                    deserialized_token
                } else {
                    return Err(get_service_error(ServiceError::InvalidFormat));
                }
            } else {
                return Err(get_service_error(ServiceError::NotFound(
                    user.id.to_string(),
                )));
            }
        };

        if token.id == token_id && token.password == temporary_password {
            let hashed_password = password_util::get_hashed_password(new_password);
            if let Ok(count_updated) =
                user_repository.update(user.id, &None, &Some(hashed_password), &None)
            {
                if count_updated > 0 {
                    let _ = token_repository.delete();
                    Ok(true)
                } else {
                    Err(get_service_error(ServiceError::QueryExecutionFailure))
                }
            } else {
                Err(get_service_error(ServiceError::UserNotFound(
                    email.to_string(),
                )))
            }
        } else {
            Err(get_service_error(ServiceError::Unauthorized))
        }
    }
}
