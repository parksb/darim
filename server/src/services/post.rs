use cfg_if::cfg_if;
use chrono::NaiveDateTime;

use crate::models::error::{get_service_error, ServiceError};
use crate::models::post::*;

cfg_if! {
    if #[cfg(test)] {
        use crate::models::post::MockPostRepositoryTrait as PostRepository;
    } else {
        use crate::models::post::PostRepository;
    }
}

pub struct PostService {
    post_repository: Option<PostRepository>,
}

impl PostService {
    pub fn new() -> Self {
        Self {
            post_repository: None,
        }
    }

    cfg_if! {
        if #[cfg(test)] {
            pub fn new_with_repository(post_repository: PostRepository) -> Self {
                Self { post_repository: Some(post_repository) }
            }
        }
    }

    fn post_repository(&mut self, post_repository: Option<PostRepository>) -> &PostRepository {
        if self.post_repository.is_some() {
            self.post_repository.as_ref().unwrap()
        } else {
            self.post_repository = post_repository;
            self.post_repository.as_ref().unwrap()
        }
    }

    /// Finds a post by user id and post id.
    pub fn get(&mut self, user_id: u64, id: u64) -> Result<PostDTO, ServiceError> {
        let post = {
            let post_repository = Some(PostRepository::new());
            self.post_repository(post_repository).find(user_id, id)?
        };

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
    pub fn get_list(&mut self, user_id: u64) -> Result<Vec<PostDTO>, ServiceError> {
        let post_list = {
            let post_repository = Some(PostRepository::new());
            self.post_repository(post_repository)
                .find_all_in_desc_date_order(user_id)?
        };

        Ok(post_list
            .iter()
            .map(|post| -> PostDTO {
                PostDTO {
                    id: post.id,
                    title: post.title.clone(),
                    content: post.content.clone(),
                    date: post.date,
                    created_at: post.created_at,
                    updated_at: post.updated_at,
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
    ) -> Result<u64, ServiceError> {
        if title.trim().is_empty() || content.trim().is_empty() {
            return Err(get_service_error(ServiceError::InvalidArgument));
        }

        let post_list = {
            let post_repository = Some(PostRepository::new());
            self.post_repository(post_repository)
                .create(user_id, title, content, date)?;
            self.post_repository(None).find_all(user_id)?
        };

        Ok(post_list[post_list.len() - 1].id)
    }

    /// Deletes a post written by specific user.
    pub fn delete(&mut self, id: u64, user_id: u64) -> Result<bool, ServiceError> {
        let post_repository = Some(PostRepository::new());
        self.post_repository(post_repository).delete(user_id, id)
    }

    /// Updates a post written by specific user.
    pub fn update(
        &mut self,
        id: u64,
        user_id: u64,
        title: &Option<String>,
        content: &Option<String>,
        date: &Option<NaiveDateTime>,
    ) -> Result<bool, ServiceError> {
        if title.is_none() && content.is_none() && date.is_none() {
            return Err(get_service_error(ServiceError::InvalidArgument));
        }

        if let Some(content) = content {
            if content.trim().is_empty() {
                return Err(get_service_error(ServiceError::InvalidArgument));
            }
        }

        if let Some(title) = title {
            if title.trim().is_empty() {
                return Err(get_service_error(ServiceError::InvalidArgument));
            }
        }

        let post_repository = Some(PostRepository::new());
        self.post_repository(post_repository)
            .update(user_id, id, title, content, date)
    }
}

impl Default for PostService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mockall::predicate::*;

    use super::*;
    use crate::models::post::MockPostRepositoryTrait;

    #[test]
    fn test_get_list() {
        let mut mocked_post_repository = MockPostRepositoryTrait::new();

        let id = 3;
        let user_id = 5;

        mocked_post_repository
            .expect_find_all_in_desc_date_order()
            .with(eq(user_id))
            .times(1)
            .returning(move |passed_user_id| {
                let now = Utc::now().naive_utc();
                let post = Post {
                    id,
                    user_id: passed_user_id,
                    title: String::from("Title"),
                    content: String::from("Content"),
                    date: now.clone(),
                    created_at: now.clone(),
                    updated_at: None,
                };

                Ok(vec![post])
            });

        let mut post_service = PostService::new_with_repository(mocked_post_repository);
        let post_list: Vec<PostDTO> = post_service.get_list(user_id).unwrap();

        assert_eq!(post_list.first().unwrap().id, id);
    }
}
