use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::{db_connection, error::ServiceError, post::*};
use crate::schema::posts::dsl;
use crate::services::user;

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

pub fn get_list() -> Result<Vec<PostToShow>, ServiceError> {
    let conn = db_connection::connect();
    let post_list: Vec<Post> = dsl::posts
        .order(dsl::created_at.desc())
        .load::<Post>(&conn)?;

    // FIXME: Change to join query
    let post_to_show_list: Vec<PostToShow> = post_list
        .iter()
        .map(|post| -> Option<PostToShow> {
            let found_author = user::get_one(post.user_id);
            return if let Ok(author) = found_author {
                Some(PostToShow {
                    id: post.id,
                    user_id: author.id,
                    user_name: author.name,
                    user_avatar_url: author.avatar_url,
                    content: post.content.clone(),
                    created_at: post.created_at,
                    updated_at: post.updated_at,
                })
            } else {
                None
            };
        })
        .filter(|post| post.is_some())
        .map(|post| post.unwrap())
        .collect();

    Ok(post_to_show_list)
}

pub fn create(user_id: u64, args: CreateArgs) -> Result<bool, ServiceError> {
    if args.content.trim().is_empty() {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
    }

    if user_id != args.user_id {
        println!("{}", ServiceError::Unauthorized);
        return Err(ServiceError::Unauthorized);
    }

    let conn = db_connection::connect();

    let post = PostToCreate {
        user_id: args.user_id,
        content: args.content,
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
    if args.content.is_none() {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
    }

    if let Some(content) = &args.content {
        if content.trim().is_empty() {
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
        content: args.content,
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
