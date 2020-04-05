use mysql::{Error, prelude::*};
use chrono::NaiveDateTime;

use crate::models::{db_connection, post::Post, post::CreatePostArgs};

pub fn get_list() -> Result<Vec<Post>, Error> {
    let mut conn = db_connection::connect()?;
    let mut post_list: Vec<Post> = vec!();

    conn.query_map(
        "SELECT id, author, content, created_at, updated_at FROM posts",
        |(id, author, content, created_at, updated_at): (i32, String, String, NaiveDateTime, NaiveDateTime)| {
            post_list.push(Post {
                author,
                content,
                id: Some(id),
                created_at: Some(created_at),
                updated_at: Some(updated_at),
            });
        }
    )?;

    Ok(post_list)
}

pub fn create(args: CreatePostArgs) -> Result<bool, Error> {
    let mut conn = db_connection::connect()?;
    let post = Post::new(args.author, args.content);

    conn.exec_drop(
        "INSERT INTO posts (author, content, created_at, updated_at) VALUES (?, ?, ?, ?)",
        (&post.author, &post.content, &post.created_at, &post.updated_at)
    )?;

    Ok(true)
}

pub fn delete(id: i32) -> Result<bool, Error> {
    let mut conn = db_connection::connect()?;

    conn.exec_drop(
        "DELETE FROM posts WHERE id = ?",
        (id,),
    )?;

    Ok(true)
}
