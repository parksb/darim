use actix_web::HttpResponse;
use serde::Serialize;

use crate::models::error::{Error, Result};

/// HTTP response of the API.
#[derive(Serialize)]
pub struct Response<T> {
    data: Option<T>,
    error: Option<String>,
}

impl<T> Response<T> {
    /// Creates a response containing normal data.
    fn ok(data: T) -> Self {
        Response {
            data: Some(data),
            error: None,
        }
    }

    /// Creates a response containing error.
    fn err(error: Error) -> Self {
        Response {
            data: None,
            error: Some(error.message()),
        }
    }
}

/// Converts service result to HTTP response, and return it.
///
/// # Arguments
///
/// * `data` - A result of the service.
pub fn response<T: Serialize>(data: Result<T>) -> HttpResponse {
    match data {
        Ok(data) => HttpResponse::Ok().json(Response::<T>::ok(data)),
        Err(Error::NotFound) => HttpResponse::NotFound().json(Response::<T>::err(Error::NotFound)),
        Err(Error::InvalidArgument) => {
            HttpResponse::BadRequest().json(Response::<T>::err(Error::InvalidArgument))
        }
        Err(Error::InvalidFormat) => {
            HttpResponse::BadRequest().json(Response::<T>::err(Error::InvalidFormat))
        }
        Err(Error::DuplicatedKey) => {
            HttpResponse::Conflict().json(Response::<T>::err(Error::DuplicatedKey))
        }
        Err(Error::Unauthorized) => {
            HttpResponse::Unauthorized().json(Response::<T>::err(Error::Unauthorized))
        }
        Err(Error::Diesel { source }) => match source {
            diesel::NotFound => HttpResponse::NotFound().json(Response::<T>::err(Error::NotFound)),
            _ => HttpResponse::InternalServerError()
                .json(Response::<T>::err(Error::InternalServerError)),
        },
        _ => {
            HttpResponse::InternalServerError().json(Response::<T>::err(Error::InternalServerError))
        }
    }
}
