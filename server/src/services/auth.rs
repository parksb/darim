use diesel::{result::Error, prelude::*};

use crate::models::{auth::*, db_connection, error::ServiceError, user::*};
use crate::schema::users::dsl;

pub fn login(args: LoginArgs) -> Result<UserSession, ServiceError> {
    let conn = db_connection::connect();

    let found_user: Result<User, Error> = dsl::users
        .filter(
            dsl::email
                .eq(&args.email)
                .and(dsl::password.eq(&args.password)),
        )
        .get_result::<User>(&conn);

    let logged_in_user_session = match found_user {
        Ok(user) => UserSession { user_email: user.email, user_name: user.name },
        Err(error) => return match error {
            Error::NotFound => {
                println!("{}", ServiceError::NotFound(args.email.clone()));
                Err(ServiceError::NotFound(args.email))
            },
            _ => Err(ServiceError::QueryExecutionFailure),
        }
    };

    Ok(logged_in_user_session)
}
