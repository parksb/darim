use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::{db_connection, error::ServiceError, post::*};
use crate::schema::posts::dsl;

fn check_has_permission(
    id: &u64,
    user_id: &u64,
    conn: &MysqlConnection,
) -> Result<bool, ServiceError> {
    let post: Result<Post, Error> = dsl::posts.find(id).get_result::<Post>(conn);
    match post {
        Ok(found_post) => {
            if &found_post.user_id != user_id {
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

pub fn get(user_id: u64, post_id: u64) -> Result<PostToShow, ServiceError> {
    let conn = db_connection::connect();

    let post: Result<Post, Error> = dsl::posts
        .find(post_id)
        .get_result::<Post>(&conn);

    match post {
        Ok(found_post) => {
            if found_post.user_id != user_id {
                println!("{}", ServiceError::Unauthorized);
                Err(ServiceError::Unauthorized)
            } else {
                Ok(
                    PostToShow {
                        id: found_post.id,
                        title: found_post.title,
                        content: found_post.content,
                        date: found_post.date,
                        updated_at: found_post.updated_at,
                        created_at: found_post.created_at,
                    }
                )
            }
        }
        Err(error) => match error {
            Error::NotFound => {
                println!("{}", ServiceError::NotFound(post_id.to_string()));
                Err(ServiceError::NotFound(post_id.to_string()))
            }
            _ => {
                println!("{}", ServiceError::QueryExecutionFailure);
                Err(ServiceError::QueryExecutionFailure)
            }
        },
    }
}

pub fn get_list(user_id: u64) -> Result<Vec<PostToShow>, ServiceError> {
    let conn = db_connection::connect();

    let post_list: Vec<Post> = dsl::posts
        .filter(dsl::user_id.eq(&user_id))
        .order(dsl::created_at.desc())
        .load::<Post>(&conn)?;
    let post_to_show_list = post_list
        .iter()
        .map(|post| -> PostToShow {
            PostToShow {
                id: post.id,
                title: post.title.clone(),
                content: post.content.clone(),
                date: post.date,
                created_at: post.created_at,
                updated_at: post.updated_at,
            }
        })
        .collect();

    Ok(post_to_show_list)
}

pub fn create(user_id: u64, args: CreateArgs) -> Result<bool, ServiceError> {
    if args.title.trim().is_empty() || args.content.trim().is_empty() {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
    }

    let conn = db_connection::connect();

    let post = PostToCreate {
        user_id,
        title: args.title,
        content: args.content,
        date: args.date,
    };
    let count = diesel::insert_into(dsl::posts)
        .values(post)
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

    let target_post = dsl::posts.find(id);
    let count = diesel::delete(target_post).execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::NotFound(id.to_string()));
        Err(ServiceError::NotFound(id.to_string()))
    } else {
        Ok(true)
    }
}

pub fn update(id: u64, user_id: u64, args: UpdateArgs) -> Result<bool, ServiceError> {
    if args.title.is_none() && args.content.is_none() && args.date.is_none() {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
    }

    if let Some(content) = &args.content {
        if content.trim().is_empty() {
            println!("{}", ServiceError::InvalidArgument);
            return Err(ServiceError::InvalidArgument);
        }
    }

    if let Some(title) = &args.title {
        if title.trim().is_empty() {
            println!("{}", ServiceError::InvalidArgument);
            return Err(ServiceError::InvalidArgument);
        }
    }

    let conn = db_connection::connect();

    let has_permission = check_has_permission(&id, &user_id, &conn);
    if has_permission.is_err() {
        return has_permission;
    }

    let post = PostToUpdate {
        title: args.title,
        content: args.content,
        date: args.date,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let count = diesel::update(dsl::posts.find(id))
        .set(post)
        .execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
    } else {
        Ok(true)
    }
}
