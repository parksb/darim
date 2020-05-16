use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::{db_connection, error::ServiceError, user::*};
use crate::schema::users::dsl;
use crate::utils::password_util;

fn check_has_permission(
    id: &u64,
    user_id: &u64,
    conn: &MysqlConnection,
) -> Result<bool, ServiceError> {
    let user: Result<User, Error> = dsl::users.find(id).get_result::<User>(conn);
    match user {
        Ok(found_user) => {
            if &found_user.id != user_id {
                println!("{}", ServiceError::Unauthorized);
                Err(ServiceError::Unauthorized)
            } else {
                Ok(true)
            }
        }
        Err(error) => match error {
            Error::NotFound => {
                println!("{}", ServiceError::NotFound(id.to_string()));
                Err(ServiceError::NotFound(id.to_string()))
            }
            _ => {
                println!("{}", ServiceError::QueryExecutionFailure);
                Err(ServiceError::QueryExecutionFailure)
            }
        },
    }
}

pub fn get_one(id: u64) -> Result<User, ServiceError> {
    let conn = db_connection::connect();
    let user = dsl::users.find(id).first::<User>(&conn)?;
    Ok(user)
}

pub fn get_list() -> Result<Vec<User>, ServiceError> {
    let conn = db_connection::connect();
    let user_list = dsl::users.load::<User>(&conn)?;
    Ok(user_list)
}

pub fn create(args: CreateArgs) -> Result<bool, ServiceError> {
    if args.name.trim().is_empty()
        || args.email.trim().is_empty()
        || args.password.trim().is_empty()
    {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
    }

    let conn = db_connection::connect();

    let duplicated_email_count = dsl::users
        .filter(dsl::email.eq(&args.email))
        .count()
        .execute(&conn)?;
    if duplicated_email_count > 0 {
        println!("{}", ServiceError::DuplicatedKey);
        return Err(ServiceError::DuplicatedKey);
    }

    let user = UserToCreate {
        name: args.name,
        email: args.email,
        password: password_util::get_hashed_password(args.password),
        avatar_url: args.avatar_url,
    };
    let count = diesel::insert_into(dsl::users)
        .values(user)
        .execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
    } else {
        Ok(true)
    }
}

pub fn delete(id: u64, user_id: u64) -> Result<bool, ServiceError> {
    let conn = db_connection::connect();

    let has_permission = check_has_permission(&id, &user_id, &conn);
    if has_permission.is_err() {
        return has_permission;
    }

    // Consider also logical deletion
    let count = diesel::delete(dsl::users.find(id)).execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
    } else {
        Ok(true)
    }
}

pub fn update(id: u64, user_id: u64, args: UpdateArgs) -> Result<bool, ServiceError> {
    if args.name.is_none() && args.password.is_none() && args.avatar_url.is_none() {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
    }

    if let (Some(name), Some(password), Some(avatar_url)) =
        (&args.name, &args.password, &args.avatar_url)
    {
        if name.trim().is_empty() || password.trim().is_empty() || avatar_url.trim().is_empty() {
            println!("{}", ServiceError::InvalidArgument);
            return Err(ServiceError::InvalidArgument);
        }
    }

    let conn = db_connection::connect();

    let has_permission = check_has_permission(&id, &user_id, &conn);
    if has_permission.is_err() {
        return has_permission;
    }

    let user = UserToUpdate {
        name: args.name,
        password: if let Some(password) = args.password {
            Some(password_util::get_hashed_password(password))
        } else {
            None
        },
        avatar_url: args.avatar_url,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let count = diesel::update(dsl::users.find(id))
        .set(user)
        .execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
    } else {
        Ok(true)
    }
}
