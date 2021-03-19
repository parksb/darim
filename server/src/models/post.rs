use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use mockall::automock;
use serde::{Deserialize, Serialize};

use crate::models::connection;
use crate::models::error::{get_service_error, ServiceError};
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

/// Summarized post DTO using between routes layer and service layer.
#[derive(Serialize, Deserialize)]
pub struct SummarizedPostDTO {
    pub id: u64,
    pub title: String,
    pub date: NaiveDateTime,
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

#[automock]
pub trait PostRepositoryTrait {
    fn find(&self, user_id: u64, post_id: u64) -> Result<Post, ServiceError>;
    fn find_all(&self, user_id: u64) -> Result<Vec<Post>, ServiceError>;
    fn find_all_in_desc_date_order(&self, user_id: u64) -> Result<Vec<Post>, ServiceError>;
    fn create(
        &self,
        user_id: u64,
        title: &str,
        content: &str,
        date: &NaiveDateTime,
    ) -> Result<bool, ServiceError>;
    fn update(
        &self,
        user_id: u64,
        post_id: u64,
        title: &Option<String>,
        content: &Option<String>,
        date: &Option<NaiveDateTime>,
    ) -> Result<bool, ServiceError>;
    fn delete(&self, user_id: u64, post_id: u64) -> Result<bool, ServiceError>;
}

impl PostRepository {
    /// Creates a new post repository.
    pub fn new() -> Self {
        Self {
            conn: connection::connect_rdb(),
        }
    }

    /// Finds a post by user id and post id.
    pub fn find(&self, user_id: u64, post_id: u64) -> Result<Post, ServiceError> {
        let post: Result<Post, Error> = dsl::posts
            .find(post_id)
            .filter(dsl::user_id.eq(user_id))
            .get_result::<Post>(&self.conn);

        match post {
            Ok(post) => Ok(post),
            Err(error) => match error {
                Error::NotFound => Err(get_service_error(ServiceError::NotFound(
                    post_id.to_string(),
                ))),
                _ => Err(get_service_error(ServiceError::QueryExecutionFailure)),
            },
        }
    }

    /// Finds all post written by specific user.
    pub fn find_all(&self, user_id: u64) -> Result<Vec<Post>, ServiceError> {
        let post_list: Result<Vec<Post>, Error> = dsl::posts
            .filter(dsl::user_id.eq(user_id))
            .load::<Post>(&self.conn);

        match post_list {
            Ok(post_list) => Ok(post_list),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Finds all post written by specific user in desc date order.
    pub fn find_all_in_desc_date_order(&self, user_id: u64) -> Result<Vec<Post>, ServiceError> {
        let post_list: Result<Vec<Post>, Error> = dsl::posts
            .filter(dsl::user_id.eq(user_id))
            .order((dsl::date.desc(), dsl::id.desc()))
            .load::<Post>(&self.conn);

        match post_list {
            Ok(post_list) => Ok(post_list),
            Err(_) => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        }
    }

    /// Creates a new post.
    pub fn create(
        &self,
        user_id: u64,
        title: &str,
        content: &str,
        date: &NaiveDateTime,
    ) -> Result<bool, ServiceError> {
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
            .execute(&self.conn);

        if let Ok(count) = count {
            if count > 0 {
                Ok(true)
            } else {
                Err(get_service_error(ServiceError::QueryExecutionFailure))
            }
        } else {
            Err(get_service_error(ServiceError::QueryExecutionFailure))
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
    ) -> Result<bool, ServiceError> {
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
            .execute(&self.conn);

        match count {
            Ok(count) => {
                if count > 0 {
                    Ok(true)
                } else {
                    Err(get_service_error(ServiceError::QueryExecutionFailure))
                }
            }
            Err(error) => match error {
                Error::NotFound => Err(get_service_error(ServiceError::NotFound(
                    post_id.to_string(),
                ))),
                _ => Err(get_service_error(ServiceError::QueryExecutionFailure)),
            },
        }
    }

    /// Deletes a post written by specific user.
    pub fn delete(&self, user_id: u64, post_id: u64) -> Result<bool, ServiceError> {
        let target_post = dsl::posts.find(post_id).filter(dsl::user_id.eq(user_id));
        let count = diesel::delete(target_post).execute(&self.conn);

        match count {
            Ok(count) => {
                if count > 0 {
                    Ok(true)
                } else {
                    Err(get_service_error(ServiceError::QueryExecutionFailure))
                }
            }
            Err(error) => match error {
                Error::NotFound => Err(get_service_error(ServiceError::NotFound(
                    post_id.to_string(),
                ))),
                _ => Err(get_service_error(ServiceError::QueryExecutionFailure)),
            },
        }
    }
}

impl Default for PostRepository {
    fn default() -> Self {
        Self::new()
    }
}
