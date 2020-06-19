use diesel::result::Error;

use crate::models::{error::ServiceError, user::*};
use crate::utils::password_util;

pub fn get_one(id: u64) -> Result<UserDTO, ServiceError> {
    let user = {
        let user_repository = UserRepository::new();
        user_repository.find_by_id(id)
    };

    match user {
        Ok(user) => Ok(UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar_url: user.avatar_url,
            updated_at: user.updated_at,
            created_at: user.created_at,
        }),
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

pub fn get_list() -> Result<Vec<UserDTO>, ServiceError> {
    let user_list = {
        let user_repository = UserRepository::new();
        user_repository.find_all()
    };

    if let Ok(user_list) = user_list {
        let user_to_show_list = user_list
            .iter()
            .map(|user| -> UserDTO {
                UserDTO {
                    id: user.id,
                    name: user.name.clone(),
                    email: user.email.clone(),
                    avatar_url: user.avatar_url.clone(),
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                }
            })
            .collect();

        Ok(user_to_show_list)
    } else {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
    }
}

pub fn create(args: CreateArgs) -> Result<bool, ServiceError> {
    if args.name.trim().is_empty()
        || args.email.trim().is_empty()
        || args.password.trim().is_empty()
    {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
    }

    let created_count = {
        let user_repository = UserRepository::new();
        let password = password_util::get_hashed_password(&args.password);
        user_repository.create(&args.name, &args.email, &password, &args.avatar_url)
    };

    if let Ok(created_count) = created_count {
        if created_count > 0 {
            Ok(true)
        } else {
            println!("{}", ServiceError::QueryExecutionFailure);
            Err(ServiceError::QueryExecutionFailure)
        }
    } else {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
    }
}

pub fn delete(id: u64) -> Result<bool, ServiceError> {
    let deleted_count = {
        let user_repository = UserRepository::new();
        user_repository.delete(id)
    };

    if let Ok(deleted_count) = deleted_count {
        if deleted_count > 0 {
            Ok(true)
        } else {
            println!("{}", ServiceError::QueryExecutionFailure);
            Err(ServiceError::QueryExecutionFailure)
        }
    } else {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
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

    let updated_count = {
        let user_repository = UserRepository::new();

        let password = if let Some(password) = args.password {
            Some(password_util::get_hashed_password(&password))
        } else {
            None
        };

        user_repository.update(id, &args.name, &password, &args.avatar_url)
    };

    if let Ok(updated_count) = updated_count {
        if updated_count > 0 {
            Ok(true)
        } else {
            println!("{}", ServiceError::QueryExecutionFailure);
            Err(ServiceError::QueryExecutionFailure)
        }
    } else {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
    }
}
