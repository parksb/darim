use diesel::result::Error;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::models::error::{get_service_error, ServiceError};
use crate::models::user_key::UserKeyRepository;
use crate::models::{auth::*, user::*};
use crate::utils::password_util;

/// Signs in to set user session.
///
/// 1. Finds password of the user by email from arguments.
/// 1. Compares password from the found user and it from the arguments.
/// 1. If the passwords are equal, returns the found user.
pub fn login(args: LoginArgs) -> Result<UserSession, ServiceError> {
    let user = {
        let user_repository = UserRepository::new();
        let password = user_repository.find_password_by_email(&args.email);

        match password {
            Ok(password) => {
                if password_util::check_password(&args.password, &password) {
                    user_repository.find_by_email(&args.email)
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
                return Err(get_service_error(ServiceError::NotFound(args.email)));
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
                Error::NotFound => Err(get_service_error(ServiceError::NotFound(args.email))),
                _ => Err(get_service_error(ServiceError::QueryExecutionFailure)),
            }
        }
    };

    Ok(logged_in_user_session)
}

/// Sets token for sign up process.
///
/// 1. Generates a random string called pin.
/// 1. Creates a new token containing the pin and information of the user from arguments.
/// 1. Serializes the token and inserts it to redis.
pub fn set_sign_up_token(args: SetSignUpTokenArgs) -> Result<bool, ServiceError> {
    if args.name.trim().is_empty()
        || args.email.trim().is_empty()
        || args.password.trim().is_empty()
    {
        return Err(get_service_error(ServiceError::InvalidArgument));
    }

    let pin: String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
    let password = password_util::get_hashed_password(&args.password);

    let token = Token {
        pin,
        name: args.name,
        email: args.email,
        password,
        avatar_url: args.avatar_url,
    };

    let serialized_token = serde_json::to_string(&token);
    let serialized_token = if let Ok(serialized_token) = serialized_token {
        serialized_token
    } else {
        return Err(get_service_error(ServiceError::InvalidFormat));
    };

    let result = {
        let mut token_repository = TokenRepository::new();
        token_repository.save(&serialized_token)
    };

    match result {
        Ok(_) => Ok(true),
        Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
    }
}
