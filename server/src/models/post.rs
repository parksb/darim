use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::models::db_connection;
use crate::schema::{posts, posts::dsl};

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

#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct PostDTO {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "posts"]
struct PostDAO {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct PostRepository {
    pub conn: MysqlConnection,
}

impl PostRepository {
    pub fn new() -> Self {
        Self {
            conn: db_connection::connect(),
        }
    }

    pub fn find(&self, user_id: u64, post_id: u64) -> Result<Post, Error> {
        let post: Post = dsl::posts
            .find(post_id)
            .filter(dsl::user_id.eq(user_id))
            .get_result::<Post>(&self.conn)?;

        Ok(post)
    }

    pub fn find_all(&self, user_id: u64) -> Result<Vec<Post>, Error> {
        let post_list: Vec<Post> = dsl::posts
            .filter(dsl::user_id.eq(user_id))
            .order((dsl::date.desc(), dsl::id.desc()))
            .load::<Post>(&self.conn)?;

        Ok(post_list)
    }

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
