use chrono::Utc;
use diesel::prelude::*;

use crate::models::{db_connection, error::ServiceError, user::*};
use crate::schema::users;
use crate::utils::password_util;

pub fn get_one(id: u64) -> Result<User, ServiceError> {
    let conn = db_connection::connect();
    let user = users::table.find(id).first::<User>(&conn)?;
    Ok(user)
}

pub fn get_list() -> Result<Vec<User>, ServiceError> {
    let conn = db_connection::connect();
    let user_list = users::table.load::<User>(&conn)?;
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

    let duplicated_email_count = users::table
        .filter(users::email.eq(&args.email))
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
    let count = diesel::insert_into(users::table)
        .values(user)
        .execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
    } else {
        Ok(true)
    }
}

pub fn delete(id: u64) -> Result<bool, ServiceError> {
    let conn = db_connection::connect();

    // Consider also logical deletion
    let target_user = users::table.find(id);
    let count = diesel::delete(target_user).execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::NotFound(id.to_string()));
        Err(ServiceError::NotFound(id.to_string()))
    } else {
        Ok(true)
    }
}

pub fn update(id: u64, args: UpdateArgs) -> Result<bool, ServiceError> {
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

    let target_user = users::table.find(id);
    let count = diesel::update(target_user).set(user).execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::NotFound(id.to_string()));
        Err(ServiceError::NotFound(id.to_string()))
    } else {
        Ok(true)
    }
}
