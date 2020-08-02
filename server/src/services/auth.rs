use actix_session::Session;
use diesel::result::Error;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::models::error::{get_service_error, ServiceError};
use crate::models::user_key::UserKeyRepository;
use crate::models::{auth::*, user::*};
use crate::utils::{email_util, password_util, session_util};

pub struct AuthService {}

impl AuthService {
    /// Signs in to set user session.
    ///
    /// 1. Finds password of the user by email from arguments.
    /// 1. Compares password from the found user and it from the arguments.
    /// 1. If the passwords are equal, returns the found user.
    pub fn login(email: &str, password: &str) -> Result<UserSession, ServiceError> {
        let user = {
            let user_repository = UserRepository::new();
            let found_password = user_repository.find_password_by_email(email);

            match found_password {
                Ok(found_password) => {
                    if password_util::check_password(password, &found_password) {
                        user_repository.find_by_email(email)
                    } else {
                        Err(Error::NotFound)
                    }
                }
                Err(error) => Err(error),
            }
        };

        let logged_in_user_session = match user {
            Ok(user) => {
                let user_key = {
                    let user_repository = UserKeyRepository::new();
                    user_repository.find_by_user_id(user.id)
                };

                let user_public_key = if let Ok(user_key) = user_key {
                    user_key.public_key
                } else {
                    return Err(get_service_error(ServiceError::NotFound(email.to_string())));
                };

                UserSession {
                    user_id: user.id,
                    user_email: user.email,
                    user_name: user.name,
                    user_public_key,
                    user_avatar_url: user.avatar_url,
                }
            }
            Err(error) => {
                return match error {
                    Error::NotFound => {
                        Err(get_service_error(ServiceError::NotFound(email.to_string())))
                    }
                    _ => Err(get_service_error(ServiceError::QueryExecutionFailure)),
                }
            }
        };

        Ok(logged_in_user_session)
    }

    /// Refreshes the user session.
    pub fn refresh_user_session(mut session: Session) -> Result<UserSession, ServiceError> {
        let user_session = session_util::get_session(&session);

        if let Some(user_session) = user_session {
            let user = {
                let user_repository = UserRepository::new();
                user_repository.find_by_id(user_session.user_id)
            };

            if let Ok(user) = user {
                session_util::set_session(
                    &mut session,
                    user_session.user_id,
                    &user_session.user_email,
                    &user.name,
                    &user_session.user_public_key,
                    &user.avatar_url,
                );

                if let Some(refreshed_user_session) = session_util::get_session(&session) {
                    Ok(refreshed_user_session)
                } else {
                    Err(get_service_error(ServiceError::Unauthorized))
                }
            } else {
                Err(get_service_error(ServiceError::UserNotFound(
                    user_session.user_id.to_string(),
                )))
            }
        } else {
            Err(get_service_error(ServiceError::Unauthorized))
        }
    }

    /// Sets token for sign up process.
    ///
    /// 1. Generates a random string called pin.
    /// 1. Creates a new token containing the pin and information of the user from arguments.
    /// 1. Serializes the token and inserts it to redis.
    pub fn set_sign_up_token(
        name: &str,
        email: &str,
        password: &str,
        avatar_url: &Option<String>,
    ) -> Result<bool, ServiceError> {
        if name.trim().is_empty() || email.trim().is_empty() || password.trim().is_empty() {
            return Err(get_service_error(ServiceError::InvalidArgument));
        }

        let pin: String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
        let hashed_password = password_util::get_hashed_password(password);

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
            let mut token_repository = SignUpTokenRepository::new();
            token_repository.save(&serialized_token)
        };

        // TODO: Specify the link.
        let _ = email_util::send_email(
            &token.email,
            &String::from("Welcome to Darim"),
            &format!("Hello {} :)\n\nYou’ve joined Darim.\n\nPlease visit the link to finish the sign up processs:\n{}", token.name, token.pin),
        );

        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Sets token for temporary password deposition in password finding process.
    pub fn set_password_token(email: &str) -> Result<bool, ServiceError> {
        let user = {
            let user_repository = UserRepository::new();
            if let Ok(user) = user_repository.find_by_email(email) {
                user
            } else {
                return Err(get_service_error(ServiceError::UserNotFound(
                    email.to_string(),
                )));
            }
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
            let mut token_repository = PasswordTokenRepository::new(user.id);
            token_repository.save(&serialized_token)
        };

        // TODO: Specify the link.
        let _ = email_util::send_email(
            email,
            &String::from("Please reset your password"),
            &format!("Hello :)\n\nPlease copy the temporary password:\n{}\n\nand visit the link to reset your password:\n{}", token.password, token.id),
        );

        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }
}
