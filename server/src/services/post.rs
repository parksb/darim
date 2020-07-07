use diesel::result::Error;

use crate::models::error::{get_service_error, ServiceError};
use crate::models::post::*;

pub fn get(user_id: u64, id: u64) -> Result<PostDTO, ServiceError> {
    let post_repository = PostRepository::new();
    let post = post_repository.find(user_id, id);

    match post {
        Ok(post) => Ok(PostDTO {
            id: post.id,
            title: post.title,
            content: post.content,
            date: post.date,
            updated_at: post.updated_at,
            created_at: post.created_at,
        }),
        Err(error) => match error {
            Error::NotFound => Err(get_service_error(ServiceError::NotFound(id.to_string()))),
            _ => Err(get_service_error(ServiceError::QueryExecutionFailure)),
        },
    }
}

pub fn get_list(user_id: u64) -> Result<Vec<PostDTO>, ServiceError> {
    let post_repository = PostRepository::new();
    let post_list = post_repository.find_all(user_id);

    if let Ok(post_list) = post_list {
        let post_to_show_list = post_list
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
            .collect();

        Ok(post_to_show_list)
    } else {
        Err(get_service_error(ServiceError::QueryExecutionFailure))
    }
}

pub fn create(user_id: u64, args: CreateArgs) -> Result<u64, ServiceError> {
    if args.title.trim().is_empty() || args.content.trim().is_empty() {
        return Err(get_service_error(ServiceError::InvalidArgument));
    }

    let post_repository = PostRepository::new();
    let created_count = post_repository.create(user_id, &args.title, &args.content, &args.date);

    if let Ok(created_count) = created_count {
        if created_count > 0 {
            if let Ok(post_list) = post_repository.find_all(user_id) {
                let created_post = &post_list[post_list.len() - 1];
                Ok(created_post.id)
            } else {
                Err(get_service_error(ServiceError::QueryExecutionFailure))
            }
        } else {
            Err(get_service_error(ServiceError::QueryExecutionFailure))
        }
    } else {
        Err(get_service_error(ServiceError::QueryExecutionFailure))
    }
}

pub fn delete(id: u64, user_id: u64) -> Result<bool, ServiceError> {
    let post_repository = PostRepository::new();
    let deleted_count = post_repository.delete(user_id, id);

    if let Ok(deleted_count) = deleted_count {
        if deleted_count > 0 {
            Ok(true)
        } else {
            Err(get_service_error(ServiceError::NotFound(id.to_string())))
        }
    } else {
        Err(get_service_error(ServiceError::QueryExecutionFailure))
    }
}

pub fn update(id: u64, user_id: u64, args: UpdateArgs) -> Result<bool, ServiceError> {
    if args.title.is_none() && args.content.is_none() && args.date.is_none() {
        return Err(get_service_error(ServiceError::InvalidArgument));
    }

    if let Some(content) = &args.content {
        if content.trim().is_empty() {
            return Err(get_service_error(ServiceError::InvalidArgument));
        }
    }

    if let Some(title) = &args.title {
        if title.trim().is_empty() {
            return Err(get_service_error(ServiceError::InvalidArgument));
        }
    }

    let post_repository = PostRepository::new();
    let updated_count = post_repository.update(user_id, id, &args.title, &args.content, &args.date);

    if let Ok(updated_count) = updated_count {
        if updated_count > 0 {
            Ok(true)
        } else {
            Err(get_service_error(ServiceError::NotFound(id.to_string())))
        }
    } else {
        Err(get_service_error(ServiceError::QueryExecutionFailure))
    }
}
