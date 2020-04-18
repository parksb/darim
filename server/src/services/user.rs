use chrono::Utc;
use crypto::{digest::Digest, sha3::Sha3};
use diesel::prelude::*;

use crate::models::{db_connection, error::ServiceError, user::*};
use crate::schema::users;

fn get_hashed_password(original: String) -> String {
    let mut password_hasher = Sha3::sha3_512();
    password_hasher.input_str(&original);
    password_hasher.result_str()
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
        password: get_hashed_password(args.password),
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
        println!("{}", ServiceError::NotFound(id));
        Err(ServiceError::NotFound(id))
    } else {
        Ok(true)
    }
}

pub fn update(id: u64, args: UpdateArgs) -> Result<bool, ServiceError> {
    if args.name.is_none() && args.password.is_none() && args.avatar_url.is_none() {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
    }

    if let Some(name) = &args.name {
        if name.trim().is_empty() {
            println!("{}", ServiceError::InvalidArgument);
            return Err(ServiceError::InvalidArgument);
        }
    }

    if let Some(password) = &args.password {
        if password.trim().is_empty() {
            println!("{}", ServiceError::InvalidArgument);
            return Err(ServiceError::InvalidArgument);
        }
    }

    if let Some(avatar_url) = &args.avatar_url {
        if avatar_url.trim().is_empty() {
            println!("{}", ServiceError::InvalidArgument);
            return Err(ServiceError::InvalidArgument);
        }
    }

    let conn = db_connection::connect();

    let user = UserToUpdate {
        name: args.name,
        password: if let Some(password) = args.password {
            Some(get_hashed_password(password))
        } else {
            None
        },
        avatar_url: args.avatar_url,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let target_user = users::table.find(id);
    let count = diesel::update(target_user).set(user).execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::NotFound(id));
        Err(ServiceError::NotFound(id))
    } else {
        Ok(true)
    }
}
