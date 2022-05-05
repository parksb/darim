use chrono::NaiveDateTime;
use diesel::MysqlConnection;
use serde::{Deserialize, Serialize};

use crate::models::error::{Error, Result};
use crate::models::post::*;

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

pub struct PostService<'a> {
    post_repository: PostRepository<'a>,
}

impl<'a> PostService<'a> {
    pub fn new(conn: &'a MysqlConnection) -> Self {
        Self {
            post_repository: PostRepository::new(conn),
        }
    }

    /// Finds a post by user id and post id.
    pub fn get(&mut self, user_id: u64, id: u64) -> Result<PostDTO> {
        let post = self.post_repository.find(user_id, id)?;

        Ok(PostDTO {
            id: post.id,
            title: post.title,
            content: post.content,
            date: post.date,
            updated_at: post.updated_at,
            created_at: post.created_at,
        })
    }

    /// Finds all post written by specific user.
    pub fn get_list(&mut self, user_id: u64) -> Result<Vec<PostDTO>> {
        let post_list = self.post_repository.find_all_in_desc_date_order(user_id)?;

        Ok(post_list
            .into_iter()
            .map(|post| -> PostDTO {
                PostDTO {
                    id: post.id,
                    title: post.title,
                    content: post.content,
                    date: post.date,
                    created_at: post.created_at,
                    updated_at: post.updated_at,
                }
            })
            .collect())
    }

    /// Finds all summarized post written by specific user.
    pub fn get_summarized_list(&mut self, user_id: u64) -> Result<Vec<SummarizedPostDTO>> {
        let post_list = self.post_repository.find_all_in_desc_date_order(user_id)?;

        Ok(post_list
            .into_iter()
            .map(|post| -> SummarizedPostDTO {
                SummarizedPostDTO {
                    id: post.id,
                    title: post.title,
                    date: post.date,
                }
            })
            .collect())
    }

    /// Creates a new post and returns id of the created post.
    pub fn create(
        &mut self,
        user_id: u64,
        title: &str,
        content: &str,
        date: &NaiveDateTime,
    ) -> Result<u64> {
        if title.trim().is_empty() || content.trim().is_empty() {
            return Err(Error::InvalidArgument);
        }

        let post_list = {
            let _ = self.post_repository.create(user_id, title, content, date)?;
            self.post_repository.find_all(user_id)?
        };

        Ok(post_list[post_list.len() - 1].id)
    }

    /// Deletes a post written by specific user.
    pub fn delete(&mut self, id: u64, user_id: u64) -> Result<bool> {
        Ok(self.post_repository.delete(user_id, id)?)
    }

    /// Updates a post written by specific user.
    pub fn update(
        &mut self,
        id: u64,
        user_id: u64,
        title: &Option<String>,
        content: &Option<String>,
        date: &Option<NaiveDateTime>,
    ) -> Result<bool> {
        if title.is_none() && content.is_none() && date.is_none() {
            return Err(Error::InvalidArgument);
        }

        if let Some(content) = content {
            if content.trim().is_empty() {
                return Err(Error::InvalidArgument);
            }
        }

        if let Some(title) = title {
            if title.trim().is_empty() {
                return Err(Error::InvalidArgument);
            }
        }

        Ok(self
            .post_repository
            .update(user_id, id, title, content, date)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<'a> PostService<'a> {
        pub fn new_with_repository(post_repository: PostRepository<'a>) -> Self {
            Self { post_repository }
        }
    }
}
