use mysql::{Error, prelude::*};
use chrono::NaiveDateTime;

use crate::models::{db_connection, post::Post};

pub fn get_list() -> Result<Vec<Post>, Error> {
    let mut conn = db_connection::connect()?;
    let mut post_list: Vec<Post> = vec!();

    conn.query_map(
        "SELECT id, author, content, created_at, updated_at FROM posts",
        |(id, author, content, created_at, updated_at): (i32, String, String, NaiveDateTime, NaiveDateTime)| {
            let post = Post { id: Some(id), author, content, created_at, updated_at };
            post_list.push(post);
        }
    )?;

    Ok(post_list)
}
