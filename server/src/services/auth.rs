use diesel::result::Error;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::models::{auth::*, error::ServiceError, user::*};
use crate::utils::password_util;

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
        Ok(user) => UserSession {
            user_id: user.id,
            user_email: user.email,
            user_name: user.name,
            user_avatar_url: user.avatar_url,
        },
        Err(error) => {
            return match error {
                Error::NotFound => {
                    println!("{}", ServiceError::NotFound(args.email.clone()));
                    Err(ServiceError::NotFound(args.email))
                }
                _ => {
                    println!("{}", ServiceError::QueryExecutionFailure);
                    Err(ServiceError::QueryExecutionFailure)
                }
            }
        }
    };

    Ok(logged_in_user_session)
}

pub fn set_sign_up_token(args: SetSignUpTokenArgs) -> Result<bool, ServiceError> {
    if args.name.trim().is_empty()
        || args.email.trim().is_empty()
        || args.password.trim().is_empty()
    {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
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
        println!("{}", ServiceError::InvalidFormat);
        return Err(ServiceError::InvalidFormat);
    };

    let result = {
        let mut token_repository = TokenRepository::new();
        token_repository.save(&serialized_token)
    };

    match result {
        Ok(_) => Ok(true),
        Err(_) => {
            println!("{}", ServiceError::QueryExecutionFailure);
            Err(ServiceError::QueryExecutionFailure)
        }
    }
}
