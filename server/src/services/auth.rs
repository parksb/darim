use diesel::result::Error;

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
