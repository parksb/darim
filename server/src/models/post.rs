use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::models::connection;
use crate::schema::{posts, posts::dsl};

/// Post representing `posts` table.
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

/// Arguments for `POST /posts` API.
#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
}

/// Arguments for `PATCH /posts/:id` API.
#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDateTime>,
}

/// Post DTO using between routes layer and service layer.
#[derive(Serialize, Deserialize)]
pub struct PostDTO {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

/// Post DAO using between models layer and RDB.
#[derive(Insertable, AsChangeset)]
#[table_name = "posts"]
struct PostDAO {
    id: Option<u64>,
    user_id: Option<u64>,
    title: Option<String>,
    content: Option<String>,
    date: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

/// A core data repository for post.
pub struct PostRepository {
    conn: MysqlConnection,
}

impl PostRepository {
    /// Creates a new post repository.
    pub fn new() -> Self {
        Self {
            conn: connection::connect_rdb(),
        }
    }

    /// Finds a post by user id and post id.
    pub fn find(&self, user_id: u64, post_id: u64) -> Result<Post, Error> {
        let post: Post = dsl::posts
            .find(post_id)
            .filter(dsl::user_id.eq(user_id))
            .get_result::<Post>(&self.conn)?;

        Ok(post)
    }

    /// Finds all post written by specific user.
    pub fn find_all(&self, user_id: u64) -> Result<Vec<Post>, Error> {
        let post_list: Vec<Post> = dsl::posts
            .filter(dsl::user_id.eq(user_id))
            .order((dsl::date.desc(), dsl::id.desc()))
            .load::<Post>(&self.conn)?;

        Ok(post_list)
    }

    /// Creates a new post.
    pub fn create(
        &self,
        user_id: u64,
        title: &str,
        content: &str,
        date: &NaiveDateTime,
    ) -> Result<usize, Error> {
        let post_to_create = PostDAO {
            id: None,
            user_id: Some(user_id),
            title: Some(title.to_string()),
            content: Some(content.to_string()),
            date: Some(*date),
            updated_at: None,
        };

        let count = diesel::insert_into(dsl::posts)
            .values(post_to_create)
            .execute(&self.conn)?;

        Ok(count)
    }

    /// Updates a post written by specific user.
    pub fn update(
        &self,
        user_id: u64,
        post_id: u64,
        title: &Option<String>,
        content: &Option<String>,
        date: &Option<NaiveDateTime>,
    ) -> Result<usize, Error> {
        let post_to_update = PostDAO {
            id: Some(post_id),
            user_id: None,
            title: title.clone(),
            content: content.clone(),
            date: *date,
            updated_at: Some(Utc::now().naive_utc()),
        };

        let target_post = dsl::posts.find(post_id).filter(dsl::user_id.eq(user_id));
        let count = diesel::update(target_post)
            .set(post_to_update)
            .execute(&self.conn)?;

        Ok(count)
    }

    /// Deletes a post written by specific user.
    pub fn delete(&self, user_id: u64, post_id: u64) -> Result<usize, Error> {
        let target_post = dsl::posts.find(post_id).filter(dsl::user_id.eq(user_id));
        let count = diesel::delete(target_post).execute(&self.conn)?;

        Ok(count)
    }
}

impl Default for PostRepository {
    fn default() -> Self {
        Self::new()
    }
}
