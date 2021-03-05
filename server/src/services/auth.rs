use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::env;

use crate::models::auth::*;
use crate::models::error::{get_service_error, ServiceError};
use crate::models::user::UserRepository;
use crate::models::user_key::UserKeyRepository;
use crate::utils::{email_util, password_util};

pub struct AuthService {
    sign_up_token_repository: Option<SignUpTokenRepository>,
    password_token_repository: Option<PasswordTokenRepository>,
    user_key_repository: Option<UserKeyRepository>,
    user_repository: Option<UserRepository>,
}

impl AuthService {
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

    /// Signs in to set user session.
    ///
    /// 1. Finds password of the user by email from arguments.
    /// 2. Compares password from the found user and it from the arguments.
    /// 3. If the passwords are equal, returns the found user.
    pub fn login(&mut self, email: &str, password: &str) -> Result<UserSession, ServiceError> {
        let user = {
            let fallback_repository =
                some_if_true!(self.user_repository.is_none() => UserRepository::new());
            let found_password = self
                .user_repository(fallback_repository)
                .find_password_by_email(email)?;

            if password_util::check_password(password, &found_password) {
                self.user_repository(None).find_by_email(email)?
            } else {
                return Err(ServiceError::Unauthorized);
            }
        };

        let logged_in_user_session = {
            let user_public_key = {
                let fallback_repository =
                    some_if_true!(self.user_key_repository.is_none() => UserKeyRepository::new());
                self.user_key_repository(fallback_repository)
                    .find_by_user_id(user.id)?
                    .public_key
            };

            UserSession {
                user_id: user.id,
                user_email: user.email,
                user_name: user.name,
                user_public_key,
                user_avatar_url: user.avatar_url,
            }
        };

        Ok(logged_in_user_session)
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

        // FIXME: The compiler doesn't consider it string type when running test.
        let result = {
            let fallback_repository = some_if_true!(self.sign_up_token_repository.is_none() => SignUpTokenRepository::new());
            self.sign_up_token_repository(fallback_repository)
                .save(&serialized_token)?
                .to_string()
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

        let client_address = env::var("CLIENT_ADDRESS").expect("CLIENT_ADDRESS not found");
        let email_content = format!(
            "Hello :)<br/><br/>\
            Please copy the temporary password:<br/><br/>\
            <div style=\"background-color: #f0f0f0; padding: 10px; font-weight: bold\">{}</div><br/><br/>\
            and visit the link to reset your password:<br/><br/>\
            <a href=\"{}/password_reset/{}\">{}/password_reset/{}</a>",
            token.password, client_address, token.id, client_address, token.id,
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
