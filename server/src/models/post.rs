use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use mockall::automock;
use serde::{Deserialize, Serialize};

use crate::models::error::{Error, Result};
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
pub struct PostRepository<'a> {
    conn: &'a MysqlConnection,
}

#[automock]
pub trait PostRepositoryTrait {
    fn find(&self, user_id: u64, post_id: u64) -> Result<Post>;
    fn find_all(&self, user_id: u64) -> Result<Vec<Post>>;
    fn find_all_in_desc_date_order(&self, user_id: u64) -> Result<Vec<Post>>;
    fn create(
        &self,
        user_id: u64,
        title: &str,
        content: &str,
        date: &NaiveDateTime,
    ) -> Result<bool>;
    fn update(
        &self,
        user_id: u64,
        post_id: u64,
        title: &Option<String>,
        content: &Option<String>,
        date: &Option<NaiveDateTime>,
    ) -> Result<bool>;
    fn delete(&self, user_id: u64, post_id: u64) -> Result<bool>;
}

impl<'a> PostRepository<'a> {
    /// Creates a new post repository.
    pub fn new(conn: &'a MysqlConnection) -> Self {
        Self { conn }
    }

    /// Finds a post by user id and post id.
    pub fn find(&self, user_id: u64, post_id: u64) -> Result<Post> {
        let post = dsl::posts
            .find(post_id)
            .filter(dsl::user_id.eq(user_id))
            .get_result::<Post>(self.conn)?;

        Ok(post)
    }

    /// Finds all post written by specific user.
    pub fn find_all(&self, user_id: u64) -> Result<Vec<Post>> {
        let post_list = dsl::posts
            .filter(dsl::user_id.eq(user_id))
            .load::<Post>(self.conn)?;

        Ok(post_list)
    }

    /// Finds all post written by specific user in desc date order.
    pub fn find_all_in_desc_date_order(&self, user_id: u64) -> Result<Vec<Post>> {
        let post_list: Vec<Post> = dsl::posts
            .filter(dsl::user_id.eq(user_id))
            .order((dsl::date.desc(), dsl::id.desc()))
            .load::<Post>(self.conn)?;

        Ok(post_list)
    }

    /// Creates a new post.
    pub fn create(
        &self,
        user_id: u64,
        title: &str,
        content: &str,
        date: &NaiveDateTime,
    ) -> Result<bool> {
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
            .execute(self.conn)?;

        if count > 0 {
            Ok(true)
        } else {
            Err(Error::QueryExecutionFailure)
        }
    }

    /// Updates a post written by specific user.
    pub fn update(
        &self,
        user_id: u64,
        post_id: u64,
        title: &Option<String>,
        content: &Option<String>,
        date: &Option<NaiveDateTime>,
    ) -> Result<bool> {
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
            .execute(self.conn)?;

        if count > 0 {
            Ok(true)
        } else {
            Err(Error::QueryExecutionFailure)
        }
    }

    /// Deletes a post written by specific user.
    pub fn delete(&self, user_id: u64, post_id: u64) -> Result<bool> {
        let target_post = dsl::posts.find(post_id).filter(dsl::user_id.eq(user_id));
        let count = diesel::delete(target_post).execute(self.conn)?;

        if count > 0 {
            Ok(true)
        } else {
            Err(Error::QueryExecutionFailure)
        }
    }
}
