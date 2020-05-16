use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::{db_connection, error::ServiceError, post::*};
use crate::schema::posts::dsl;
use crate::services::user;

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

pub fn create(args: CreateArgs) -> Result<bool, ServiceError> {
    if args.content.trim().is_empty() {
        println!("{}", ServiceError::InvalidArgument);
        return Err(ServiceError::InvalidArgument);
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

pub fn delete(id: u64) -> Result<bool, ServiceError> {
    let conn = db_connection::connect();

    let target_post = dsl::posts.find(id);
    let count = diesel::delete(target_post).execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::NotFound(id.to_string()));
        Err(ServiceError::NotFound(id.to_string()))
    } else {
        Ok(true)
    }
}

pub fn update(id: u64, args: UpdateArgs) -> Result<bool, ServiceError> {
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

    let post = PostToUpdate {
        content: args.content,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let target_post_query = dsl::posts.find(id);
    let target_post: Result<Post, Error> = target_post_query.get_result::<Post>(&conn);

    if let Ok(found_target_post) = target_post {
        if found_target_post.user_id != args.user_id {
            return Err(ServiceError::Unauthorized);
        }
    } else {
        if let Err(error) = target_post {
            return match error {
                Error::NotFound => {
                    println!("{}", ServiceError::NotFound(id.to_string()));
                    Err(ServiceError::NotFound(id.to_string()))
                }
                _ => Err(ServiceError::QueryExecutionFailure),
            };
        }
    }

    let count = diesel::update(target_post_query).set(post).execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::QueryExecutionFailure);
        Err(ServiceError::QueryExecutionFailure)
    } else {
        Ok(true)
    }
}
