use diesel::{prelude::*, result::Error};

use crate::schema::posts;
use crate::models::{db_connection, post::*};

pub fn get_list() -> Result<Vec<Post>, Error> {
    let conn = db_connection::connect();
    let post_list: Vec<Post> = posts::table.load::<Post>(&conn)?;
    Ok(post_list)
}

pub fn create(args: NewPost) -> Result<bool, Error> {
    let conn = db_connection::connect();

    let post = NewPost { author: args.author, content: args.content };
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
