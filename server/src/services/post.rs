use diesel::prelude::*;
use chrono::Utc;

use crate::schema::posts;
use crate::models::{db_connection, error::ServiceError, post::*};

pub fn get_list() -> Result<Vec<Post>, ServiceError> {
    let conn = db_connection::connect();
    let post_list: Vec<Post> = posts::table.load::<Post>(&conn)?;
    Ok(post_list)
}

pub fn create(args: CreateArgs) -> Result<bool, ServiceError> {
    let conn = db_connection::connect();

    let post = PostToCreate { author: args.author, content: args.content };
    let count = diesel::insert_into(posts::table)
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

    let target_post = posts::table.find(id);
    let count = diesel::delete(target_post)
        .execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::NotFound(id));
        Err(ServiceError::NotFound(id))
    } else {
        Ok(true)
    }
}

pub fn update(id: u64, args: UpdateArgs) -> Result<bool, ServiceError> {
    let conn = db_connection::connect();

    let post = PostToUpdate {
        author: args.author,
        content: args.content,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let target_post = posts::table.find(id);
    let count = diesel::update(target_post)
        .set(post)
        .execute(&conn)?;

    if count < 1 {
        println!("{}", ServiceError::NotFound(id));
        Err(ServiceError::NotFound(id))
    } else {
        Ok(true)
    }
}
