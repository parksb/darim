use actix_web::HttpResponse;
use serde::Serialize;

use crate::models::error::ServiceError;

/// HTTP response of the API.
#[derive(Serialize)]
pub struct ServiceResponse<T> {
    data: Option<T>,
    error: Option<String>,
}

impl<T> ServiceResponse<T> {
    /// Creates a response containing normal data.
    fn ok(data: T) -> Self {
        ServiceResponse {
            data: Some(data),
            error: None,
        }
    }

    /// Creates a response containing error.
    fn err(error: ServiceError) -> Self {
        ServiceResponse {
            data: None,
            error: Some(format!("{}", error)),
        }
    }
}

/// Converts service result to HTTP response, and return it.
///
/// # Arguments
///
/// * `data` - A result of the service.
pub fn get_response<T: Serialize>(data: Result<T, ServiceError>) -> HttpResponse {
    match data {
        Ok(data) => HttpResponse::Ok().json(ServiceResponse::<T>::ok(data)),
        Err(ServiceError::NotFound(key)) => {
            HttpResponse::NotFound().json(ServiceResponse::<T>::err(ServiceError::NotFound(key)))
        }
        Err(ServiceError::InvalidArgument) => HttpResponse::BadRequest()
            .json(ServiceResponse::<T>::err(ServiceError::InvalidArgument)),
        Err(ServiceError::InvalidFormat) => {
            HttpResponse::BadRequest().json(ServiceResponse::<T>::err(ServiceError::InvalidFormat))
        }
        Err(ServiceError::DuplicatedKey) => {
            HttpResponse::Conflict().json(ServiceResponse::<T>::err(ServiceError::DuplicatedKey))
        }
        Err(ServiceError::Unauthorized) => {
            HttpResponse::Unauthorized().json(ServiceResponse::<T>::err(ServiceError::Unauthorized))
        }
        _ => HttpResponse::InternalServerError()
            .json(ServiceResponse::<T>::err(ServiceError::InternalServerError)),
    }
}
