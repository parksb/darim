use diesel::{prelude::*, result::Error};
use chrono::Utc;

use crate::schema::posts;
use crate::models::{db_connection, post::*};

pub fn get_list() -> Result<Vec<Post>, Error> {
    let conn = db_connection::connect();
    let post_list: Vec<Post> = posts::table.load::<Post>(&conn)?;
    Ok(post_list)
}

pub fn create(args: CreateArgs) -> Result<bool, Error> {
    let conn = db_connection::connect();

    let post = PostToCreate { author: args.author, content: args.content };
    diesel::insert_into(posts::table)
        .values(post)
        .execute(&conn)?;

    Ok(true)
}

pub fn delete(id: u64) -> Result<bool, Error> {
    let conn = db_connection::connect();

    let target_post = posts::table.find(id);
    diesel::delete(target_post)
        .execute(&conn)?;

    Ok(true)
}

pub fn update(id: u64, args: UpdateArgs) -> Result<bool, Error> {
    let conn = db_connection::connect();

    let post = PostToUpdate {
        author: args.author,
        content: args.content,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let target_post = posts::table.find(id);
    diesel::update(target_post)
        .set(post)
        .execute(&conn)?;

    Ok(true)
}
