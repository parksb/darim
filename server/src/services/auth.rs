use diesel::{prelude::*, result::Error};

use crate::models::{auth::*, db_connection, error::ServiceError, user::*};
use crate::schema::users::dsl;
use crate::utils::password_util;

pub fn login(args: LoginArgs) -> Result<UserSession, ServiceError> {
    let conn = db_connection::connect();

    let hashed_password = password_util::get_hashed_password(args.password);
    let found_user: Result<User, Error> = dsl::users
        .filter(
            dsl::email
                .eq(&args.email)
                .and(dsl::password.eq(hashed_password)),
        )
        .get_result::<User>(&conn);

    let logged_in_user_session = match found_user {
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
                _ => Err(ServiceError::QueryExecutionFailure),
            }
        }
    };

    Ok(logged_in_user_session)
}
